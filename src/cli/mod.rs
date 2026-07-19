//! clap parsing; thin wrapper around `core`; shared entry-spec flags.

use std::path::Path;

use chrono::{NaiveDate, TimeDelta, Utc};
use clap::{Args, Parser, Subcommand};

use crate::core::{self, IssueTotal, TodayView};
use crate::model::{Entry, Id};
use crate::store::{active, issues};

mod time;

/// Validates an issue key: two or more uppercase letters, a dash, then one
/// or more digits (e.g. `YY-1`, `DFG-1234`, `KJJ-2`).
pub fn parse_issue_key(s: &str) -> Result<String, String> {
    let Some((prefix, suffix)) = s.split_once('-') else {
        return Err(format!(
            "issue key must be LETTERS-NUMBER, e.g. YY-1 (got \"{s}\")"
        ));
    };

    let prefix_ok = prefix.len() >= 2 && prefix.chars().all(|c| c.is_ascii_uppercase());
    let suffix_ok = !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit());

    if prefix_ok && suffix_ok {
        Ok(s.to_string())
    } else {
        Err(format!(
            "issue key must be LETTERS-NUMBER, e.g. YY-1 (got \"{s}\")"
        ))
    }
}

#[derive(Parser)]
#[command(name = "yy")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show yesterday's (last working day's) data instead of today's.
    #[arg(long, global = true)]
    pub yesterday: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Begin an entry; auto-stops the currently running one.
    #[command(alias = "st")]
    Start {
        desc: Option<String>,
        #[command(flatten)]
        spec: EntrySpec,
    },
    /// Close the running entry.
    #[command(alias = "x")]
    Stop {
        #[arg(long)]
        at: Option<String>,
    },
    /// Current task, worked-today total, remaining vs the 8h target.
    #[command(alias = "s")]
    Status,
}

#[derive(Args)]
pub struct EntrySpec {
    #[arg(long, value_parser = parse_issue_key)]
    pub issue: Option<String>,
    #[arg(long = "tag")]
    pub tags: Vec<String>,
    #[arg(long)]
    pub from: Option<String>,
    #[arg(long)]
    pub to: Option<String>,
}

/// `today` if `yesterday` is false, else the last working day before `today`.
pub fn resolve_target_date(today: NaiveDate, yesterday: bool) -> NaiveDate {
    if yesterday {
        core::last_working_day(today)
    } else {
        today
    }
}

fn format_duration(d: TimeDelta) -> String {
    let total_minutes = d.num_minutes();
    format!("{}h{:02}m", total_minutes / 60, (total_minutes % 60).abs())
}

/// Render the today/yesterday combined view. `issue_keys` maps each entry's
/// `issue_id` to a display label (its key, or the raw id if unresolved) —
/// built by `run()` via `store::issues::find_by_id` before calling this.
pub fn render_today(view: &TodayView, issue_labels: &[(Option<Id>, String)]) -> String {
    let label_for = |issue_id: Option<Id>| -> String {
        issue_labels
            .iter()
            .find(|(id, _)| *id == issue_id)
            .map(|(_, label)| label.clone())
            .unwrap_or_else(|| "(no issue)".to_string())
    };

    if view.entries.is_empty() {
        return "no entries today\n".to_string();
    }

    let mut out = String::new();
    for entry in &view.entries {
        let desc = entry.note.as_deref().unwrap_or("(no desc)");
        let elapsed = active::elapsed(entry);
        out.push_str(&format!(
            "{}  {}  {}  {}\n",
            entry.start.format("%H:%M"),
            label_for(entry.issue_id),
            desc,
            format_duration(elapsed)
        ));
    }

    out.push_str("\ntotals:\n");
    for IssueTotal { issue_id, elapsed } in &view.totals {
        out.push_str(&format!(
            "  {}  {}\n",
            label_for(*issue_id),
            format_duration(*elapsed)
        ));
    }

    out
}

const DAILY_TARGET: TimeDelta = TimeDelta::hours(8);

/// Render `yy status`: current task, worked so far, remaining vs an 8h target.
pub fn render_status(active_entry: &Option<Entry>, worked_today: TimeDelta) -> String {
    let current = match active_entry {
        Some(entry) => entry.note.as_deref().unwrap_or("(no desc)").to_string(),
        None => "no active entry".to_string(),
    };

    let remaining = DAILY_TARGET - worked_today;
    format!(
        "current: {current}\nworked: {}\nremaining: {}\n",
        format_duration(worked_today),
        format_duration(remaining)
    )
}

