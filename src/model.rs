//! Issue, Entry, IssueKind, Id — plain data types (with `t`).
//!
//! Every stored record carries a type discriminator `t` so the file format stays
//! extensible and unknown types can be preserved on read. See architecture.md §3.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ulid::Ulid;

/// ULID-backed identifier shared by issues and entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub Ulid);

impl Id {
    pub fn new() -> Self {
        Id(Ulid::generate())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// What kind of work an issue represents. Drives exclusivity rules (config
/// `nonexclusive_kinds`) and pipeline classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    Task,
    Meeting,
    Recurring,
    Custom(String),
}

macro_rules! record_tag {
    ($name:ident, $value:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name;

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                if s == $value {
                    Ok($name)
                } else {
                    Err(serde::de::Error::custom(format!(
                        "expected t = \"{}\", got \"{}\"",
                        $value, s
                    )))
                }
            }
        }
    };
}

record_tag!(IssueTag, "issue");
record_tag!(EntryTag, "entry");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub t: IssueTag,
    pub id: Id,
    /// External reference (e.g. a tracker key), if any.
    pub key: Option<String>,
    pub title: String,
    pub kind: IssueKind,
    pub created_at: DateTime<Utc>,
}

impl Issue {
    pub fn new(title: impl Into<String>, kind: IssueKind) -> Self {
        Issue {
            t: IssueTag,
            id: Id::new(),
            key: None,
            title: title.into(),
            kind,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub t: EntryTag,
    pub id: Id,
    /// `None` = unassigned.
    pub issue_id: Option<Id>,
    pub start: DateTime<Utc>,
    /// `None` = currently running. Elapsed is always derived, never stored.
    pub end: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub note: Option<String>,
}

impl Entry {
    pub fn start_now(issue_id: Option<Id>) -> Self {
        Entry {
            t: EntryTag,
            id: Id::new(),
            issue_id,
            start: Utc::now(),
            end: None,
            tags: Vec::new(),
            note: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_round_trips_with_t_discriminator() {
        let issue = Issue::new("Bootstrap module skeleton", IssueKind::Task);

        let json = serde_json::to_string(&issue).unwrap();
        assert!(json.contains("\"t\":\"issue\""));

        let back: Issue = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, issue.id);
        assert_eq!(back.title, issue.title);
        assert_eq!(back.kind, issue.kind);
    }

    #[test]
    fn entry_round_trips_running_and_closed() {
        let running = Entry::start_now(Some(Id::new()));
        let json = serde_json::to_string(&running).unwrap();
        assert!(json.contains("\"t\":\"entry\""));
        let back: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, running.id);
        assert!(back.end.is_none());

        let mut closed = running.clone();
        closed.end = Some(Utc::now());
        let json = serde_json::to_string(&closed).unwrap();
        let back: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.end, closed.end);
    }

    #[test]
    fn custom_issue_kind_round_trips() {
        let issue = Issue::new("Ad-hoc", IssueKind::Custom("Sprint Review".into()));
        let json = serde_json::to_string(&issue).unwrap();
        let back: Issue = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kind, IssueKind::Custom("Sprint Review".into()));
    }

    #[test]
    fn wrong_t_value_is_rejected() {
        let bad = r#"{"t":"entry","id":"01ARZ3NDEKTSV4RRFFQ69G5FAV","key":null,"title":"x","kind":"task","created_at":"2026-01-01T00:00:00Z"}"#;
        let result: Result<Issue, _> = serde_json::from_str(bad);
        assert!(result.is_err());
    }
}
