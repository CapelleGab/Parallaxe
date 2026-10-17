pub mod app;
pub mod library;

use std::str::FromStr;

use parallaxe_core::error::{CoreError, ErrorCode};

/// IDs arrive as strings from the WebView and are never trusted.
fn parse_id<T: FromStr>(raw: &str, what: &str) -> Result<T, CoreError> {
    raw.parse()
        .map_err(|_| CoreError::new(ErrorCode::InvalidArgument, format!("Unknown {what}.")))
}

#[cfg(test)]
mod tests {
    use parallaxe_core::id::LibraryId;

    use super::*;

    #[test]
    fn parses_valid_ids() {
        let id = LibraryId::new();
        assert_eq!(
            parse_id::<LibraryId>(&id.to_string(), "library").unwrap(),
            id
        );
    }

    #[test]
    fn rejects_garbage_ids() {
        let error = parse_id::<LibraryId>("../../etc/passwd", "library").unwrap_err();
        assert_eq!(error.code, ErrorCode::InvalidArgument);
        assert_eq!(error.message, "Unknown library.");
    }
}
