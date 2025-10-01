# Contributing to Pandora Genesis SDK

Thank you for your interest in contributing to the Pandora Genesis SDK! This document provides guidelines and information for contributors.

## 🎯 How to Contribute

### 1. Fork and Clone
1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/B.1_COS.git
   cd B.1_COS
   ```

### 2. Set up Development Environment
1. Install Rust toolchain (latest stable):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install additional tools:
   ```bash
   cargo install cargo-audit cargo-udeps
   ```

3. Build the project:
   ```bash
   cd sdk
   cargo build --workspace
   ```

### 3. Make Changes
1. Create a new branch for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the coding standards below

3. Test your changes:
   ```bash
   cargo test --workspace --all-features
   cargo clippy --workspace -- -D warnings
   cargo fmt --check --workspace
   ```

### 4. Submit Pull Request
1. Commit your changes with a clear message
2. Push to your fork
3. Create a Pull Request with a detailed description

## 📝 Coding Standards

### Rust Code Style
- Follow standard Rust formatting: `cargo fmt`
- Use `cargo clippy -- -D warnings` to catch issues
- Write comprehensive tests for new functionality
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Commit Messages
Use conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(core): add new FepCell implementation`
- `fix(orchestrator): resolve skill registration bug`
- `docs(readme): update installation instructions`

### Code Organization
- Keep modules focused and cohesive
- Use feature flags for optional functionality
- Prefer composition over inheritance
- Follow the existing architectural patterns

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test --workspace --all-features

# Run tests for specific crate
cargo test -p pandora_core

# Run integration tests
cargo test -p integration_tests

# Run with output
cargo test --workspace -- --nocapture
```

### Writing Tests
- Write unit tests for individual functions
- Write integration tests for component interactions
- Use descriptive test names
- Test both success and failure cases
- Mock external dependencies when appropriate

## 🏗️ Architecture Guidelines

### Core Principles
1. **Modularity**: Each crate should have a clear, focused responsibility
2. **Extensibility**: Design for future expansion and customization
3. **Testability**: Code should be easy to test in isolation
4. **Documentation**: Public APIs must be well-documented

### Adding New Features
1. **Core Concepts**: Add to `pandora_core` if it's a fundamental concept
2. **Skills**: Add to `pandora_tools` if it's a specific capability
3. **Orchestration**: Add to `pandora_orchestrator` if it's about coordination
4. **Learning**: Add to `pandora_learning_engine` if it's about adaptation

### Feature Flags
- Use feature flags for optional dependencies
- Keep default features minimal
- Document feature flag usage in README
- Test both with and without features enabled

## 🐛 Bug Reports

When reporting bugs, please include:
1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce the bug
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: Rust version, OS, etc.
6. **Logs**: Any relevant error messages or logs

## 💡 Feature Requests

When requesting features, please include:
1. **Use Case**: Why is this feature needed?
2. **Proposed Solution**: How should it work?
3. **Alternatives**: What other approaches were considered?
4. **Implementation Notes**: Any technical considerations

## 📚 Documentation

### Code Documentation
- Use `///` for public API documentation
- Include examples in doc comments
- Document error conditions
- Explain the "why" not just the "what"

### README Updates
- Keep README current with new features
- Include usage examples
- Update installation instructions
- Document any breaking changes

## 🔄 Release Process

### Versioning
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] Release notes prepared

## 🤝 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the golden rule

### Communication
- Use clear, concise language
- Ask questions when unsure
- Provide context for discussions
- Be patient with newcomers

## 📞 Getting Help

- **Issues**: Use GitHub Issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Documentation**: Check the docs/ directory for detailed guides

## 🙏 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to the future of AI! 🚀
