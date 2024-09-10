use serde::Serialize;

/// A request struct for invalidating an access token and client token.
///
/// This struct is used to represent a request to invalidate the user's
/// access token in Yggdrasil's authentication system.
#[derive(Serialize)]
pub struct InvalidateRequest {
    /// The access token to invalidate.
    #[serde(rename = "accessToken")]
    pub access_token: String,

    /// The client token.
    #[serde(rename = "clientToken")]
    pub client_token: String,
}

impl InvalidateRequest {
    /// Creates a new `InvalidateRequest` with the provided access token and client token.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token to invalidate.
    /// * `client_token` - The client token.
    ///
    /// # Returns
    ///
    /// A new `InvalidateRequest` instance.
    pub fn new(access_token: String, client_token: String) -> InvalidateRequest {
        InvalidateRequest {
            access_token,
            client_token,
        }
    }
}
