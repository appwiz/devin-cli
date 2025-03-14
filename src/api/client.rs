use reqwest::blocking::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to connect to API: {0}")]
    ConnectionError(String),
    
    #[error("API request failed: {0}")]
    RequestError(String),
}

/// Client for interacting with the Devin API
pub struct ApiClient {
    client: Client,
    api_token: String,
    api_url: String,
}

impl ApiClient {
    /// Create a new API client with the given token
    pub fn new(token: &str) -> Self {
        Self::new_with_url(token, "https://api.devin.ai")
    }
    
    /// Create a new API client with a custom API URL
    pub fn new_with_url(token: &str, url: &str) -> Self {
        Self {
            client: Client::new(),
            api_token: token.to_string(),
            api_url: url.to_string(),
        }
    }
    
    /// Check if the API is reachable
    pub fn check_connection(&self) -> Result<(), ApiError> {
        // In a real implementation, this would make an actual API call
        // For now, we'll just return success if we have a token
        if self.api_token.is_empty() {
            return Err(ApiError::ConnectionError("API token is empty".to_string()));
        }
        
        // In a real implementation, we would use self.client to make an HTTP request
        // For example:
        // let response = self.client.get(&format!("{}/health", self.api_url))
        //     .header("Authorization", format!("Bearer {}", self.api_token))
        //     .send()
        //     .map_err(|e| ApiError::ConnectionError(e.to_string()))?;
        //
        // if !response.status().is_success() {
        //     return Err(ApiError::RequestError(format!("API returned status: {}", response.status())));
        // }
        
        Ok(())
    }
    
    /// Make a request to the API
    pub fn make_request(&self, endpoint: &str) -> Result<String, ApiError> {
        // This is a stub implementation to demonstrate how the client would be used
        // In a real implementation, we would use self.client to make an HTTP request
        if self.api_token.is_empty() {
            return Err(ApiError::ConnectionError("API token is empty".to_string()));
        }
        
        // Simulate a request
        if endpoint == "/error" {
            return Err(ApiError::RequestError("Endpoint returned an error".to_string()));
        }
        
        Ok(format!("Response from {}{}", self.api_url, endpoint))
    }
    
    /// Get the API URL
    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }
    
    /// Get the API token (masked)
    pub fn get_masked_token(&self) -> String {
        if self.api_token.len() <= 8 {
            return self.api_token.clone();
        }
        
        let visible_chars = 4;
        let first = &self.api_token[..visible_chars];
        let last = &self.api_token[self.api_token.len() - visible_chars..];
        format!("{}...{}", first, last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_creation() {
        let token = "test-token";
        let client = ApiClient::new(token);
        assert_eq!(client.api_token, token);
        assert_eq!(client.api_url, "https://api.devin.ai");
    }
    
    #[test]
    fn test_client_with_custom_url() {
        let token = "test-token";
        let url = "https://custom-api.example.com";
        let client = ApiClient::new_with_url(token, url);
        assert_eq!(client.api_token, token);
        assert_eq!(client.api_url, url);
    }
    
    #[test]
    fn test_check_connection() {
        let token = "test-token";
        let client = ApiClient::new(token);
        let result = client.check_connection();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_check_connection_with_empty_token() {
        let token = "";
        let client = ApiClient::new(token);
        let result = client.check_connection();
        assert!(result.is_err());
        match result {
            Err(ApiError::ConnectionError(msg)) => {
                assert!(msg.contains("API token is empty"));
            }
            _ => panic!("Expected ConnectionError"),
        }
    }
    
    #[test]
    fn test_make_request() {
        let token = "test-token";
        let client = ApiClient::new(token);
        let result = client.make_request("/test");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("/test"));
    }
    
    #[test]
    fn test_make_request_error() {
        let token = "test-token";
        let client = ApiClient::new(token);
        let result = client.make_request("/error");
        assert!(result.is_err());
        match result {
            Err(ApiError::RequestError(msg)) => {
                assert!(msg.contains("Endpoint returned an error"));
            }
            _ => panic!("Expected RequestError"),
        }
    }
    
    #[test]
    fn test_get_api_url() {
        let token = "test-token";
        let url = "https://custom-api.example.com";
        let client = ApiClient::new_with_url(token, url);
        assert_eq!(client.get_api_url(), url);
    }
    
    #[test]
    fn test_get_masked_token() {
        let token = "test-token-12345";
        let client = ApiClient::new(token);
        assert_eq!(client.get_masked_token(), "test...2345");
        
        let short_token = "1234";
        let client = ApiClient::new(short_token);
        assert_eq!(client.get_masked_token(), "1234");
    }
}
