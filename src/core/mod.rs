//! Use cases: start, stop, assign, split, merge, report, gaps, audits.
//! Exposes a DTO/query layer. MUST NOT depend on `tui` or `cli`.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};

use crate::model::{Entry, Id};
use crate::store::{active, entries};

/// Start a new entry for `issue_id`. If one is already running, it is closed at `at`
/// (or now) and appended to its monthly file before the new entry becomes active.
pub fn start(
    work_folder: &Path,
    issue_id: Option<Id>,
    at: Option<DateTime<Utc>>,
) -> anyhow::Result<Entry> {
    let mut current = active::read(work_folder)?;
    if let Some(running) = current.entry.take() {
        close_and_archive(work_folder, running, at)?;
    }

    let mut entry = Entry::start_now(issue_id);
    if let Some(at) = at {
        entry.start = at;
    }

    active::write(
        work_folder,
        &active::Active {
            entry: Some(entry.clone()),
        },
    )?;

    Ok(entry)
}

/// Stop the running entry at `at` (or now), archiving it to its monthly file and
/// clearing the active-set. Returns `None` if nothing was running.
pub fn stop(work_folder: &Path, at: Option<DateTime<Utc>>) -> anyhow::Result<Option<Entry>> {
    let mut current = active::read(work_folder)?;
    let Some(running) = current.entry.take() else {
        return Ok(None);
    };

    let closed = close_and_archive(work_folder, running, at)?;
    active::write(work_folder, &active::Active::default())?;

    Ok(Some(closed))
}

fn close_and_archive(
    work_folder: &Path,
    mut entry: Entry,
    at: Option<DateTime<Utc>>,
) -> anyhow::Result<Entry> {
    entry.end = Some(at.unwrap_or_else(Utc::now));
    entries::append(work_folder, &entry)?;
    Ok(entry)
}

/// Per-issue total elapsed time for a set of entries.
#[derive(Debug, Clone, PartialEq)]
pub struct IssueTotal {
    pub issue_id: Option<Id>,
    pub elapsed: TimeDelta,
}

/// Today's combined view: the chronological timeline and per-issue totals.
#[derive(Debug, Clone, PartialEq)]
pub struct TodayView {
    pub entries: Vec<Entry>,
    pub totals: Vec<IssueTotal>,
}

/// Aggregate `date`'s entries: closed entries from the monthly file plus the active
/// entry if it started on `date`, sorted chronologically, with per-issue totals.
pub fn today(work_folder: &Path, date: NaiveDate) -> anyhow::Result<TodayView> {
    let mut day_entries = entries::read_month(work_folder, date)?
        .into_iter()
        .filter(|e| e.start.date_naive() == date)
        .collect::<Vec<_>>();

    if let Some(running) = active::read(work_folder)?.entry
        && running.start.date_naive() == date
    {
        day_entries.push(running);
    }

    day_entries.sort_by_key(|e| e.start);

    let mut totals: BTreeMap<Option<Id>, TimeDelta> = BTreeMap::new();
    for entry in &day_entries {
        *totals.entry(entry.issue_id).or_insert_with(TimeDelta::zero) += active::elapsed(entry);
    }

    Ok(TodayView {
        entries: day_entries,
        totals: totals
            .into_iter()
            .map(|(issue_id, elapsed)| IssueTotal { issue_id, elapsed })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn tmp_dir(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("yy-core-test-{label}-{}", ulid::Ulid::generate()))
    }

    #[test]
    fn start_then_stop_produces_closed_interval() {
        let work_folder = tmp_dir("start-stop");
        let start_at = Utc.with_ymd_and_hms(2026, 3, 7, 9, 0, 0).unwrap();
        let stop_at = Utc.with_ymd_and_hms(2026, 3, 7, 10, 0, 0).unwrap();

        let started = start(&work_folder, None, Some(start_at)).unwrap();
        assert_eq!(
            active::read(&work_folder).unwrap().entry,
            Some(started.clone())
        );

        let stopped = stop(&work_folder, Some(stop_at)).unwrap().unwrap();
        assert_eq!(stopped.start, start_at);
        assert_eq!(stopped.end, Some(stop_at));
        assert_eq!(
            active::read(&work_folder).unwrap(),
            active::Active::default()
        );

        let month_entries = entries::read_month(&work_folder, start_at.date_naive()).unwrap();
        assert_eq!(month_entries, vec![stopped]);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn starting_again_stops_the_previous_entry() {
        let work_folder = tmp_dir("restart");
        let t1 = Utc.with_ymd_and_hms(2026, 3, 7, 9, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 3, 7, 9, 30, 0).unwrap();

        let first = start(&work_folder, None, Some(t1)).unwrap();
        let second = start(&work_folder, None, Some(t2)).unwrap();

        let archived = entries::read_month(&work_folder, t1.date_naive()).unwrap();
        assert_eq!(archived.len(), 1);
        assert_eq!(archived[0].id, first.id);
        assert_eq!(archived[0].end, Some(t2));

        let active_now = active::read(&work_folder).unwrap().entry.unwrap();
        assert_eq!(active_now.id, second.id);
        assert_eq!(active_now.end, None);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn stop_with_nothing_running_returns_none() {
        let work_folder = tmp_dir("stop-empty");
        assert_eq!(stop(&work_folder, None).unwrap(), None);
    }

    #[test]
    fn today_combines_closed_entries_and_running_entry() {
        let work_folder = tmp_dir("today");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let issue = Id::new();

        let closed_start = Utc.with_ymd_and_hms(2026, 3, 7, 9, 0, 0).unwrap();
        let closed_stop = Utc.with_ymd_and_hms(2026, 3, 7, 9, 30, 0).unwrap();
        start(&work_folder, Some(issue), Some(closed_start)).unwrap();
        stop(&work_folder, Some(closed_stop)).unwrap();

        let running_start = Utc.with_ymd_and_hms(2026, 3, 7, 10, 0, 0).unwrap();
        start(&work_folder, Some(issue), Some(running_start)).unwrap();

        let view = today(&work_folder, date).unwrap();
        assert_eq!(view.entries.len(), 2);
        assert_eq!(view.entries[0].start, closed_start);
        assert_eq!(view.entries[1].start, running_start);

        assert_eq!(view.totals.len(), 1);
        assert_eq!(view.totals[0].issue_id, Some(issue));
        assert!(view.totals[0].elapsed >= TimeDelta::minutes(30));

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn today_ignores_entries_from_other_days() {
        let work_folder = tmp_dir("today-filter");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();

        let other_day = Utc.with_ymd_and_hms(2026, 3, 6, 9, 0, 0).unwrap();
        start(&work_folder, None, Some(other_day)).unwrap();
        stop(
            &work_folder,
            Some(Utc.with_ymd_and_hms(2026, 3, 6, 9, 30, 0).unwrap()),
        )
        .unwrap();

        let view = today(&work_folder, date).unwrap();
        assert_eq!(view.entries, Vec::new());
        assert_eq!(view.totals, Vec::new());

        std::fs::remove_dir_all(&work_folder).unwrap();
    }
}
