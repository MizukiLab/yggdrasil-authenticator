# Yggdrasil Authenticator Library

---

This Rust library provides an interface for interacting with Yggdrasil's authentication system. It includes models for handling authentication requests, responses, and errors, as well as a client for sending HTTP requests to authenticate, refresh, validate, and sign out users.

Implemented Standard: [authlib-injector](https://github.com/yushijinhun/authlib-injector/wiki/Yggdrasil-%E6%9C%8D%E5%8A%A1%E7%AB%AF%E6%8A%80%E6%9C%AF%E8%A7%84%E8%8C%83)

## Features

- **AuthClient**: The main client for handling authentication operations.
- **JSON Models**: Structs for serializing/deserializing request and response data, including agents, profiles, users, and errors.
- **Error Handling**: Custom error types for handling authentication failures.

## Sample Code

```rust
use yggdrasil_authenticator::client::AuthClient;
use yggdrasil_authenticator::model::{AuthAgent, AuthProfile};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = AuthClient::new(
        "https://myauthserver.com/auth-endpoint".to_string(), // No "/" at the end
        None, // No proxy URL
    );

    // Authenticate a user
    let agent = AuthAgent::new("Minecraft".to_string(), 1);
    let auth_response = client
        .authenticate(agent, "username", "password", "client_token", true)
        .await?;

    println!("Access Token: {}", auth_response.access_token);

    // Refresh token
    let refresh_response = client
        .refresh(&auth_response.access_token, &auth_response.client_token, true, None)
        .await?;

    println!("New Access Token: {}", refresh_response.access_token);

    Ok(())
}

```