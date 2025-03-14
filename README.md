# Devin CLI

A command-line interface for interacting with the Devin AI API.

## Installation

```bash
cargo install devin
```

Or clone the repository and build from source:

```bash
git clone https://github.com/appwiz/devin-cli.git
cd devin-cli
cargo build --release
```

## Configuration

Before using the CLI, you need to configure your API token:

```bash
devin configure <your-api-token>
```

You can also set the token using the `DEVIN_API_TOKEN` environment variable.

## Usage

### Interactive Session

Start an interactive session with Devin:

```bash
devin
```

This will open a shell-like interface where you can send messages to Devin and receive responses:

```
> my prompt
responses
responses
> my prompt
responses
responses
```

### Slash Commands

The interactive session supports the following slash commands:

- `/quit` - Exit the session
- `/help` - Show help message
- `/sessions` - List all available sessions
- `/connect <session_id>` - Connect to an existing session

### Connect to Existing Session

You can connect to an existing session using the `--session-id` option:

```bash
devin session --session-id <session-id>
```

Or using the shorter form:

```bash
devin session -s <session-id>
```

### Other Commands

- `devin show` - Show the configured API token
- `devin doctor` - Check if the CLI is set up correctly

## API Documentation

For more information about the Devin API, see the [official documentation](https://docs.devin.ai/api-reference/overview).

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.
