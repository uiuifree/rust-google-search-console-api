//! Error types for the Google Search Console API client.

use std::fmt::{Debug, Display, Formatter};

/// Error type for Google Search Console API operations.
///
/// This enum represents all possible errors that can occur when using this library.
#[derive(Clone)]
pub enum GoogleApiError {
    /// HTTP connection error (network issues, DNS resolution, etc.).
    Connection(String),
    /// JSON parsing error (invalid response format).
    JsonParse(String),
    /// API error response with status code and message.
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
    },
    /// Authentication error (invalid or expired token).
    Auth(String),
}

impl Display for GoogleApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleApiError::Connection(e) => write!(f, "Connection error: {}", e),
            GoogleApiError::JsonParse(e) => write!(f, "JSON parse error: {}", e),
            GoogleApiError::Api { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            GoogleApiError::Auth(e) => write!(f, "Authentication error: {}", e),
        }
    }
}

impl Debug for GoogleApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for GoogleApiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_error_display() {
        let error = GoogleApiError::Connection("timeout".to_string());
        assert_eq!(format!("{}", error), "Connection error: timeout");
    }

    #[test]
    fn test_json_parse_error_display() {
        let error = GoogleApiError::JsonParse("invalid json".to_string());
        assert_eq!(format!("{}", error), "JSON parse error: invalid json");
    }

    #[test]
    fn test_api_error_display() {
        let error = GoogleApiError::Api {
            status: 403,
            message: "Forbidden".to_string(),
        };
        assert_eq!(format!("{}", error), "API error (403): Forbidden");
    }

    #[test]
    fn test_auth_error_display() {
        let error = GoogleApiError::Auth("token expired".to_string());
        assert_eq!(format!("{}", error), "Authentication error: token expired");
    }

    #[test]
    fn test_error_debug() {
        let error = GoogleApiError::Connection("test".to_string());
        assert_eq!(format!("{:?}", error), "Connection error: test");
    }

    #[test]
    fn test_error_clone() {
        let error = GoogleApiError::Api {
            status: 404,
            message: "Not Found".to_string(),
        };
        let cloned = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned));
    }

    #[test]
    fn test_error_is_std_error() {
        let error: Box<dyn std::error::Error> =
            Box::new(GoogleApiError::Connection("test".to_string()));
        assert!(error.to_string().contains("Connection error"));
    }
}
