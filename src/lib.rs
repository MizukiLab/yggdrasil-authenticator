// JSON models for networking.
mod model {
    // Agent JSON model.
    pub mod auth_agent; // Defines the JSON model for authentication agents.

    // Error JSON model.
    pub mod auth_error; // Defines the JSON model for authentication errors.

    // User JSON model.
    pub mod auth_user; // Defines the JSON model for user information.

    // Profile JSON model.
    pub mod auth_profile; // Defines the JSON model for user profiles.

    // JSON models for requests.
    pub mod request; // Contains request-related JSON models.
    pub use self::request::*; // Re-exports request models for easier access.

    // JSON models for responses.
    pub mod response; // Contains response-related JSON models.
    pub use self::response::*; // Re-exports response models for easier access.
}

mod client {
    pub mod client; // Contains the client implementation for interacting with the authentication system.
}

// Re-exports all models and client modules for easier access from the top level.
pub use model::*;
