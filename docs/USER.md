# Devin CLI User Guide

This guide provides instructions on how to install, configure, and use the Devin CLI tool.

## Overview

The Devin CLI is a command-line interface tool that allows you to interact with the Devin API. It provides the following commands:

- `configure`: Set up your Devin API token
- `show`: Display your currently configured API token
- `doctor`: Verify your CLI setup and API connection

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager

### Installing from Source

```bash
# Clone the repository
git clone https://github.com/appwiz/devin-cli.git
cd devin-cli

# Build and install
cargo install --path .
```

### Installing from Cargo

```bash
cargo install devin-cli
```

## Getting Started

### Obtaining a Devin API Token

Before using the Devin CLI, you need to obtain an API token:

1. Visit the [Devin API Portal](https://api.devin.com/signup) (replace with actual URL)
2. Create an account or log in to your existing account
3. Navigate to the "API Tokens" section
4. Click "Create New Token"
5. Give your token a name (e.g., "CLI Access")
6. Copy the generated token (you will only see it once)

### Configuring the CLI

Once you have your API token, configure the CLI:

```bash
devin configure YOUR_API_TOKEN
```

Replace `YOUR_API_TOKEN` with the token you obtained from the Devin API Portal.

You should see a confirmation message: "API token configured successfully."

## Using the CLI

### Viewing Your API Token

To view your configured API token:

```bash
devin show
```

This will display your token in a masked format for security (e.g., "API Token: abcd...-xyz").

### Verifying Your Setup

To verify that your CLI is properly configured and can connect to the API:

```bash
devin doctor
```

This will check:
- If an API token is configured
- If the CLI can connect to the Devin API
- If the token is valid

### Getting Help

For general help:

```bash
devin --help
```

For command-specific help:

```bash
devin configure --help
devin show --help
devin doctor --help
```

## Environment Variables

You can also set your API token using an environment variable:

```bash
export DEVIN_API_TOKEN=your_api_token
```

When set, the environment variable takes precedence over the configured token.

## Troubleshooting

### Common Issues

- **"API token not found"**: Run `devin configure` to set up your token
- **"Failed to connect to API"**: Check your internet connection
- **"Invalid API token"**: Verify your token is correct and not expired
- **"Permission denied"**: Ensure your token has the necessary permissions

### Getting Support

If you encounter issues not covered in this guide, please:

1. Check the [GitHub Issues](https://github.com/appwiz/devin-cli/issues) for similar problems
2. Open a new issue if your problem is not already reported

## Uninstalling

To uninstall the Devin CLI:

```bash
cargo uninstall devin-cli
```

## License

This project is licensed under the MIT License.
