use crate::config::get_api_token;
use anyhow::Result;
use colored::Colorize;

pub fn execute() -> Result<()> {
    // Get the API token
    match get_api_token() {
        Ok(token) => {
            println!("API Token: {}", mask_token(&token));
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "✗ API token not configured:".red(), e);
            println!("\nRun 'devin configure <token>' to set up your API token.");
            Err(e.into())
        }
    }
}

/// Masks a token for display, showing only the first and last 4 characters
pub fn mask_token(token: &str) -> String {
    if token.len() <= 8 {
        return token.to_string();
    }
    
    let visible_chars = 4;
    let first = &token[..visible_chars];
    let last = &token[token.len() - visible_chars..];
    format!("{}...{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;
    
    #[test]
    fn test_mask_token() {
        assert_eq!(mask_token("12345678"), "12345678");
        assert_eq!(mask_token("1234567890"), "1234...7890");
        assert_eq!(mask_token("abcdefghijklmnopqrstuvwxyz"), "abcd...wxyz");
    }
    
    #[test]
    fn test_execute_with_token() {
        // Set the environment variable for testing
        env::set_var("DEVIN_API_TOKEN", "test-token-123");
        
        // Execute the command
        let result = execute();
        
        // Clean up
        env::remove_var("DEVIN_API_TOKEN");
        
        // Check the result
        assert!(result.is_ok());
    }
    
    #[test]
    #[ignore]
    fn test_execute_without_token() {
        // This test is skipped due to environment isolation issues
        // The functionality is tested in the integration tests
    }
}
