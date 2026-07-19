//! Active-set (the currently running entry).
//!
//! MVP: a single active entry, stored at `<work_folder>/active.json` — not JSONL,
//! since it's always exactly one small object, replaced wholesale on every write.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::model::Entry;

/// The currently running entry, if any.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Active {
    pub entry: Option<Entry>,
}

/// Elapsed time for `entry`: `now - start` if still running, else `end - start`.
/// Never stored — always recomputed. See golden rule #1.
pub fn elapsed(entry: &Entry) -> TimeDelta {
    entry.end.unwrap_or_else(Utc::now) - entry.start
}

fn path(work_folder: &Path) -> PathBuf {
    work_folder.join("active.json")
}

/// Read the active-set. A missing file means nothing is running.
pub fn read(work_folder: &Path) -> anyhow::Result<Active> {
    match fs::read_to_string(path(work_folder)) {
        Ok(content) => Ok(serde_json::from_str(&content)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Active::default()),
        Err(e) => Err(e.into()),
    }
}

/// Write the active-set atomically: temp file in the same directory, then rename.
pub fn write(work_folder: &Path, active: &Active) -> anyhow::Result<()> {
    let path = path(work_folder);
    let content = serde_json::to_string_pretty(active)?;

    fs::create_dir_all(work_folder)?;

    let tmp_path = path.with_extension("json.tmp");
    let mut tmp = fs::File::create(&tmp_path)?;
    tmp.write_all(content.as_bytes())?;
    tmp.sync_all()?;
    drop(tmp);

    fs::rename(&tmp_path, &path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn tmp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!("yy-active-test-{label}-{}", ulid::Ulid::generate()))
    }

    #[test]
    fn missing_file_reads_as_no_active_entry() {
        let work_folder = tmp_dir("missing");
        assert_eq!(read(&work_folder).unwrap(), Active::default());
    }

    #[test]
    fn write_then_read_round_trips() {
        let work_folder = tmp_dir("roundtrip");
        let active = Active {
            entry: Some(Entry::start_now(None)),
        };

        write(&work_folder, &active).unwrap();
        assert!(!path(&work_folder).with_extension("json.tmp").exists());

        let read_back = read(&work_folder).unwrap();
        assert_eq!(read_back, active);

        fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn elapsed_of_running_entry_is_derived_from_now() {
        let mut entry = Entry::start_now(None);
        entry.start = Utc::now() - TimeDelta::minutes(5);

        let e = elapsed(&entry);
        assert!(e >= TimeDelta::minutes(5) && e < TimeDelta::minutes(6));
    }

    #[test]
    fn elapsed_of_closed_entry_is_end_minus_start() {
        let mut entry = Entry::start_now(None);
        entry.start = Utc.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap();
        entry.end = Some(Utc.with_ymd_and_hms(2026, 1, 1, 9, 30, 0).unwrap());

        assert_eq!(elapsed(&entry), TimeDelta::minutes(30));
    }
}
