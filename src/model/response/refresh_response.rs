use crate::auth_profile::AuthProfile;
use crate::auth_user::AuthUser;
use serde::Deserialize;

/// A response struct for handling the result of a token refresh request in Yggdrasil's authentication system.
///
/// This struct represents the response data returned after refreshing the user's access and client tokens,
/// including an optional selected profile and user information.
#[derive(Deserialize)]
pub struct RefreshResponse {
    /// The new access token assigned after the refresh.
    #[serde(rename = "accessToken")]
    access_token: String,

    /// The client token used for authentication.
    #[serde(rename = "clientToken")]
    client_token: String,

    /// The profile selected by the user, if any.
    #[serde(rename = "selectedProfile")]
    selected_profile: Option<AuthProfile>,

    /// Optional user information for the authenticated user, if available.
    user: Option<AuthUser>,
}
