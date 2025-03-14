#[cfg(test)]
mod tests {
    use devin::config::{Config, get_api_token, save_api_token};
    use std::env;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.api_token, "");
    }
    
    #[test]
    fn test_env_var_token() {
        // Create a temporary directory for the config file
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().to_str().unwrap();
        
        // Create a unique config path for this test
        let unique_config_path = format!("{}/test_env_var_{}", config_path, std::process::id());
        std::fs::create_dir_all(&unique_config_path).unwrap();
        
        // Save the original environment variables
        let original_token = env::var("DEVIN_API_TOKEN").ok();
        let original_config_path = env::var("CONFY_CONFIG_PATH").ok();
        
        // Set a custom config path for testing and set the token
        env::set_var("CONFY_CONFIG_PATH", &unique_config_path);
        env::set_var("DEVIN_API_TOKEN", "test-token-cli");
        
        // Get the token
        let result = get_api_token();
        
        // Restore the original environment variables
        match original_token {
            Some(token) => env::set_var("DEVIN_API_TOKEN", token),
            None => env::remove_var("DEVIN_API_TOKEN"),
        }
        
        match original_config_path {
            Some(path) => env::set_var("CONFY_CONFIG_PATH", path),
            None => env::remove_var("CONFY_CONFIG_PATH"),
        }
        
        // Check the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-token-cli");
    }
    
    #[test]
    fn test_config_roundtrip() {
        // Create a temporary directory for the config file
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().to_str().unwrap();
        
        // Create a unique config path for this test
        let unique_config_path = format!("{}/test_roundtrip_{}", config_path, std::process::id());
        std::fs::create_dir_all(&unique_config_path).unwrap();
        
        // Save the original environment variables
        let original_token = env::var("DEVIN_API_TOKEN").ok();
        let original_config_path = env::var("CONFY_CONFIG_PATH").ok();
        
        // Set a custom config path for testing and remove any token
        env::set_var("CONFY_CONFIG_PATH", &unique_config_path);
        env::remove_var("DEVIN_API_TOKEN");
        
        // Save a token
        let token = "test-token-456";
        let save_result = save_api_token(token);
        assert!(save_result.is_ok());
        
        // Get the token
        let get_result = get_api_token();
        
        // Restore the original environment variables
        match original_token {
            Some(token) => env::set_var("DEVIN_API_TOKEN", token),
            None => env::remove_var("DEVIN_API_TOKEN"),
        }
        
        match original_config_path {
            Some(path) => env::set_var("CONFY_CONFIG_PATH", path),
            None => env::remove_var("CONFY_CONFIG_PATH"),
        }
        
        // Check the result
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), token);
    }
}
