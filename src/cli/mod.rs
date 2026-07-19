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
    let sign = if total_minutes < 0 { "-" } else { "" };
    let hours = total_minutes.abs() / 60;
    let minutes = total_minutes.abs() % 60;
    format!("{sign}{hours:02}:{minutes:02}")
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

/// Render `yy status`: active task (with issue label, already bracketed by
/// the caller), worked so far, remaining vs an 8h target (parenthesized
/// overtime diff on `worked` when negative).
pub fn render_status(
    active_entry: &Option<Entry>,
    current_label: Option<&str>,
    worked_today: TimeDelta,
) -> String {
    let active = match active_entry {
        Some(entry) => {
            let desc = entry.note.as_deref().unwrap_or("(no desc)");
            match current_label {
                Some(label) => format!("{label} {desc}"),
                None => desc.to_string(),
            }
        }
        None => "no active entry".to_string(),
    };

    let remaining = DAILY_TARGET - worked_today;
    let worked = if remaining < TimeDelta::zero() {
        format!(
            "{}({})",
            format_duration(worked_today),
            format_duration(remaining)
        )
    } else {
        format_duration(worked_today)
    };

    format!(
        "active: {active}\nworked: {worked}\nremaining: {}\n",
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

            let label = match issue_id {
                Some(id) => issues::find_by_id(work_folder, id)?
                    .and_then(|i| i.key)
                    .unwrap_or_else(|| entry.id.to_string()),
                None => entry.id.to_string(),
            };
            Ok(format!("started {label}\n"))
        }
        Some(Commands::Stop { at }) => {
            let at = at
                .as_deref()
                .map(|s| time::parse_time(s, target_date))
                .transpose()?;
            match core::stop(work_folder, at)? {
                Some(entry) => {
                    let label = match entry.issue_id {
                        Some(id) => issues::find_by_id(work_folder, id)?
                            .and_then(|i| i.key)
                            .unwrap_or_else(|| entry.id.to_string()),
                        None => entry.id.to_string(),
                    };
                    Ok(format!("stopped {label}\n"))
                }
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
            Ok(render_status(&active_entry, None, worked_today))
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
    fn format_duration_zero_pads_sub_hour() {
        assert_eq!(format_duration(TD::minutes(6)), "00:06");
    }

    #[test]
    fn format_duration_zero_pads_multi_hour() {
        assert_eq!(format_duration(TD::hours(8) + TD::minutes(15)), "08:15");
    }

    #[test]
    fn format_duration_handles_zero() {
        assert_eq!(format_duration(TD::zero()), "00:00");
    }

    #[test]
    fn format_duration_sign_prefixes_negative() {
        assert_eq!(format_duration(-TD::minutes(15)), "-00:15");
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
        assert!(out.contains("00:45"));
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
    fn render_status_shows_issue_label_and_remaining() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("[YY-6]"), TD::hours(2));
        assert!(out.contains("active: [YY-6] deep work"));
        assert!(out.contains("worked: 02:00"));
        assert!(out.contains("remaining: 06:00"));
    }

    #[test]
    fn render_status_shows_no_issue_placeholder() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("(no issue)"), TD::hours(2));
        assert!(out.contains("active: (no issue) deep work"));
    }

    #[test]
    fn render_status_handles_nothing_running() {
        let out = render_status(&None, None, TD::zero());
        assert!(out.contains("active: no active entry"));
    }

    #[test]
    fn render_status_shows_overtime_parens() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("[YY-6]"), TD::hours(8) + TD::minutes(15));
        assert!(out.contains("worked: 08:15(-00:15)"));
        assert!(out.contains("remaining: -00:15"));
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
