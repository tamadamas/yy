//! Issue storage: a single `issues.jsonl` in the work folder, resolved/created
//! by key (e.g. `YY-1`, `DFG-1234`).

use std::path::{Path, PathBuf};

use crate::model::{Id, Issue, IssueKind};
use crate::store::jsonl::{self, Line, Record};

/// Path to the issues file: `<work_folder>/issues.jsonl`.
pub fn path(work_folder: &Path) -> PathBuf {
    work_folder.join("issues.jsonl")
}

/// Read all [`Issue`] records, ignoring comments, malformed lines, and entries.
fn read_all(work_folder: &Path) -> anyhow::Result<Vec<Issue>> {
    let lines = jsonl::read(&path(work_folder))?;
    Ok(lines
        .into_iter()
        .filter_map(|line| match line {
            Line::Record(Record::Issue(issue)) => Some(issue),
            _ => None,
        })
        .collect())
}

/// Find an issue by its stable [`Id`].
pub fn find_by_id(work_folder: &Path, id: Id) -> anyhow::Result<Option<Issue>> {
    Ok(read_all(work_folder)?.into_iter().find(|i| i.id == id))
}

/// Resolve `key` to a stable [`Id`], creating a new `Issue` on first use.
/// `desc` (if given) becomes the new issue's title; falls back to `key`.
pub fn resolve_or_create(work_folder: &Path, key: &str, desc: Option<&str>) -> anyhow::Result<Id> {
    let existing = read_all(work_folder)?
        .into_iter()
        .find(|i| i.key.as_deref() == Some(key));

    if let Some(issue) = existing {
        return Ok(issue.id);
    }

    let mut issue = Issue::new(desc.unwrap_or(key), IssueKind::Task);
    issue.key = Some(key.to_string());

    let file_path = path(work_folder);
    let mut lines = jsonl::read(&file_path)?;
    lines.push(Line::Record(Record::Issue(issue.clone())));
    jsonl::write(&file_path, &lines)?;

    Ok(issue.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("yy-issues-test-{label}-{}", ulid::Ulid::generate()))
    }

    #[test]
    fn resolve_or_create_creates_on_first_use() {
        let work_folder = tmp_dir("create");
        let id = resolve_or_create(&work_folder, "YY-1", Some("Bootstrap")).unwrap();

        let issue = find_by_id(&work_folder, id).unwrap().unwrap();
        assert_eq!(issue.key, Some("YY-1".to_string()));
        assert_eq!(issue.title, "Bootstrap");
        assert_eq!(issue.kind, crate::model::IssueKind::Task);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn resolve_or_create_reuses_existing_key() {
        let work_folder = tmp_dir("reuse");
        let first = resolve_or_create(&work_folder, "YY-1", Some("Bootstrap")).unwrap();
        let second = resolve_or_create(&work_folder, "YY-1", Some("different desc")).unwrap();

        assert_eq!(first, second);

        let all = jsonl::read(&path(&work_folder)).unwrap();
        let issue_count = all
            .iter()
            .filter(|l| matches!(l, Line::Record(Record::Issue(_))))
            .count();
        assert_eq!(issue_count, 1);

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn resolve_or_create_falls_back_to_key_as_title_when_no_desc() {
        let work_folder = tmp_dir("no-desc");
        let id = resolve_or_create(&work_folder, "YY-2", None).unwrap();

        let issue = find_by_id(&work_folder, id).unwrap().unwrap();
        assert_eq!(issue.title, "YY-2");

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn find_by_id_returns_none_when_missing() {
        let work_folder = tmp_dir("missing");
        assert_eq!(
            find_by_id(&work_folder, crate::model::Id::new()).unwrap(),
            None
        );
    }
}
