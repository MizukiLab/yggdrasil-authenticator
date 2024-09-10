use crate::auth_profile::AuthProfile;
use crate::auth_user::AuthUser;
use serde::Deserialize;

/// A response struct for handling authentication results from Yggdrasil's authentication system.
///
/// This struct represents the response data returned by the authentication server, which includes
/// the access token, client token, available profiles, selected profile, and optional user information.
#[derive(Deserialize)]
pub struct AuthResponse {
    /// The access token assigned to the authenticated user.
    #[serde(rename = "accessToken")]
    pub access_token: String,

    /// The client token used for authenticating the user.
    #[serde(rename = "clientToken")]
    pub client_token: String,

    /// A list of available profiles for the authenticated user.
    #[serde(rename = "availableProfiles")]
    pub available_profiles: Vec<AuthProfile>,

    /// The profile selected by the user, if any.
    #[serde(rename = "selectedProfile")]
    pub selected_profile: Option<AuthProfile>,

    /// Optional user information for the authenticated user.
    #[serde(rename = "user")]
    pub user: Option<AuthUser>,
}
