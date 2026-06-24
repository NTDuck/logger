use crate::edge::models::{DomainLog, EdgeError, JwtClaims};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use struson::reader::{JsonReader, JsonStreamReader, ValueType};
use tap::TapFallible;


macro_rules! try_local {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return ::axiom::err!(e),
        }
    };
}

macro_rules! try_fallible {
    ($e:expr) => {
        match $e {
            Ok(Ok(v)) => v,
            Ok(Err(errs)) => return ::axiom::errs!(errs),
            Err(e) => return Err(e.into()),
        }
    };
}


pub fn parse_and_validate_log(bytes: &[u8]) -> ::axiom::result::Fallible<::core::result::Result<DomainLog, ::std::vec::Vec<EdgeError>>> {
    let mut depth = 0;
    let mut in_string = false;
    let mut escape = false;
    for &b in bytes {
        if escape {
            escape = false;
            continue;
        }
        match b {
            b'"' => in_string = !in_string,
            b'\\' if in_string => escape = true,
            b'{' | b'[' if !in_string => {
                depth += 1;
                if depth > 5 {
                    return ::axiom::err!(EdgeError::BadRequest("Nesting depth exceeds 5".to_string()));
                }
            }
            b'}' | b']' if !in_string => {
                depth -= 1;
            }
            _ => {}
        }
    }
    if in_string || depth != 0 {
        return ::axiom::err!(EdgeError::BadRequest(
            "Malformed JSON byte stream".to_string(),
        ));
    }

    let mut reader = JsonStreamReader::new(bytes);
    reader
        .begin_object()
        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;

    let mut timestamp = None;
    let mut level = None;
    let mut message = None;
    let mut app_name = None;
    let mut error_code = None;
    let mut attribute_keys = Vec::new();
    let mut attribute_values_string = Vec::new();

    let mut root_props = 0;
    while reader
        .has_next()
        .map_err(|e| EdgeError::BadRequest(e.to_string()))?
    {
        root_props += 1;
        if root_props > 50 {
            return ::axiom::err!(EdgeError::BadRequest("Too many properties".to_string()));
        }
        let name = reader
            .next_name()
            .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
        if name.len() > 255 {
            return ::axiom::err!(EdgeError::BadRequest("Key too long".to_string()));
        }

        match name {
            "timestamp" => {
                timestamp = Some(
                    reader
                        .next_string()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?,
                );
            }
            "level" => {
                let lvl = reader
                    .next_string()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                if !matches!(
                    lvl.as_str(),
                    "DEBUG" | "INFO" | "WARN" | "ERROR" | "CRITICAL"
                ) {
                    return ::axiom::err!(EdgeError::BadRequest("Invalid level".to_string()));
                }
                level = Some(lvl);
            }
            "message" => {
                let msg = reader
                    .next_string()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                if msg.len() > 32768 {
                    return ::axiom::err!(EdgeError::BadRequest("Message too long".to_string()));
                }
                message = Some(msg);
            }
            "app_name" => {
                let app = reader
                    .next_string()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                if app.len() > 255 {
                    return ::axiom::err!(EdgeError::BadRequest("App name too long".to_string()));
                }
                app_name = Some(app);
            }
            "error_code" => {
                let vt = reader
                    .peek()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                if let ValueType::Null = vt {
                    reader
                        .next_null()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                } else {
                    let code = reader
                        .next_string()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                    if code.len() > 255 {
                        return ::axiom::err!(EdgeError::BadRequest("Error code too long".to_string()));
                    }
                    error_code = Some(code);
                }
            }
            "attributes" => {
                try_fallible!(flatten_attributes(
                    &mut reader,
                    &mut attribute_keys,
                    &mut attribute_values_string,
                ));
            }
            _ => {
                reader
                    .skip_value()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
            }
        }
    }
    reader
        .end_object()
        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;

    ::axiom::ok!(DomainLog::builder()
        .timestamp(timestamp.ok_or_else(|| EdgeError::BadRequest("Missing timestamp".into()))?)
        .level(level.ok_or_else(|| EdgeError::BadRequest("Missing level".into()))?)
        .message(message.ok_or_else(|| EdgeError::BadRequest("Missing message".into()))?)
        .app_name(app_name.ok_or_else(|| EdgeError::BadRequest("Missing app_name".into()))?)
        .maybe_error_code(error_code)
        .attribute_keys(attribute_keys)
        .attribute_values_string(attribute_values_string)
        .build())
}

enum Container {
    Object {
        prefix: String,
        count: usize,
    },
    Array {
        prefix: String,
        count: usize,
        index: usize,
    },
}

