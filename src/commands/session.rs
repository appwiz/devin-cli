use crate::api::client::ApiClient;
use crate::config::get_api_token;
use anyhow::{anyhow, Result};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

// Slash commands
const CMD_QUIT: &str = "/quit";
const CMD_HELP: &str = "/help";
const CMD_SESSIONS: &str = "/sessions";
const CMD_CONNECT: &str = "/connect";

pub fn execute(session_id: Option<&str>) -> Result<()> {
    // Get API token
    let token = match get_api_token() {
        Ok(token) => token,
        Err(e) => {
            println!("{} {}", "✗ API token not configured:".red(), e);
            println!("\nRun 'devin configure <token>' to set up your API token.");
            return Err(e.into());
        }
    };
    
    // Create API client
    let api_client = ApiClient::new(&token);
    
    // Initialize readline
    let mut rl = DefaultEditor::new()?;
    
    // Connect to existing session or create a new one
    let mut current_session_id = match session_id {
        Some(id) => {
            println!("Connecting to existing session {}...", id);
            match api_client.get_session_details(id) {
                Ok(_) => {
                    println!("{}", "✓ Connected to session".green());
                    id.to_string()
                },
                Err(e) => {
                    println!("{} {}", "✗ Failed to connect to session:".red(), e);
                    return Err(anyhow!("Failed to connect to session: {}", e));
                }
            }
        },
        None => String::new()
    };
    
    println!("Welcome to Devin CLI");
    println!("Type {} to exit, {} for help", CMD_QUIT.yellow(), CMD_HELP.yellow());
    
    // Main interaction loop
    loop {
        let prompt = if current_session_id.is_empty() {
            "> ".to_string()
        } else {
            format!("[{}] > ", current_session_id)
        };
        
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                // Add to history
                rl.add_history_entry(&line)?;
                
                // Process the input
                if line.trim().is_empty() {
                    continue;
                }
                
                // Handle slash commands
                if line.starts_with('/') {
                    match line.trim() {
                        CMD_QUIT => {
                            println!("Goodbye!");
                            break;
                        },
                        CMD_HELP => {
                            println!("Available commands:");
                            println!("  {} - Exit the session", CMD_QUIT.yellow());
                            println!("  {} - Show this help message", CMD_HELP.yellow());
                            println!("  {} - List all sessions", CMD_SESSIONS.yellow());
                            println!("  {} <session_id> - Connect to an existing session", CMD_CONNECT.yellow());
                            println!("Any other input will be sent as a message to Devin.");
                        },
                        cmd if cmd.starts_with(CMD_CONNECT) => {
                            let parts: Vec<&str> = cmd.split_whitespace().collect();
                            if parts.len() < 2 {
                                println!("Usage: {} <session_id>", CMD_CONNECT.yellow());
                                continue;
                            }
                            
                            let new_session_id = parts[1];
                            match api_client.get_session_details(new_session_id) {
                                Ok(_) => {
                                    println!("{} {}", "✓ Connected to session".green(), new_session_id);
                                    current_session_id = new_session_id.to_string();
                                },
                                Err(e) => {
                                    println!("{} {}", "✗ Failed to connect to session:".red(), e);
                                }
                            }
                        },
                        CMD_SESSIONS => {
                            match api_client.list_sessions() {
                                Ok(sessions) => {
                                    if sessions.is_empty() {
                                        println!("No sessions found.");
                                    } else {
                                        println!("Available sessions:");
                                        for session in sessions {
                                            println!("  {} (created: {})", session.session_id, session.created_at);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{} {}", "✗ Failed to list sessions:".red(), e);
                                }
                            }
                        },
                        _ => {
                            println!("Unknown command. Type {} for help.", CMD_HELP.yellow());
                        }
                    }
                    continue;
                }
                
                // Send message to Devin
                if current_session_id.is_empty() {
                    // Create a new session with the first message
                    match api_client.create_session(&line) {
                        Ok(session_id) => {
                            println!("{} {}", "✓ Created new session:".green(), session_id);
                            current_session_id = session_id;
                            
                            // Wait for and display the response
                            match api_client.send_message(&current_session_id, "") {
                                Ok(response) => {
                                    println!("{}", response.message);
                                },
                                Err(e) => {
                                    println!("{} {}", "✗ Failed to get response:".red(), e);
                                }
                            }
                        },
                        Err(e) => {
                            println!("{} {}", "✗ Failed to create session:".red(), e);
                        }
                    }
                } else {
                    // Send message to existing session
                    match api_client.send_message(&current_session_id, &line) {
                        Ok(response) => {
                            println!("{}", response.message);
                        },
                        Err(e) => {
                            println!("{} {}", "✗ Failed to send message:".red(), e);
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_execute_no_token() {
        // Save the original token
        let original_token = env::var("DEVIN_API_TOKEN").ok();
        
        // Ensure no token is set
        env::remove_var("DEVIN_API_TOKEN");
        
        // Execute the command
        let result = execute(None);
        
        // Restore the original token
        if let Some(token) = original_token {
            env::set_var("DEVIN_API_TOKEN", token);
        }
        
        // Check the result
        assert!(result.is_err());
    }
}