/// Run the CLI against `work_folder`, returning the text to print.
pub fn run(work_folder: &Path, cli: Cli) -> anyhow::Result<String> {
    let today = Utc::now().date_naive();
    let target_date = resolve_target_date(today, cli.yesterday);

    match cli.command {
        None => {
            let view = core::today(work_folder, target_date)?;
            let labels = issue_labels(work_folder, &view)?;
            Ok(render_today(&view, &labels))
        }
        Some(Commands::Start { desc, spec }) => {
            let issue_id = match &spec.issue {
                Some(key) => Some(issues::resolve_or_create(
                    work_folder,
                    key,
                    desc.as_deref(),
                )?),
                None => None,
            };

            let from = spec
                .from
                .as_deref()
                .map(|s| time::parse_time(s, target_date))
                .transpose()?;
            let to = spec
                .to
                .as_deref()
                .map(|s| time::parse_time(s, target_date))
                .transpose()?;

            let entry = core::start(work_folder, issue_id, desc, spec.tags, from)?;

            if let Some(to) = to {
                core::stop(work_folder, Some(to))?;
            }

            Ok(format!("started {}\n", entry.id))
        }
        Some(Commands::Stop { at }) => {
            let at = at
                .as_deref()
                .map(|s| time::parse_time(s, target_date))
                .transpose()?;
            match core::stop(work_folder, at)? {
                Some(entry) => Ok(format!("stopped {}\n", entry.id)),
                None => Ok("nothing running\n".to_string()),
            }
        }
        Some(Commands::Status) => {
            let view = core::today(work_folder, target_date)?;
            let worked_today = view
                .totals
                .iter()
                .fold(TimeDelta::zero(), |acc, t| acc + t.elapsed);
            let active_entry = active::read(work_folder)?.entry;
            Ok(render_status(&active_entry, worked_today))
        }
    }
}

fn issue_labels(work_folder: &Path, view: &TodayView) -> anyhow::Result<Vec<(Option<Id>, String)>> {
    let mut labels = Vec::new();
    for entry in &view.entries {
        if let Some(issue_id) = entry.issue_id
            && !labels.iter().any(|(id, _)| *id == Some(issue_id))
        {
            let label = issues::find_by_id(work_folder, issue_id)?
                .and_then(|i| i.key)
                .unwrap_or_else(|| issue_id.to_string());
            labels.push((Some(issue_id), label));
        }
    }
    Ok(labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta as TD;

    fn sample_entry(issue_id: Option<Id>, note: Option<&str>) -> Entry {
        let mut e = Entry::start_now(issue_id);
        e.note = note.map(str::to_string);
        e
    }

    #[test]
    fn resolve_target_date_without_yesterday_is_today() {
        let today = NaiveDate::from_ymd_opt(2026, 3, 10).unwrap();
        assert_eq!(resolve_target_date(today, false), today);
    }

    #[test]
    fn resolve_target_date_with_yesterday_uses_last_working_day() {
        let monday = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        assert_eq!(
            resolve_target_date(monday, true),
            NaiveDate::from_ymd_opt(2026, 3, 6).unwrap()
        );
    }

    #[test]
    fn render_today_shows_entries_and_totals() {
        let issue = Id::new();
        let view = TodayView {
            entries: vec![sample_entry(Some(issue), Some("wrote tests"))],
            totals: vec![IssueTotal {
                issue_id: Some(issue),
                elapsed: TD::minutes(45),
            }],
        };
        let out = render_today(&view, &[]);
        assert!(out.contains("wrote tests"));
        assert!(out.contains("45m") || out.contains("0h45m"));
    }

    #[test]
    fn render_today_handles_empty_day() {
        let view = TodayView {
            entries: Vec::new(),
            totals: Vec::new(),
        };
        let out = render_today(&view, &[]);
        assert!(out.contains("no entries") || out.contains("nothing"));
    }

    #[test]
    fn render_status_shows_active_note_and_remaining() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, TD::hours(2));
        assert!(out.contains("deep work"));
        assert!(out.contains("2h") || out.contains("02:00"));
        assert!(out.contains("6h") || out.contains("06:00")); // 8h target - 2h worked
    }

    #[test]
    fn render_status_handles_nothing_running() {
        let out = render_status(&None, TD::zero());
        assert!(out.contains("no active entry") || out.contains("nothing running"));
    }

    #[test]
    fn accepts_valid_keys() {
        assert_eq!(parse_issue_key("YY-1"), Ok("YY-1".to_string()));
        assert_eq!(parse_issue_key("DFG-1234"), Ok("DFG-1234".to_string()));
        assert_eq!(parse_issue_key("KJJ-2"), Ok("KJJ-2".to_string()));
    }

    #[test]
    fn rejects_single_letter_prefix() {
        assert!(parse_issue_key("Y-1").is_err());
    }

    #[test]
    fn rejects_lowercase() {
        assert!(parse_issue_key("yy-1").is_err());
    }

    #[test]
    fn rejects_missing_number() {
        assert!(parse_issue_key("YY-").is_err());
        assert!(parse_issue_key("YY").is_err());
    }

    #[test]
    fn rejects_non_numeric_suffix() {
        assert!(parse_issue_key("YY-1a").is_err());
    }

    #[test]
    fn error_message_includes_hint() {
        let err = parse_issue_key("bad").unwrap_err();
        assert!(
            err.contains("YY-1"),
            "error should hint at valid format: {err}"
        );
    }
}
