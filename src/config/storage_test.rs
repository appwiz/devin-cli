#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_roundtrip() {
        // Create a temporary directory for the config file
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config");
        
        // Set a custom config path for testing
        env::set_var("CONFY_CONFIG_PATH", config_path.to_str().unwrap());
        
        // Test saving and loading config
        let test_token = "test-token-123";
        set_api_token(test_token).unwrap();
        
        let loaded_token = get_api_token().unwrap();
        assert_eq!(loaded_token, test_token);
        
        // Clean up
        env::remove_var("CONFY_CONFIG_PATH");
    }
    
    #[test]
    fn test_missing_token() {
        // Create a temporary directory for the config file
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config");
        
        // Set a custom config path for testing
        env::set_var("CONFY_CONFIG_PATH", config_path.to_str().unwrap());
        
        // Test with no token configured
        let config = Config::default();
        save_config(&config).unwrap();
        
        let result = get_api_token();
        assert!(result.is_err());
        
        // Clean up
        env::remove_var("CONFY_CONFIG_PATH");
    }
}
