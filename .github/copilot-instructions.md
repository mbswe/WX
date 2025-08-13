# WX - Weather CLI Application

WX is a Rust command-line application that fetches weather data from yr.no (Norwegian Meteorological Institute). The application can fetch weather forecasts by location name or by latitude/longitude coordinates.

**ALWAYS reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Bootstrap and Build
- Ensure Rust toolchain is available: `rustc --version && cargo --version`
- **NEVER CANCEL**: `cargo check` takes ~15-90 seconds (faster after dependencies are cached). Set timeout to 180+ seconds.
- **NEVER CANCEL**: `cargo build` takes ~17-22 seconds. Set timeout to 60+ seconds.
- **NEVER CANCEL**: `cargo build --release` takes ~38-40 seconds. Set timeout to 90+ seconds.
- **NEVER CANCEL**: `cargo test` takes <1 second (no tests exist). Set timeout to 30+ seconds.

### Code Quality and Linting
- **ALWAYS run before committing**: `cargo fmt` to format code automatically
- **ALWAYS run before committing**: `cargo clippy` to check for linting issues (takes <1 second, set timeout to 30+ seconds)
- Check formatting without fixing: `cargo fmt --check`
- The codebase currently has formatting issues and clippy warnings but builds successfully

### Run the Application
- **ALWAYS build first**: Run `cargo build` before testing functionality
- List available locations: `./target/debug/wx` or `cargo run`
- Get weather for a location: `./target/debug/wx "Effectsoft Göteborg"` or `cargo run -- "Effectsoft Göteborg"`
- Get weather by coordinates: `./target/debug/wx 57.960308 12.126554` or `cargo run -- 57.960308 12.126554`
- Test input validation: `./target/debug/wx invalid lat` (should show error message)

## Validation

### What Works in Any Environment
- **ALWAYS test these after making changes**:
  - Build both debug and release versions
  - Run with no arguments to list locations (creates `locations.toml` on first run)
  - Test argument validation with invalid inputs
  - Run linting and formatting checks

### Network-Dependent Features
- **Weather data fetching requires internet access** to api.met.no
- In environments without internet access, the app will panic with a DNS error when trying to fetch weather data
- This is expected behavior - the core functionality (config management, argument parsing) can still be validated

### Manual Testing Scenarios
- **ALWAYS run through these scenarios after making changes**:
  1. Clean build: `rm -rf target && cargo build`
  2. Config creation: `rm locations.toml && ./target/debug/wx` (should create config and list locations)
  3. Location listing: `./target/debug/wx` (should list "Effectsoft Göteborg" and "Effectsoft Halmstad")
  4. Input validation: `./target/debug/wx invalid input` (should show error and exit with code 1)
  5. Formatting check: `cargo fmt --check` (shows current formatting issues)
  6. Lint check: `cargo clippy` (shows current warnings but passes)

## Project Structure

### Repository Root
```
.
├── Cargo.toml          # Rust project configuration and dependencies
├── README.md           # Basic usage documentation
├── src/
│   ├── main.rs         # Main application logic and CLI handling
│   └── structs.rs      # Weather data structures for JSON deserialization
├── target/             # Build artifacts (gitignored)
├── locations.toml      # Runtime config file (created on first run, gitignored)
└── .github/
    └── copilot-instructions.md  # This file
```

### Key Dependencies
- `reqwest`: HTTP client for API calls
- `serde` + `serde_json`: JSON serialization/deserialization  
- `tokio`: Async runtime
- `chrono`: Date/time handling
- `toml`: Configuration file parsing

### Current Code Issues
- Code formatting does not follow rustfmt standards (run `cargo fmt` to fix)
- 5 clippy warnings exist but do not prevent compilation
- No unit tests exist in the project
- Error handling uses `unwrap()` which can cause panics

## Common Commands Reference

### Rust Toolchain Verification
```bash
rustc --version    # Should show 1.70+ 
cargo --version    # Should show 1.70+
```

### Build Commands (with timing expectations)
```bash
cargo check        # ~15-90s depending on cache. NEVER CANCEL - set 180s timeout
cargo build        # ~17-22s debug build. NEVER CANCEL - set 60s timeout  
cargo build --release  # ~38-40s release build. NEVER CANCEL - set 90s timeout
cargo clean        # Clean build artifacts
```

### Testing and Quality
```bash
cargo test         # <1s (0 tests). NEVER CANCEL - set 30s timeout
cargo clippy       # <1s linting. NEVER CANCEL - set 30s timeout
cargo fmt          # Format code automatically
cargo fmt --check  # Check formatting without changing files
```

### Application Usage
```bash
# Build and run scenarios
cargo build
./target/debug/wx                    # List locations
./target/debug/wx "Effectsoft Göteborg"  # Get weather (needs internet)
./target/debug/wx 57.96 12.13       # Get weather by coordinates (needs internet)
./target/debug/wx invalid input     # Test error handling

# Using cargo run
cargo run                           # List locations  
cargo run -- "Effectsoft Göteborg" # Get weather (needs internet)
cargo run -- 57.96 12.13          # Get weather by coordinates (needs internet)
```

## Development Workflow

### Making Changes
1. **ALWAYS build and test before making changes**: `cargo build && ./target/debug/wx`
2. Make your code changes
3. **Format code**: `cargo fmt`
4. **Check for issues**: `cargo clippy`
5. **Build and test**: `cargo build && ./target/debug/wx`
6. **Validate specific functionality** you changed
7. **For release**: `cargo build --release && ./target/release/wx`

### Adding Features
- Modify `src/main.rs` for CLI logic and main application flow
- Modify `src/structs.rs` for data structures (weather API response format)
- Update `Cargo.toml` if adding new dependencies
- No existing test framework - consider adding tests if making significant changes

### Debugging Network Issues
- Weather fetching requires internet access to `api.met.no`
- DNS errors or connection timeouts are expected in restricted environments
- Test core functionality (config, parsing, validation) independently of network calls
- Use `RUST_BACKTRACE=1` for detailed error traces

## Troubleshooting

### Common Issues
- **"cannot find binary"**: Run `cargo build` first
- **DNS errors**: Expected without internet access, core app functions still work
- **Formatting failures**: Run `cargo fmt` to auto-fix
- **Clippy warnings**: Check output but warnings don't prevent builds
- **Long build times**: Normal on first build due to dependency compilation

### Recovery Commands
```bash
cargo clean         # Clean all build artifacts
rm -rf target        # Remove build directory completely  
rm locations.toml    # Reset application config
cargo build          # Rebuild from scratch
```

This application demonstrates a typical Rust CLI tool structure with external API integration, configuration management, and argument parsing.