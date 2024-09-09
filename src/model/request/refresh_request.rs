use crate::auth_profile::AuthProfile;
use serde::Serialize;

/// A request struct for refreshing an access token in Yggdrasil's authentication system.
///
/// This struct is used to represent a request to refresh the user's access token,
/// client token, and optionally request user information or refresh a selected profile.
#[derive(Serialize)]
pub struct RefreshRequest {
    /// The access token to refresh.
    #[serde(rename = "accessToken")]
    access_token: String,

    /// The client token associated with the user.
    #[serde(rename = "clientToken")]
    client_token: String,

    /// Whether to request user information in the response.
    #[serde(rename = "requestUser")]
    request_user: bool,

    /// The selected profile to refresh, if any. This field is not serialized if it is `None`.
    #[serde(rename = "selectedProfile", skip_serializing_if = "Option::is_none")]
    selected_profile: Option<AuthProfile>,
}

impl RefreshRequest {
    /// Creates a new `RefreshRequest` with the provided access token, client token, and optionally
    /// the selected profile and a flag indicating whether to request user information.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token to refresh.
    /// * `client_token` - The client token associated with the user.
    /// * `request_user` - Whether to request user information in the response.
    /// * `selected_profile` - An optional profile to refresh. Can be `None` if no profile is selected.
    ///
    /// # Returns
    ///
    /// A new `RefreshRequest` instance.
    pub fn new(
        access_token: String,
        client_token: String,
        request_user: bool,
        selected_profile: Option<AuthProfile>,
    ) -> RefreshRequest {
        RefreshRequest {
            access_token,
            client_token,
            request_user,
            selected_profile,
        }
    }
}
