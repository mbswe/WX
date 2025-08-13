# WX - Weather Data CLI Tool

WX is a Rust CLI application that fetches weather data from the Norwegian Meteorological Institute (yr.no) API. The application can fetch weather data by location name or latitude/longitude coordinates.

**CRITICAL: Always follow these instructions first and only fallback to search or bash commands when you encounter unexpected information that does not match the information here.**

## Working Effectively

### Prerequisites and Setup
- Rust toolchain is already installed (rustc 1.88.0, cargo 1.88.0)
- No additional dependencies need to be installed beyond what Cargo manages

### Building the Project
- **First build (downloads dependencies)**: `cargo build` -- takes ~2 minutes. NEVER CANCEL. Set timeout to 180+ seconds.
- **Clean build (dependencies cached)**: `cargo clean && cargo build` -- takes ~25 seconds. NEVER CANCEL. Set timeout to 60+ seconds.
- **Incremental build**: `cargo build` -- takes ~0.1 seconds when no changes made
- **Release build**: `cargo build --release` -- takes ~40 seconds. NEVER CANCEL. Set timeout to 90+ seconds.

### Testing
- Run tests: `cargo test` -- takes ~1 second (no tests currently defined)
- The project currently has no unit tests, but the test framework is available

### Running the Application
- **List locations**: `cargo run` or `./target/release/wx` -- creates locations.toml config file on first run
- **Get weather by location**: `cargo run "Effectsoft Göteborg"` or `./target/release/wx "Effectsoft Göteborg"`
- **Get weather by coordinates**: `cargo run 57.96 12.13` or `./target/release/wx 57.96 12.13`
- **Note**: Network requests will fail in sandboxed environments due to DNS restrictions, but the application logic and argument parsing work correctly

### Code Quality and Linting
- **Format code**: `cargo fmt` -- applies Rust standard formatting
- **Check formatting**: `cargo fmt --check` -- exits with error if code needs formatting
- **Lint code**: `cargo clippy` -- shows warnings and suggestions for code improvements
- **Fix clippy warnings**: `cargo clippy --fix` -- automatically applies suggested fixes
- **ALWAYS run `cargo fmt` and `cargo clippy` before committing changes**

## Validation Scenarios

### After Making Changes:
1. **ALWAYS build and test your changes**:
   ```bash
   cargo build  # Takes ~2 minutes first time, ~5 seconds incremental
   cargo test   # Takes ~1 second
   cargo clippy # Takes ~10 seconds, shows code quality warnings
   cargo fmt --check  # Verifies code formatting
   ```

2. **Test basic functionality**:
   ```bash
   cargo run  # Should list "Effectsoft Göteborg" and "Effectsoft Halmstad"
   cargo run arg1 arg2 arg3 arg4  # Should show "Too many arguments"
   cargo run invalid_lat invalid_lng  # Should show "Invalid latitude" error
   # Note: Unknown location names will cause panic - this is current behavior
   ```

3. **Test release build**:
   ```bash
   cargo build --release  # Takes ~40 seconds
   ./target/release/wx     # Should work identically to cargo run
   ```

## Project Structure

### Key Files and Directories:
- `src/main.rs` -- Main application logic, CLI argument handling, weather API interaction
- `src/structs.rs` -- Data structures for deserializing weather API responses
- `Cargo.toml` -- Project configuration and dependencies
- `locations.toml` -- Generated config file with predefined weather locations (created on first run)
- `target/` -- Build artifacts (excluded from git)

### Dependencies:
- `reqwest` (0.11.20) -- HTTP client with blocking and JSON features
- `serde` (1.0.188) -- Serialization/deserialization with derive features  
- `tokio` (1.32.0) -- Async runtime with macros and multi-thread features
- `serde_json` (1.0) -- JSON support for serde
- `toml` (0.7.6) -- TOML configuration file parsing
- `chrono` (0.4.28) -- Date and time handling

## Common Tasks and Expected Timings

### Build Times (NEVER CANCEL):
- **First build with dependency download**: ~2 minutes -- Set timeout to 180+ seconds
- **Clean build (dependencies cached)**: ~25 seconds -- Set timeout to 60+ seconds
- **Incremental build**: ~0.1 seconds when no changes made
- **Release build**: ~40 seconds -- Set timeout to 90+ seconds  
- **Test run**: ~0.6 seconds
- **Clippy linting**: ~10 seconds
- **Code formatting check**: ~0.1 seconds

### Typical Development Workflow:
1. Make code changes in `src/main.rs` or `src/structs.rs`
2. `cargo build` -- Build to check compilation (~25 seconds clean, ~0.1s incremental)
3. `cargo clippy` -- Check for code quality issues (~10 seconds)
4. `cargo fmt` -- Apply standard formatting (~0.1 seconds)
5. `cargo run` -- Test basic functionality
6. `cargo build --release` -- Final release build test (~40 seconds)

## Error Handling and Troubleshooting

### Common Issues:
- **Network errors**: Expected in sandboxed environments. The app logic works correctly even when API calls fail.
- **Unknown location panic**: Providing a location name not in `locations.toml` causes panic - this is current behavior
- **Formatting errors**: Run `cargo fmt` to fix automatically
- **Clippy warnings**: Run `cargo clippy --fix` to auto-fix where possible
- **Build errors**: Check `cargo build` output for specific compilation issues

### Manual Testing Scenarios:
- **Configuration creation**: Run `wx` without arguments to create `locations.toml`
- **Location listing**: Verify predefined locations are displayed
- **Argument validation**: Test invalid latitude/longitude inputs
- **Error handling**: Test with too many command line arguments

## Important Notes

### Development Guidelines:
- The application uses `unwrap()` in several places which will panic on errors - this is the current design
- The locations.toml file is created automatically with Swedish locations (Göteborg and Halmstad)
- All HTTP requests include a "Gurka 1.0" User-Agent header as required by the yr.no API
- Date/time handling converts UTC timestamps from the API to local time for display

### API Information:
- **Endpoint**: `https://api.met.no/weatherapi/locationforecast/2.0/complete`
- **Parameters**: `lat` (latitude), `lon` (longitude)
- **User-Agent**: Required header "Gurka 1.0"
- **Data format**: JSON with structured weather forecast data

### Code Formatting Standards:
- Use `cargo fmt` for consistent Rust formatting
- Address clippy warnings before committing
- Maintain existing code structure and patterns
- Follow Rust naming conventions and idioms

## Common Outputs for Reference

### cargo run (first time):
```
Created locations config file
Effectsoft Göteborg
Effectsoft Halmstad
```

### cargo run (subsequent times):
```
Effectsoft Göteborg
Effectsoft Halmstad
```

### locations.toml content:
```toml
"Effectsoft Göteborg" = { latitude = 57.960308, longitude = 12.126554 }
"Effectsoft Halmstad" = { latitude = 56.676086, longitude = 12.858977 }
```

### Repository root structure:
```
.
├── .git/
├── .github/
│   └── copilot-instructions.md
├── .gitignore
├── Cargo.toml
├── README.md
├── locations.toml  (created on first run)
├── src/
│   ├── main.rs
│   └── structs.rs
└── target/  (build artifacts)
```