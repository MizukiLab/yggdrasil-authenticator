use serde::{Deserialize, Serialize};

/// Represents an authentication profile in Yggdrasil's authentication system.
///
/// This struct contains information about a user's profile, including the profile's name and ID.
#[derive(Serialize, Deserialize)]
pub struct AuthProfile {
    /// The name of the authentication profile.
    pub name: String,

    /// The unique identifier for the authentication profile.
    pub id: String,
}

impl AuthProfile {
    /// Creates a new `AuthProfile` with the provided name and ID.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the authentication profile.
    /// * `id` - The unique identifier for the authentication profile.
    ///
    /// # Returns
    ///
    /// A new `AuthProfile` instance.
    pub fn new(name: String, id: String) -> AuthProfile {
        AuthProfile { name, id }
    }
}
