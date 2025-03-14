use clap::{Parser, Subcommand};
use anyhow::Result;

mod api;
mod commands;
mod config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the API token
    Configure {
        /// The API token to use
        token: String,
    },
    
    /// Show the configured API token
    Show,
    
    /// Check if the CLI is set up correctly
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Configure { token }) => {
            commands::configure::execute(token)
        }
        Some(Commands::Show) => {
            commands::show::execute()
        }
        Some(Commands::Doctor) => {
            commands::doctor::execute()
        }
        None => {
            println!("No command specified. Use --help for usage information.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    
    #[test]
    fn test_main_no_command() {
        let result = main();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_cli_parse_configure() {
        let cli = Cli::parse_from(["devin", "configure", "test-token"]);
        match cli.command {
            Some(Commands::Configure { token }) => {
                assert_eq!(token, "test-token");
            }
            _ => panic!("Expected Configure command"),
        }
    }
    
    #[test]
    fn test_cli_parse_show() {
        let cli = Cli::parse_from(["devin", "show"]);
        match cli.command {
            Some(Commands::Show) => {}
            _ => panic!("Expected Show command"),
        }
    }
    
    #[test]
    fn test_cli_parse_doctor() {
        let cli = Cli::parse_from(["devin", "doctor"]);
        match cli.command {
            Some(Commands::Doctor) => {}
            _ => panic!("Expected Doctor command"),
        }
    }
}
