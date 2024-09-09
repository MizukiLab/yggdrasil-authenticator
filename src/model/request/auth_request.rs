use crate::auth_agent::AuthAgent;
use serde::Serialize;

/// Represents an authentication request to be sent to the Yggdrasil authentication server.
#[derive(Serialize)]
pub struct AuthRequest {
    /// The agent for which the request is being made (e.g., Minecraft).
    agent: AuthAgent,

    /// The username of the user.
    username: String,

    /// The password of the user.
    password: String,

    /// The client token used for authentication.
    #[serde(rename = "clientToken")]
    client_token: String,

    /// Whether to request user information in the response.
    #[serde(rename = "requestUser")]
    request_user: bool,
}

impl AuthRequest {
    /// Creates a new `AuthRequest` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent for which the request is being made (e.g., Minecraft).
    /// * `username` - The username of the user.
    /// * `password` - The password of the user.
    /// * `client_token` - The client token used for authentication.
    /// * `request_user` - Whether to request user information in the response.
    ///
    /// # Returns
    ///
    /// An initialized `AuthRequest` instance.
    pub fn new(
        agent: AuthAgent,
        username: String,
        password: String,
        client_token: String,
        request_user: bool,
    ) -> AuthRequest {
        AuthRequest {
            agent,
            username,
            password,
            client_token,
            request_user,
        }
    }
}
