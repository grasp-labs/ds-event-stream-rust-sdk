# Contributing to DS Event Stream Rust SDK

Thank you for your interest in contributing to the DS Event Stream Rust SDK! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/your-username/ds-event-stream-rust-sdk.git
   cd ds-event-stream-rust-sdk
   ```

3. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/grasp-labs/ds-event-stream-rust-sdk.git
   ```

## Development Setup

### Prerequisites

- Rust 1.61.0 or later
- A Kafka instance for testing (local or remote)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
# Set required environment variables
export KAFKA_BOOTSTRAP_SERVERS="localhost:9092"
export KAFKA_CONSUMER_GROUP="test-group"

# Run examples
cargo run --example producer
cargo run --example consumer
```

## Making Changes

### Code Style

- Follow Rust conventions and use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Ensure all tests pass with `cargo test`
- Add tests for new functionality

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for custom topic configuration
fix: resolve memory leak in consumer stream
docs: update README with new examples
test: add integration tests for producer
```

### Pull Request Process

1. **Create a feature branch** from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and ensure they pass all tests

3. **Update documentation** if you've added new features or changed APIs

4. **Push your branch** and create a pull request:

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Fill out the PR template** with a clear description of your changes

## Areas for Contribution

- **Bug fixes** - Report and fix issues
- **New features** - Add functionality to the SDK
- **Documentation** - Improve examples, docs, and comments
- **Performance** - Optimize existing code
- **Testing** - Add more comprehensive tests
- **Examples** - Create more usage examples

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- SDK version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Questions?

Feel free to open an issue for questions or start a discussion. We're happy to help!
