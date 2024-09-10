use serde::Serialize;

/// Represents an authentication agent in Yggdrasil's authentication system.
///
/// This struct is used to identify the agent (e.g., Minecraft) making the authentication request,
/// including its name and version.
#[derive(Serialize)]
pub struct AuthAgent {
    /// The name of the authentication agent.
    pub name: String,

    /// The version of the authentication agent.
    pub version: i32,
}

impl AuthAgent {
    /// Creates a new `AuthAgent` with the provided name and version.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the authentication agent.
    /// * `version` - The version of the authentication agent.
    ///
    /// # Returns
    ///
    /// A new `AuthAgent` instance.
    pub fn new(name: String, version: i32) -> AuthAgent {
        AuthAgent { name, version }
    }
}