fn flatten_attributes(
    reader: &mut JsonStreamReader<&[u8]>,
    keys: &mut Vec<String>,
    values: &mut Vec<String>,
) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<EdgeError>>> {
    let mut stack = Vec::new();

    let vt = reader
        .peek()
        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
    match vt {
        ValueType::Object => {
            reader
                .begin_object()
                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
            stack.push(Container::Object {
                prefix: "".to_string(),
                count: 0,
            });
        }
        ValueType::Array => {
            reader
                .begin_array()
                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
            stack.push(Container::Array {
                prefix: "".to_string(),
                count: 0,
                index: 0,
            });
        }
        ValueType::Null => {
            reader
                .next_null()
                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
            return ::axiom::ok!(());
        }
        _ => {
            let val = try_fallible!(next_scalar_as_string(reader));
            keys.push("attributes".to_string());
            values.push(val);
            return ::axiom::ok!(());
        }
    }

    while let Some(mut current) = stack.pop() {
        match &mut current {
            Container::Object { prefix, count } => {
                if reader
                    .has_next()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?
                {
                    *count += 1;
                    if *count > 50 {
                        return ::axiom::err!(EdgeError::BadRequest("Too many properties".to_string()));
                    }
                    let key = reader
                        .next_name()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                    if key.len() > 255 {
                        return ::axiom::err!(EdgeError::BadRequest("Key too long".to_string()));
                    }
                    let full_key = if prefix.is_empty() {
                        key.to_string()
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    let vt = reader
                        .peek()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                    match vt {
                        ValueType::Object => {
                            reader
                                .begin_object()
                                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                            stack.push(current);
                            stack.push(Container::Object {
                                prefix: full_key,
                                count: 0,
                            });
                        }
                        ValueType::Array => {
                            reader
                                .begin_array()
                                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                            stack.push(current);
                            stack.push(Container::Array {
                                prefix: full_key,
                                count: 0,
                                index: 0,
                            });
                        }
                        _ => {
                            let val = try_fallible!(next_scalar_as_string(reader));
                            keys.push(full_key);
                            values.push(val);
                            stack.push(current);
                        }
                    }
                } else {
                    reader
                        .end_object()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                }
            }
            Container::Array {
                prefix,
                count,
                index,
            } => {
                if reader
                    .has_next()
                    .map_err(|e| EdgeError::BadRequest(e.to_string()))?
                {
                    *count += 1;
                    if *count > 250 {
                        return ::axiom::err!(EdgeError::BadRequest("Array too large".to_string()));
                    }
                    let full_key = format!("{}[{}]", prefix, index);
                    *index += 1;

                    let vt = reader
                        .peek()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                    match vt {
                        ValueType::Object => {
                            reader
                                .begin_object()
                                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                            stack.push(current);
                            stack.push(Container::Object {
                                prefix: full_key,
                                count: 0,
                            });
                        }
                        ValueType::Array => {
                            reader
                                .begin_array()
                                .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                            stack.push(current);
                            stack.push(Container::Array {
                                prefix: full_key,
                                count: 0,
                                index: 0,
                            });
                        }
                        _ => {
                            let val = try_fallible!(next_scalar_as_string(reader));
                            keys.push(full_key);
                            values.push(val);
                            stack.push(current);
                        }
                    }
                } else {
                    reader
                        .end_array()
                        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
                }
            }
        }
    }
    ::axiom::ok!(())
}

fn next_scalar_as_string(reader: &mut JsonStreamReader<&[u8]>) -> ::axiom::result::Fallible<::core::result::Result<String, ::std::vec::Vec<EdgeError>>> {
    let vt = reader
        .peek()
        .map_err(|e| EdgeError::BadRequest(e.to_string()))?;
    match vt {
        ValueType::String => ::axiom::ok!(try_local!(reader
            .next_string()
            .map_err(|e| EdgeError::BadRequest(e.to_string())))),
        ValueType::Number => ::axiom::ok!(try_local!(reader
            .next_number_as_string()
            .map_err(|e| EdgeError::BadRequest(e.to_string())))),
        ValueType::Boolean => ::axiom::ok!(try_local!(reader
            .next_bool()
            .map_err(|e| EdgeError::BadRequest(e.to_string())))
            .to_string()),
        ValueType::Null => {
            try_local!(reader
                .next_null()
                .map_err(|e| EdgeError::BadRequest(e.to_string())));
            ::axiom::ok!("null".to_string())
        }
        _ => return ::axiom::err!(EdgeError::BadRequest("Expected scalar value".to_string())),
    }
}

pub fn validate_jwt(token: &str, public_key_pem: &[u8]) -> ::axiom::result::Fallible<::core::result::Result<JwtClaims, ::std::vec::Vec<EdgeError>>> {
    let key = DecodingKey::from_rsa_pem(public_key_pem)
        .tap_err(|e| ::tracing::error!(error = %e, "Invalid JWT public key PEM"))
        .map_err(|_| EdgeError::Unauthorized("Invalid key".to_string()))?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.required_spec_claims.insert("exp".to_string());

    let token_data = decode::<JwtClaims>(token, &key, &validation)
        .tap_err(|e| ::tracing::error!(error = %e, "JWT decode failed"))
        .map_err(|e| EdgeError::Unauthorized(e.to_string()))?;

    ::axiom::ok!(token_data.claims)
}

pub fn check_app_grant(claims: &JwtClaims, app_name: &str) -> ::axiom::result::Fallible<::core::result::Result<(), ::std::vec::Vec<EdgeError>>> {
    if claims.app_grants.iter().any(|g| g == "*" || g == app_name) {
        ::axiom::ok!(())
    } else {
        return ::axiom::err!(EdgeError::Forbidden);
    }
}
