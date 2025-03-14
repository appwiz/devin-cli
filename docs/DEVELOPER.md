# Devin CLI Developer Documentation

This document provides comprehensive information for developers who want to build, test, and contribute to the Devin CLI.

## Project Overview

The Devin CLI is a command-line interface tool written in Rust that interacts with the Devin API. It provides the following functionality:

- `configure`: Set up your Devin API token
- `show`: Display your currently configured API token
- `doctor`: Verify your CLI setup and API connection

## Project Structure

```
devin-cli/
├── Cargo.toml             # Project dependencies and metadata
├── src/
│   ├── api/               # API client implementation
│   │   ├── client.rs      # API client for Devin API
│   │   └── mod.rs         # API module exports
│   ├── commands/          # CLI command implementations
│   │   ├── configure.rs   # Configure command
│   │   ├── doctor.rs      # Doctor command
│   │   ├── show.rs        # Show command
│   │   └── mod.rs         # Commands module exports
│   ├── config/            # Configuration handling
│   │   ├── storage.rs     # Token storage using confy
│   │   └── mod.rs         # Config module exports
│   ├── lib.rs             # Library exports
│   └── main.rs            # CLI entry point
└── tests/                 # Test suite
    ├── integration/       # Integration tests
    │   ├── cli_test.rs    # CLI integration tests
    │   └── mod.rs         # Integration test exports
    └── unit/              # Unit tests
        ├── config_test.rs # Config unit tests
        └── mod.rs         # Unit test exports
```

## Dependencies

The project uses the following key dependencies:

- `clap`: Command-line argument parsing
- `confy`: Configuration file management
- `reqwest`: HTTP client for API requests
- `anyhow`: Error handling
- `serde`: Serialization/deserialization
- `colored`: Terminal text coloring
- `assert_cmd`: CLI testing utilities
- `mockito`: HTTP mocking for tests
- `tempfile`: Temporary file handling for tests

## Development Setup

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager

### Building the Project

Clone the repository and build the project:

```bash
git clone https://github.com/appwiz/devin-cli.git
cd devin-cli
cargo build
```

For a release build:

```bash
cargo build --release
```

The compiled binary will be available at `target/debug/devin` or `target/release/devin`.

## Testing

### Running Tests

Run the test suite:

```bash
cargo test
```

To run specific tests:

```bash
# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration

# Run a specific test
cargo test test_config_roundtrip
```

### Test Coverage

The project aims for at least 80% test coverage. To measure test coverage, we use `cargo-tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View the report
# Open tarpaulin-report.html in your browser
```

Current test coverage: 80.23%

### Test Structure

- **Unit Tests**: Located within the source files using Rust's `#[cfg(test)]` attribute
- **Integration Tests**: Located in the `tests/` directory
  - `tests/integration/`: Tests for CLI functionality
  - `tests/unit/`: Additional unit tests

## Development Workflow

1. **Create a Branch**: Create a new branch for your feature or bugfix
2. **Implement Changes**: Make your code changes
3. **Write Tests**: Add tests for your changes to maintain coverage
4. **Run Tests**: Ensure all tests pass
5. **Check Coverage**: Verify test coverage remains above 80%
6. **Submit PR**: Create a pull request with your changes

## Coding Guidelines

- Follow Rust's standard formatting (use `cargo fmt`)
- Run `cargo clippy` to catch common mistakes
- Maintain test coverage above 80%
- Document public API with rustdoc comments
- Keep the code modular and maintainable

## Debugging Tips

- Use `RUST_LOG=debug cargo run` for debug logging
- Test token storage in isolation with custom config paths
- Use `cargo test -- --nocapture` to see test output

## Common Issues

- **Token Storage Issues**: Ensure tests use isolated config paths
- **API Connection Failures**: Check network and token validity
- **Test Interference**: Tests should clean up after themselves

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Add your changes
4. Add tests for your changes
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the MIT License.
