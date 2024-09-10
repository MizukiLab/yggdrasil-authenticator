use serde::Deserialize;

/// Represents an authentication error in Yggdrasil's authentication system.
///
/// This struct contains information about the error encountered during authentication,
/// including a general error identifier, a detailed error message, and the cause of the error.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthError {
    /// A general identifier for the error.
    pub error: String,

    /// A detailed message explaining the error.
    #[serde(rename = "errorMessage")]
    pub error_message: String,

    /// The cause of the error.
    pub cause: String,
}

impl AuthError {
    /// Creates a new `AuthError` with the provided error details.
    ///
    /// # Arguments
    ///
    /// * `error` - A general identifier for the error.
    /// * `error_message` - A detailed message explaining the error.
    /// * `cause` - The cause of the error.
    ///
    /// # Returns
    ///
    /// A new `AuthError` instance.
    pub fn new(error: String, error_message: String, cause: String) -> AuthError {
        AuthError {
            error,
            error_message,
            cause,
        }
    }
}

impl std::fmt::Display for AuthError {
    /// Formats the `AuthError` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.error_message)
    }
}

impl std::error::Error for AuthError {
}