//! clap parsing; thin wrapper around `core`; shared entry-spec flags.

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

#[cfg(test)]
mod tests {
    use super::*;

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
