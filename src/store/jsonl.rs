//! Generic line read/write, atomic writes, type dispatch, error preservation.
//!
//! A JSONL file is a sequence of [`Line`]s. Reading never drops data: comment lines
//! (`#…`) and lines that fail to parse are kept verbatim so a re-write round-trips a
//! hand-edited file byte-faithfully (modulo the parsed/typed records themselves).
//! Records are dispatched on the `t` discriminator; unknown `t` values are kept as
//! [`Record::Unknown`] rather than dropped.

use std::fs;
use std::io::Write as _;
use std::path::Path;

use crate::model::{Entry, Issue};

/// One line of a JSONL file, preserved well enough to write back unchanged.
#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    /// A `#`-prefixed comment line, kept verbatim (including the `#`).
    Comment(String),
    /// A successfully parsed, typed record.
    Record(Record),
    /// A line that isn't blank/comment but failed to parse as JSON or didn't carry a
    /// recognizable `t`. Kept verbatim so nothing is silently lost.
    Malformed { raw: String, error: String },
}

/// A parsed record, dispatched on its `t` field.
#[derive(Debug, Clone, PartialEq)]
pub enum Record {
    Issue(Issue),
    Entry(Entry),
    /// Valid JSON with a `t` this build doesn't know about — preserved as-is so a
    /// future record type survives a round-trip through an older binary.
    Unknown(serde_json::Value),
}

impl Record {
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        match self {
            Record::Issue(issue) => serde_json::to_value(issue),
            Record::Entry(entry) => serde_json::to_value(entry),
            Record::Unknown(value) => Ok(value.clone()),
        }
    }
}

/// Parse a single non-empty, non-comment JSONL line into a [`Record`], or explain why
/// it couldn't be parsed.
fn parse_record(raw: &str) -> Result<Record, String> {
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| format!("invalid JSON: {e}"))?;

    let t = value
        .get("t")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "missing string field \"t\"".to_string())?;

    match t {
        "issue" => serde_json::from_value(value)
            .map(Record::Issue)
            .map_err(|e| format!("t=\"issue\" but fields don't match Issue: {e}")),
        "entry" => serde_json::from_value(value)
            .map(Record::Entry)
            .map_err(|e| format!("t=\"entry\" but fields don't match Entry: {e}")),
        _ => Ok(Record::Unknown(value)),
    }
}

/// Parse JSONL content into ordered [`Line`]s. Blank lines are dropped (as on
/// write, blank lines are not re-emitted); comments and malformed lines survive.
pub fn parse(content: &str) -> Vec<Line> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                return Line::Comment(line.to_string());
            }
            match parse_record(line) {
                Ok(record) => Line::Record(record),
                Err(error) => Line::Malformed {
                    raw: line.to_string(),
                    error,
                },
            }
        })
        .collect()
}

/// Read and parse a JSONL file. A missing file is treated as empty (nothing tracked
/// yet), matching first-run behavior.
pub fn read(path: &Path) -> anyhow::Result<Vec<Line>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(parse(&content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

/// Serialize lines back to JSONL text: comments and malformed lines verbatim, records
/// as one compact JSON object per line.
pub fn render(lines: &[Line]) -> serde_json::Result<String> {
    let mut out = String::new();
    for line in lines {
        match line {
            Line::Comment(raw) | Line::Malformed { raw, .. } => {
                out.push_str(raw);
            }
            Line::Record(record) => {
                out.push_str(&serde_json::to_string(&record.to_json()?)?);
            }
        }
        out.push('\n');
    }
    Ok(out)
}

/// Write lines to `path` atomically: write a temp file in the same directory, then
/// rename over the target. Never writes in place, so a crash mid-write can't corrupt
/// the existing file.
pub fn write(path: &Path, lines: &[Line]) -> anyhow::Result<()> {
    let content = render(lines)?;

    if let Some(dir) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(dir)?;
    }

    let tmp_path = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("jsonl")
    ));

    let mut tmp = fs::File::create(&tmp_path)?;
    tmp.write_all(content.as_bytes())?;
    tmp.sync_all()?;
    drop(tmp);

    fs::rename(&tmp_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::IssueKind;

    #[test]
    fn round_trips_comments_and_malformed_lines() {
        let content = "\
# a hand-written comment
not json at all
{\"t\":\"entry\",\"id\":\"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\"issue_id\":null,\"start\":\"2026-01-01T09:00:00Z\",\"end\":null,\"tags\":[],\"note\":null}
{\"t\":\"future_type\",\"whatever\":123}
";
        let lines = parse(content);
        assert_eq!(lines.len(), 4);
        assert!(matches!(&lines[0], Line::Comment(c) if c == "# a hand-written comment"));
        assert!(matches!(&lines[1], Line::Malformed { raw, .. } if raw == "not json at all"));
        assert!(matches!(&lines[2], Line::Record(Record::Entry(_))));
        assert!(matches!(&lines[3], Line::Record(Record::Unknown(_))));

        let rendered = render(&lines).unwrap();
        // Re-parsing the rendered output must preserve the same shape (byte-identical
        // is not required for the JSON-object lines, only for comment/malformed ones).
        let reparsed = parse(&rendered);
        assert_eq!(reparsed.len(), 4);
        assert_eq!(reparsed[0], lines[0]);
        assert_eq!(reparsed[1], lines[1]);
    }

    #[test]
    fn write_then_read_is_atomic_and_lossless() {
        let dir = std::env::temp_dir().join(format!("yy-jsonl-test-{}", ulid::Ulid::generate()));
        let path = dir.join("entries").join("2026-01.jsonl");

        let lines = vec![
            Line::Comment("# my notes".to_string()),
            Line::Record(Record::Issue(Issue::new("Bootstrap", IssueKind::Task))),
            Line::Malformed {
                raw: "{oops".to_string(),
                error: "invalid JSON: whatever".to_string(),
            },
        ];

        write(&path, &lines).unwrap();
        assert!(path.exists());
        assert!(!path.with_extension("tmp").exists());

        let read_back = read(&path).unwrap();
        assert_eq!(read_back.len(), 3);
        assert_eq!(read_back[0], lines[0]);
        assert!(matches!(&read_back[1], Line::Record(Record::Issue(_))));
        // The error message is recomputed on re-parse, not stored verbatim; only the
        // raw text is guaranteed to round-trip.
        assert!(matches!(&read_back[2], Line::Malformed { raw, .. } if raw == "{oops"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_file_reads_as_empty() {
        let path =
            std::env::temp_dir().join(format!("yy-jsonl-missing-{}.jsonl", ulid::Ulid::generate()));
        assert_eq!(read(&path).unwrap(), Vec::new());
    }
}
