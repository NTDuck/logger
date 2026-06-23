use sha2::{Digest, Sha256};

pub fn generate_fingerprint(app_name: &str, message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(app_name.as_bytes());
    hasher.update(b":");
    hasher.update(message.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn format_notification(app_name: &str, message: &str, timestamp: &str) -> String {
    format!(
        "🚨 **ALERT** 🚨\n\n**App:** {}\n**Time:** {}\n**Message:**\n{}",
        app_name, timestamp, message
    )
}

pub fn format_digest(
    app_name: &str,
    fingerprint: &str,
    count: u64,
    time_window_sec: u64,
) -> String {
    format!(
        "🔥 **DIGEST ALERT** 🔥\n\n**App:** {}\n**Fingerprint:** {}\n**Occurrences:** {} within {}s",
        app_name, fingerprint, count, time_window_sec
    )
}
