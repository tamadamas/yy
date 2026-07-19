pub mod cli;
pub mod core;
pub mod export;
pub mod import;
pub mod model;
pub mod pipeline;
pub mod store;
pub mod tui;
pub mod watch;

/// Hardcoded for Sprint 0 — no --work-folder flag or $YY_WORK_FOLDER yet
/// (config.toml lands in Sprint 1).
pub fn work_folder() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("HOME").expect("HOME not set")).join(".yy_logs")
}
