use crate::config::save_api_token;
use anyhow::Result;
use colored::Colorize;

pub fn execute(token: &str) -> Result<()> {
    save_api_token(token)?;
    println!("{}", "API token configured successfully".green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;
    
    #[test]
    fn test_execute() {
        // Create a temporary directory for the config file
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().to_str().unwrap();
        
        // Set a custom config path for testing
        env::set_var("CONFY_CONFIG_PATH", config_path);
        
        // Execute the command
        let token = "test-token-123";
        let result = execute(token);
        
        // Clean up
        env::remove_var("CONFY_CONFIG_PATH");
        
        // Check the result
        assert!(result.is_ok());
    }
}
