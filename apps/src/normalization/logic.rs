use regex::Regex;
use ::std::sync::LazyLock;

macro_rules! try_local {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return ::axiom::err!(e),
        }
    };
}

static PII_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Email
        Regex::new(r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}"),
        // Credit Card (basic sequence of 13-16 digits with optional spaces/dashes)
        Regex::new(r"\b(?:\d[ -]*?){13,16}\b"),
        // SSN
        Regex::new(r"\b\d{3}-\d{2}-\d{4}\b"),
    ]
    .into_iter()
    .filter_map(Result::ok)
    .collect()
});

pub fn redact_pii(message: &str) -> (String, u64) {
    let mut redacted = message.to_string();
    let mut total_redactions = 0;

    for pattern in PII_PATTERNS.iter() {
        let count = pattern.find_iter(&redacted).count() as u64;
        if count > 0 {
            redacted = pattern.replace_all(&redacted, "[REDACTED]").to_string();
            total_redactions += count;
        }
    }

    (redacted, total_redactions)
}

pub fn flatten_to_parallel_arrays(
    attribute_keys: Vec<String>,
    attribute_values_string: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    (attribute_keys, attribute_values_string)
}

pub fn is_poison_pill(raw_bytes: &[u8]) -> bool {
    raw_bytes.len() > 65536
}
