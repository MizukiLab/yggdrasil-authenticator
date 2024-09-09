use serde::Deserialize;

/// Represents a user property in Yggdrasil's authentication system.
///
/// This struct contains information about a specific property of a user, including its name and value.
#[derive(Deserialize)]
pub struct AuthUserProperty {
    /// The name of the user property.
    name: String,

    /// The value of the user property.
    value: String,
}

/// Represents a user in Yggdrasil's authentication system.
///
/// This struct contains information about a user, including the user's unique identifier and a list of properties associated with the user.
#[derive(Deserialize)]
pub struct AuthUser {
    /// The unique identifier of the user.
    id: String,

    /// A list of properties associated with the user.
    properties: Vec<AuthUserProperty>,
}
