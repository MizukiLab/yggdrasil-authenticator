#[cfg(test)]
mod tests {
    use mockito::{Matcher, Server};
    use serde_json::json;
    use std::error::Error;
    use yggdrasil_authenticator::auth_agent::AuthAgent;
    use yggdrasil_authenticator::auth_error::AuthError;
    use yggdrasil_authenticator::client::client::AuthClient;

    // Helper function to create a test client
    fn create_test_client(url: &str) -> AuthClient {
        AuthClient::new(url.to_string(), None)
    }

    #[tokio::test]
    async fn test_authenticate_success() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a successful authenticate response
        let _m = server.mock("POST", "/authenticate")
            .match_body(Matcher::Json(json!({
                "agent": {
                    "name": "Minecraft",
                    "version": 1
                },
                "username": "testuser",
                "password": "password",
                "clientToken": "client_token",
                "requestUser": false
            })))
            .with_status(200)
            .with_body(json!({
                "accessToken": "test_access_token",
                "clientToken": "client_token",
                "availableProfiles": [],
                "selectedProfile": null,
                "user": null
            }).to_string())
            .create();

        let client = create_test_client(server.url().as_str());
        let agent = AuthAgent::new("Minecraft".to_string(), 1);

        let response = client
            .authenticate(agent, "testuser", "password", "client_token", false)
            .await?;

        assert_eq!(response.access_token, "test_access_token");
        assert_eq!(response.client_token, "client_token");

        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_success() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a successful refresh response
        let _m = server.mock("POST", "/refresh")
            .match_body(Matcher::Json(json!({
                "accessToken": "old_access_token",
                "clientToken": "client_token",
                "requestUser": false,
            })))
            .with_status(200)
            .with_body(json!({
                "accessToken": "new_access_token",
                "clientToken": "client_token",
                "selectedProfile": null,
                "user": null
            }).to_string())
            .create();

        let client = create_test_client(server.url().as_str());
        let response = client
            .refresh("old_access_token", "client_token", false, None)
            .await?;

        assert_eq!(response.access_token, "new_access_token");

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_success() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a successful validate response (empty body)
        let _m = server.mock("POST", "/validate")
            .match_body(Matcher::Json(json!({
                "accessToken": "test_access_token"
            })))
            .with_status(204)
            .create();

        let client = create_test_client(server.url().as_str());
        let result = client.validate("test_access_token").await;

        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_invalidate_success() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a successful invalidate response (empty body)
        let _m = server.mock("POST", "/invalidate")
            .match_body(Matcher::Json(json!({
                "accessToken": "test_access_token",
                "clientToken": "client_token"
            })))
            .with_status(204)
            .create();

        let client = create_test_client(server.url().as_str());
        let result = client.invalidate("test_access_token", "client_token").await;

        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_signout_success() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a successful signout response (empty body)
        let _m = server.mock("POST", "/signout")
            .match_body(Matcher::Json(json!({
                "username": "testuser",
                "password": "password"
            })))
            .with_status(204)
            .create();

        let client = create_test_client(server.url().as_str());
        let result = client.signout("testuser", "password").await;

        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_auth_error_response() -> Result<(), Box<dyn Error>> {
        let mut server = Server::new_async().await;

        // Mock a failed authenticate response with error
        let _m = server.mock("POST", "/authenticate")
            .with_status(403)
            .with_body(json!({
                "error": "ForbiddenOperationException",
                "errorMessage": "Invalid credentials. Invalid username or password.",
                "cause": "User credentials are invalid"
            }).to_string())
            .create();

        let client = create_test_client(server.url().as_str());
        let agent = AuthAgent::new("Minecraft".to_string(), 1);

        let result = client
            .authenticate(agent, "invalid_user", "wrong_password", "client_token", true)
            .await;

        assert!(result.is_err());

        if let Err(e) = result {
            let auth_error = e.downcast_ref::<AuthError>().unwrap();
            assert_eq!(auth_error.error, "ForbiddenOperationException");
            assert_eq!(auth_error.error_message, "Invalid credentials. Invalid username or password.");
        }

        Ok(())
    }
}
