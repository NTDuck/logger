use crate::ws::models::{WSError, WsClientConfig};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use ::serde::Deserialize;
use tap::TapFallible;

#[derive(::core::fmt::Debug, ::serde::Deserialize)]
struct Claims {
    app_grants: Option<Vec<String>>,
}

pub fn parse_ws_claims(token: &str, decoding_key: &DecodingKey) -> ::axiom::result::Fallible<::core::result::Result<WsClientConfig, ::std::vec::Vec<WSError>>> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    let token_data = match decode::<Claims>(token, decoding_key, &validation)
        .tap_err(|e| ::tracing::error!(error = %e, "JWT verification failed")) {
            Ok(d) => d,
            Err(_) => return ::axiom::err!(WSError::InvalidToken),
        };

    let app_grants = token_data.claims.app_grants.unwrap_or_default();

    if app_grants.is_empty() {
        ::tracing::warn!("JWT valid but app_grants is empty");
        return ::axiom::err!(WSError::Forbidden);
    }

    if app_grants.len() == 1 && app_grants[0] == "*" {
        return ::axiom::ok!(WsClientConfig::builder()
            .allowed_apps(vec![])
            .is_admin(true)
            .build());
    }

    ::axiom::ok!(WsClientConfig::builder()
        .allowed_apps(app_grants)
        .is_admin(false)
        .build())
}
