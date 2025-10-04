# Contributing to FMP-RS

Thank you for your interest in contributing to FMP-RS! We welcome contributions of all kinds.

## 🚀 Quick Start

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/fmp-rs.git
   cd fmp-rs
   ```

2. **Set up Development Environment**
   ```bash
   # Install Rust if you haven't already
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development dependencies
   cargo build
   ```

3. **Set up API Key for Testing**
   ```bash
   export FMP_API_KEY="your_test_api_key"
   # Or in PowerShell:
   # $env:FMP_API_KEY = "your_test_api_key"
   ```

## 📝 Development Workflow

### Running Tests

```bash
# Run unit tests
cargo test --lib

# Run all tests (requires API key)
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Check compilation
cargo check

# Run benchmarks
cargo bench
```

### Production Validation

```bash
# Run full validation suite
pwsh -ExecutionPolicy Bypass .\validate_phase2.ps1

# Or manually:
cargo test --all-features
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## 🏗️ Architecture

### Project Structure

```
src/
├── lib.rs              # Library root and re-exports
├── client.rs           # Basic FMP client
├── error.rs            # Error types and handling
├── production.rs       # Production executor with retry logic
├── cache.rs            # Intelligent caching system
├── bulk_processing.rs  # Memory-optimized bulk operations
├── production_client.rs # Production-grade client
├── models/             # Data models for API responses
│   ├── mod.rs
│   ├── quote.rs
│   ├── company.rs
│   └── ...
└── endpoints/          # API endpoint implementations
    ├── mod.rs
    ├── quote.rs
    ├── company_info.rs
    └── ...

examples/               # Usage examples
tests/                 # Integration tests
benches/               # Performance benchmarks
```

### Design Principles

1. **Type Safety**: All API responses are strongly typed
2. **Performance**: Intelligent caching and connection pooling
3. **Reliability**: Retry logic, rate limiting, error recovery
4. **Ergonomics**: Builder patterns, sensible defaults
5. **Testability**: Mock-friendly design, comprehensive tests

## 🧪 Adding New Features

### Adding New API Endpoints

1. **Create the Model** (if needed)
   ```rust
   // src/models/new_feature.rs
   use serde::Deserialize;
   
   #[derive(Debug, Deserialize, Clone, PartialEq)]
   pub struct NewFeature {
       pub field1: String,
       pub field2: Option<f64>,
   }
   ```

2. **Implement the Endpoint**
   ```rust
   // src/endpoints/new_feature.rs
   use crate::{FmpClient, Result};
   use crate::models::new_feature::NewFeature;
   
   pub struct NewFeatureEndpoint {
       client: FmpClient,
   }
   
   impl NewFeatureEndpoint {
       pub(crate) fn new(client: FmpClient) -> Self {
           Self { client }
       }
       
       pub async fn get_feature(&self, symbol: &str) -> Result<Vec<NewFeature>> {
           self.client
               .get(&format!("/new-feature/{}", symbol))
               .await
       }
   }
   ```

3. **Add Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use crate::FmpClient;
       
       #[tokio::test]
       async fn test_get_feature() {
           let client = FmpClient::new("test_key");
           let result = client.new_feature().get_feature("AAPL").await;
           // Add assertions based on expected behavior
       }
   }
   ```

4. **Update Client**
   ```rust
   // In src/client.rs
   pub fn new_feature(&self) -> NewFeatureEndpoint {
       NewFeatureEndpoint::new(self.clone())
   }
   ```

### Production Feature Guidelines

When adding production features:

- **Add caching support** if the data is relatively static
- **Implement rate limiting** for high-frequency endpoints  
- **Add metrics collection** for monitoring
- **Handle errors gracefully** with appropriate retry logic
- **Memory efficiency** for bulk operations
- **Comprehensive tests** including edge cases

## 📋 Pull Request Process

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/new-awesome-feature
   ```

2. **Make Changes**
   - Write code following project conventions
   - Add comprehensive tests
   - Update documentation if needed
   - Ensure all tests pass

3. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add new awesome feature"
   ```
   
   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `test:` for tests
   - `perf:` for performance improvements

4. **Push and Create PR**
   ```bash
   git push origin feature/new-awesome-feature
   ```
   
   Then create a pull request on GitHub.

### PR Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] Changelog entry added (for significant changes)
- [ ] Performance impact considered
- [ ] Backward compatibility maintained

## 🐛 Reporting Issues

### Bug Reports

Please include:

- **Description** of the issue
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Code sample** if applicable

### Feature Requests

Please include:

- **Use case** description
- **Proposed API** design
- **Alternatives considered**
- **FMP API documentation** reference if applicable

## 💡 Development Tips

### Testing with Mock Data

```rust
#[cfg(test)]
mod tests {
    use mockito::Server;
    use crate::FmpClient;
    
    #[tokio::test]
    async fn test_with_mock() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/quote/AAPL")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"symbol":"AAPL","price":150.0}]"#)
            .create();
            
        // Test implementation
        mock.assert();
    }
}
```

### Performance Testing

```rust
#[cfg(test)]
mod benches {
    use criterion::{criterion_group, criterion_main, Criterion};
    
    fn benchmark_feature(c: &mut Criterion) {
        c.bench_function("feature_name", |b| {
            b.iter(|| {
                // Code to benchmark
            });
        });
    }
    
    criterion_group!(benches, benchmark_feature);
    criterion_main!(benches);
}
```

### Cache Strategy Development

When adding caching for new endpoints:

```rust
// Determine appropriate TTL based on data frequency
let ttl = match endpoint {
    "/quote/*" => Duration::from_secs(30),      // Frequently updated
    "/profile/*" => Duration::from_secs(3600),  // Rarely updated
    "/news/*" => Duration::from_secs(300),      // Moderately updated
    _ => Duration::from_secs(300),              // Default
};
```

## 📞 Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs and request features
- **Discord**: Join our community Discord (link in README)
- **Email**: Contact maintainers directly for sensitive issues

## 🏆 Recognition

Contributors will be:
- Listed in the CONTRIBUTORS file
- Mentioned in release notes for significant contributions  
- Invited to join the maintainers team (for consistent contributors)

Thank you for helping make FMP-RS better! 🦀✨