//! End-to-end smoke test driving the real `yy` binary (Task 7 manual smoke
//! test, automated): start -> status -> today -> stop -> status, each against
//! an isolated `$HOME` so it never touches the real `~/.yy_logs`.

use assert_fs::TempDir;
use assertables::{assert_contains, assert_starts_with};
use duct::cmd;

fn yy(home: &TempDir, args: &[&str]) -> String {
    cmd(env!("CARGO_BIN_EXE_yy"), args)
        .env("HOME", home.path())
        .read()
        .unwrap()
}

#[test]
fn start_status_today_stop_end_to_end() {
    let home = TempDir::new().unwrap();

    let started = yy(&home, &["start", "manual smoke test", "--issue", "YY-6"]);
    assert_eq!(started, "started YY-6");

    let status = yy(&home, &["status"]);
    assert_contains!(status, "manual smoke test");
    assert_contains!(status, "active: [YY-6] manual smoke test");

    let today = yy(&home, &[]);
    assert_contains!(today, "YY-6");
    assert_contains!(today, "manual smoke test");

    let stopped = yy(&home, &["stop"]);
    assert_eq!(stopped, "stopped YY-6");

    let status_after = yy(&home, &["status"]);
    assert_contains!(status_after, "no active entry");

    let issues = std::fs::read_to_string(home.path().join(".yy_logs/issues.jsonl")).unwrap();
    assert_contains!(issues, "\"key\":\"YY-6\"");

    let month = chrono::Utc::now().format("%Y-%m").to_string();
    let entries_path = home
        .path()
        .join(".yy_logs/entries")
        .join(format!("{month}.jsonl"));
    let entries = std::fs::read_to_string(entries_path).unwrap();
    assert_contains!(entries, "\"t\":\"entry\"");
    assert_contains!(entries, "\"end\":");
}

#[test]
fn status_with_nothing_running_reports_no_active_entry() {
    let home = TempDir::new().unwrap();
    let status = yy(&home, &["status"]);
    assert_contains!(status, "no active entry");
}

#[test]
fn start_stop_without_issue_prints_raw_id() {
    let home = TempDir::new().unwrap();

    let started = yy(&home, &["start", "no issue task"]);
    assert_starts_with!(started, "started ");
    assert!(!started.contains("no issue task"));

    let stopped = yy(&home, &["stop"]);
    assert_starts_with!(stopped, "stopped ");
}
