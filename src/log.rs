use tracing::{error, info, warn};

pub fn display_header() {
    self::info("--- Price Scraper ---".to_string());
    self::info("Version: 1.0".to_string());
    self::info("Author: Jon Fast".to_string());
}

pub fn info_static(message: &'static str) {
    info!(message);
}

pub fn warn_static(message: &'static str) {
    warn!(message);
}

pub fn error_static(message: &'static str) {
    error!(message);
}

pub fn info(s: String) {
    let message = s.as_str();
    info!(message);
}

pub fn warn(s: String) {
    let message = s.as_str();
    warn!(message);
}

pub fn error(s: String) {
    let message = s.as_str();
    error!(message);
}
