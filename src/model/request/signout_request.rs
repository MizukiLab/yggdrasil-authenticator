use serde::Serialize;

/// A request struct for signing out a user from Yggdrasil's authentication system.
///
/// This struct is used to represent a request to sign out the user by providing
/// their username and password.
#[derive(Serialize)]
pub struct SignoutRequest {
    /// The username of the user who is signing out.
    username: String,

    /// The password of the user who is signing out.
    password: String,
}

impl SignoutRequest {
    /// Creates a new `SignoutRequest` with the provided username and password.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `password` - The password of the user.
    ///
    /// # Returns
    ///
    /// A new `SignoutRequest` instance.
    pub fn new(username: String, password: String) -> SignoutRequest {
        SignoutRequest { username, password }
    }
}
