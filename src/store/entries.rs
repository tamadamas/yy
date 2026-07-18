//! Monthly file resolution, date-range queries.

use std::path::{Path, PathBuf};

use chrono::{Datelike, NaiveDate};

use crate::model::Entry;
use crate::store::jsonl::{self, Line, Record};

/// Path to the monthly entries file for `date`: `<work_folder>/entries/YYYY-MM.jsonl`.
pub fn month_path(work_folder: &Path, date: NaiveDate) -> PathBuf {
    work_folder
        .join("entries")
        .join(format!("{:04}-{:02}.jsonl", date.year(), date.month()))
}

/// Read all [`Entry`] records (ignoring comments, malformed lines, and other record
/// types) from the monthly file covering `date`. A missing file yields no entries.
pub fn read_month(work_folder: &Path, date: NaiveDate) -> anyhow::Result<Vec<Entry>> {
    let lines = jsonl::read(&month_path(work_folder, date))?;
    Ok(lines
        .into_iter()
        .filter_map(|line| match line {
            Line::Record(Record::Entry(entry)) => Some(entry),
            _ => None,
        })
        .collect())
}

/// Read all entries whose `start` falls within `[from, to]` inclusive, spanning
/// however many monthly files that covers.
pub fn entries_in_range(
    work_folder: &Path,
    from: NaiveDate,
    to: NaiveDate,
) -> anyhow::Result<Vec<Entry>> {
    let mut out = Vec::new();
    let mut month = NaiveDate::from_ymd_opt(from.year(), from.month(), 1).unwrap();
    let stop = NaiveDate::from_ymd_opt(to.year(), to.month(), 1).unwrap();

    loop {
        out.extend(read_month(work_folder, month)?.into_iter().filter(|entry| {
            let day = entry.start.date_naive();
            day >= from && day <= to
        }));

        if month >= stop {
            break;
        }
        month = if month.month() == 12 {
            NaiveDate::from_ymd_opt(month.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(month.year(), month.month() + 1, 1).unwrap()
        };
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Issue, IssueKind};
    use chrono::{TimeZone, Utc};

    fn tmp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "yy-entries-test-{label}-{}",
            ulid::Ulid::generate()
        ))
    }

    fn entry_at(y: i32, m: u32, d: u32) -> Entry {
        let mut entry = Entry::start_now(None);
        entry.start = Utc.with_ymd_and_hms(y, m, d, 9, 0, 0).unwrap();
        entry.end = Some(Utc.with_ymd_and_hms(y, m, d, 10, 0, 0).unwrap());
        entry
    }

    #[test]
    fn month_path_formats_year_month() {
        let work_folder = Path::new("/tmp/yy_logs");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        assert_eq!(
            month_path(work_folder, date),
            Path::new("/tmp/yy_logs/entries/2026-03.jsonl")
        );
    }

    #[test]
    fn missing_month_file_reads_no_entries() {
        let work_folder = tmp_dir("missing");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        assert_eq!(read_month(&work_folder, date).unwrap(), Vec::new());
    }

    #[test]
    fn read_month_ignores_non_entry_records() {
        let work_folder = tmp_dir("filter");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let path = month_path(&work_folder, date);

        let entry = entry_at(2026, 3, 7);
        let lines = vec![
            Line::Comment("# march".to_string()),
            Line::Record(Record::Issue(Issue::new("x", IssueKind::Task))),
            Line::Record(Record::Entry(entry.clone())),
        ];
        jsonl::write(&path, &lines).unwrap();

        let read_back = read_month(&work_folder, date).unwrap();
        assert_eq!(read_back, vec![entry]);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn entries_in_range_spans_multiple_months() {
        let work_folder = tmp_dir("range");

        let feb_entry = entry_at(2026, 2, 28);
        let mar_entry = entry_at(2026, 3, 1);
        let out_of_range = entry_at(2026, 3, 15);

        jsonl::write(
            &month_path(&work_folder, NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()),
            &[Line::Record(Record::Entry(feb_entry.clone()))],
        )
        .unwrap();
        jsonl::write(
            &month_path(&work_folder, NaiveDate::from_ymd_opt(2026, 3, 1).unwrap()),
            &[
                Line::Record(Record::Entry(mar_entry.clone())),
                Line::Record(Record::Entry(out_of_range)),
            ],
        )
        .unwrap();

        let from = NaiveDate::from_ymd_opt(2026, 2, 28).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let mut result = entries_in_range(&work_folder, from, to).unwrap();
        result.sort_by_key(|e| e.start);

        assert_eq!(result, vec![feb_entry, mar_entry]);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }
}
