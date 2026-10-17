pub const MAX_NAME_CHARS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    #[error("name is empty")]
    Empty,
    #[error("name is longer than {MAX_NAME_CHARS} characters")]
    TooLong,
}

/// Trims and collapses inner whitespace: `"  dark   wood "` becomes `"dark wood"`.
pub fn clean(raw: &str) -> Result<String, NameError> {
    let cleaned = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.is_empty() {
        return Err(NameError::Empty);
    }
    if cleaned.chars().count() > MAX_NAME_CHARS {
        return Err(NameError::TooLong);
    }
    Ok(cleaned)
}

/// Key used to detect duplicate tags: case-insensitive, accents kept.
pub fn normalize(cleaned: &str) -> String {
    cleaned.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_collapses_whitespace() {
        assert_eq!(clean("  dark \t  wood  ").unwrap(), "dark wood");
    }

    #[test]
    fn rejects_blank_names() {
        assert_eq!(clean(""), Err(NameError::Empty));
        assert_eq!(clean(" \n\t "), Err(NameError::Empty));
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert!(clean(&"é".repeat(MAX_NAME_CHARS)).is_ok());
        assert_eq!(
            clean(&"é".repeat(MAX_NAME_CHARS + 1)),
            Err(NameError::TooLong)
        );
    }

    #[test]
    fn normalizes_case_but_keeps_accents() {
        assert_eq!(normalize("Médiéval"), "médiéval");
        assert_ne!(normalize("Medieval"), normalize("Médiéval"));
        assert_eq!(normalize("WOOD"), normalize("wood"));
    }
}
