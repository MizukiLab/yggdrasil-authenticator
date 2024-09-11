use crate::auth_agent::AuthAgent;
use crate::auth_error::AuthError;
use crate::auth_profile::AuthProfile;
use crate::{
    AuthRequest, AuthResponse, InvalidateRequest, RefreshRequest, RefreshResponse, SignoutRequest,
    ValidateRequest,
};
use reqwest::{Client, Proxy};
use std::error::Error;
use std::str;

/// A client for authenticating, refreshing, validating, invalidating, and signing out
/// through Yggdrasil's authentication API.
pub struct AuthClient {
    /// The base URL for the Yggdrasil authentication server.
    base_yggdrasil_url: String,

    /// An optional proxy URL for sending requests through a proxy server.
    proxy_url: Option<String>,
}

impl AuthClient {
    /// Creates a new `AuthClient` with the provided `base_yggdrasil_url` and optional `proxy_url`.
    ///
    /// # Arguments
    ///
    /// * `base_yggdrasil_url` - The base URL for the Yggdrasil authentication server.
    /// * `proxy_url` - An optional proxy URL for sending requests through a proxy server.
    ///
    /// # Returns
    ///
    /// An initialized `AuthClient`.
    pub fn new(base_yggdrasil_url: String, proxy_url: Option<String>) -> AuthClient {
        AuthClient {
            base_yggdrasil_url,
            proxy_url,
        }
    }

    /// Returns the proxy URL as an `Option<&str>`.
    ///
    /// # Returns
    ///
    /// An optional string slice representing the proxy URL, or `None` if no proxy URL is set.
    fn get_proxy(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }

    /// Authenticates the user with the Yggdrasil authentication server.
    ///
    /// # Arguments
    ///
    /// * `agent` - The authentication agent (e.g., Minecraft).
    /// * `username` - The user's username.
    /// * `password` - The user's password.
    /// * `client_token` - The client token used for authentication.
    /// * `request_user` - Whether to request user information in the response.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AuthResponse` on success or an `Error` on failure.
    pub async fn authenticate(
        &self,
        agent: AuthAgent,
        username: &str,
        password: &str,
        client_token: &str,
        request_user: bool,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        let content = serde_json::to_string(&AuthRequest::new(
            agent,
            username.to_string(),
            password.to_string(),
            client_token.to_string(),
            request_user,
        ))?;
        let result = send_post_request(
            &format!("{}/authenticate", self.base_yggdrasil_url),
            &content,
            self.get_proxy(),
        ).await?;
        let response: AuthResponse = serde_json::from_str(result.as_deref().unwrap_or(""))?;
        Ok(response)
    }

    /// Refreshes the user's access token with the Yggdrasil authentication server.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The current access token.
    /// * `client_token` - The client token used for authentication.
    /// * `request_user` - Whether to request user information in the response.
    /// * `selected_profile` - An optional profile to refresh.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `RefreshResponse` on success or an `Error` on failure.
    pub async fn refresh(
        &self,
        access_token: &str,
        client_token: &str,
        request_user: bool,
        selected_profile: Option<AuthProfile>,
    ) -> Result<RefreshResponse, Box<dyn Error>> {
        let content = serde_json::to_string(&RefreshRequest::new(
            access_token.to_string(),
            client_token.to_string(),
            request_user,
            selected_profile,
        ))?;
        let result = send_post_request(
            &format!("{}/refresh", self.base_yggdrasil_url),
            &content,
            self.get_proxy(),
        ).await?;
        let response: RefreshResponse = serde_json::from_str(result.as_deref().unwrap_or(""))?;
        Ok(response)
    }

    /// Validates the user's access token with the Yggdrasil authentication server.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token to validate.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` on success or an `Error` on failure.
    pub async fn validate(&self, access_token: &str) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string(&ValidateRequest::new(access_token.to_string()))?;
        send_post_request(
            &format!("{}/validate", self.base_yggdrasil_url),
            &content,
            self.get_proxy(),
        ).await?;
        Ok(())
    }

    /// Invalidates the user's access token and client token with the Yggdrasil authentication server.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access token to invalidate.
    /// * `client_token` - The client token to invalidate.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` on success or an `Error` on failure.
    pub async fn invalidate(
        &self,
        access_token: &str,
        client_token: &str,
    ) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string(&InvalidateRequest::new(
            access_token.to_string(),
            client_token.to_string(),
        ))?;
        send_post_request(
            &format!("{}/invalidate", self.base_yggdrasil_url),
            &content,
            self.get_proxy(),
        ).await?;
        Ok(())
    }

    /// Signs out the user from the Yggdrasil authentication server.
    ///
    /// # Arguments
    ///
    /// * `username` - The user's username.
    /// * `password` - The user's password.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` on success or an `Error` on failure.
    pub async fn signout(&self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string(&SignoutRequest::new(
            username.to_string(),
            password.to_string(),
        ))?;
        send_post_request(
            &format!("{}/signout", self.base_yggdrasil_url),
            &content,
            self.get_proxy(),
        ).await?;
        Ok(())
    }
}

async fn send_post_request(
    url: &str,
    json: &str,
    proxy: Option<&str>,
) -> Result<Option<String>, Box<dyn Error>> {
    // Build the client, with or without proxy
    let client = if let Some(proxy_url) = proxy {
        let proxy = Proxy::http(proxy_url)?;
        Client::builder().proxy(proxy).build()?
    } else {
        Client::new()
    };

    let res = client
        .post(url)
        .header("User-Agent", "yggdrasil-authenticator/0.1.0")
        .header("Accept-Charset", "UTF-8")
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json.to_string())
        .send()
        .await?;

    let status = res.status();

    // Validate, invalidate and sign out operations respond with this status
    if status == reqwest::StatusCode::NO_CONTENT {
        return Ok(None);
    }

    let body = res.bytes().await?;
    let mut response = str::from_utf8(&body)?.to_string();

    // Skip the BOM character for servers with hilarious encoding
    while response.starts_with("\u{FEFF}") {
        response = response[3..].to_string();
    }

    if status != reqwest::StatusCode::OK {
        // Handle this error if possible
        let error: Result<AuthError, serde_json::Error> = serde_json::from_str(&response);
        return match error {
            Ok(auth_error) => {
                Err(auth_error.into())
            }
            Err(_error) => {
                Err(format!("server status: {}, response: {}", status, response).into())
            }
        }
    }

    Ok(Some(response))
}
