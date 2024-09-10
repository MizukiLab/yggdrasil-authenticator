use serde::Serialize;

/// A request struct for validating an access token in Yggdrasil's authentication system.
///
/// This struct is used to represent a request to validate the user's access token.
#[derive(Serialize)]
pub struct ValidateRequest {
    /// The access token to validate.
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

impl ValidateRequest {
    /// Creates a new `ValidateRequest` with the provided access token.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token to validate.
    ///
    /// # Returns
    ///
    /// A new `ValidateRequest` instance.
    pub fn new(access_token: String) -> ValidateRequest {
        ValidateRequest { access_token }
    }
}
