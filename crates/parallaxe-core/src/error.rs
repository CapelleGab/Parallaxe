use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export, rename = "IpcErrorCode"))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidArgument,
    PermissionDenied,
    RootOffline,
    AssetUnavailable,
    FormatUnsupported,
    DecoderFailed,
    ResourceLimit,
    DatabaseError,
    ScanCancelled,
    Conflict,
}

impl ErrorCode {
    pub fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::RootOffline | Self::AssetUnavailable | Self::DatabaseError | Self::ScanCancelled
        )
    }
}

/// Error shape shared by every IPC command. Never carries file contents.
#[derive(Debug, Clone, PartialEq, Serialize, thiserror::Error)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export, rename = "IpcError"))]
#[error("{code:?}: {message}")]
pub struct CoreError {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl CoreError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            retryable: code.is_retryable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_ipc_envelope() {
        let error = CoreError::new(ErrorCode::AssetUnavailable, "The file is not accessible.");
        let json = serde_json::to_value(&error).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "code": "ASSET_UNAVAILABLE",
                "message": "The file is not accessible.",
                "retryable": true
            })
        );
    }

    #[test]
    fn invalid_input_is_not_retryable() {
        assert!(!CoreError::new(ErrorCode::InvalidArgument, "bad id").retryable);
        assert!(!ErrorCode::PermissionDenied.is_retryable());
    }

    #[test]
    fn displays_code_and_message() {
        let error = CoreError::new(ErrorCode::Conflict, "already exists");
        assert_eq!(error.to_string(), "Conflict: already exists");
    }
}
