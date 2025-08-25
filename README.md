# Mopro Refactor

A Rust project with FFI bindings for multiple platforms, featuring a CLI tool and comprehensive end-to-end testing.

## Project Structure

```
mopro-refactor/
├── mopro-ffi/          # FFI bindings library
│   ├── src/
│   │   ├── lib.rs      # Main library with exported functions
│   │   └── mopro_ffi.udl # UniFFI interface definition
│   └── bin/
│       ├── ios.rs      # iOS binding generator
│       ├── android.rs  # Android binding generator
│       ├── web.rs      # Web/WASM binding generator
│       ├── flutter.rs  # Flutter binding generator
│       └── react-native.rs # React Native binding generator
├── cli/                # Command-line interface
│   └── src/
│       └── main.rs     # CLI application
├── test-e2e/           # End-to-end tests
│   └── src/
│       └── main.rs     # E2E test suite
└── Cargo.toml          # Workspace configuration
```

## Features

- **Multi-platform FFI bindings** using UniFFI and wasm-bindgen-rayon
- **Command-line interface** for building bindings
- **Comprehensive testing** with end-to-end test suite
- **Support for multiple platforms**: iOS, Android, Web, Flutter, React Native

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Cargo
- For iOS: Xcode and iOS SDK
- For Android: Android SDK
- For Web: wasm-pack (`cargo install wasm-pack`)

### Building Bindings

#### Using the CLI (Recommended)

```bash
# Build bindings for iOS
cargo run --bin cli build ios

# Build bindings for Android
cargo run --bin cli build android

# Build bindings for Web
cargo run --bin cli build web

# Build bindings for Flutter
cargo run --bin cli build flutter

# Build bindings for React Native
cargo run --bin cli build react-native

# List available platforms
cargo run --bin cli platforms
```

#### Direct Commands

```bash
# Navigate to mopro-ffi directory
cd mopro-ffi

# Generate iOS bindings (Swift)
cargo run --bin ios

# Generate Android bindings (Kotlin)
cargo run --bin android

# Generate Web bindings (JavaScript/WASM)
cargo run --bin web

# Generate Flutter bindings (Dart)
cargo run --bin flutter

# Generate React Native bindings (JavaScript)
cargo run --bin react-native
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run end-to-end tests
cargo run --bin cli test --e2e

# Or run e2e tests directly
cd test-e2e
cargo run
```

## Available Functions

The FFI library provides the following functions:

- `add_numbers(a: f64, b: f64) -> f64` - Add two numbers
- `create_user(id: u32, name: String, email: String) -> User` - Create a user object
- `calculate_square_root(value: f64) -> CalculationResult` - Calculate square root with error handling
- `sum_array(numbers: Vec<f64>) -> f64` - Sum an array of numbers
- `validate_email(email: String) -> bool` - Validate email format

## Generated Bindings

Bindings are generated in the following directories:

- **iOS**: `mopro-ffi/bindings/ios/` (Swift files)
- **Android**: `mopro-ffi/bindings/android/` (Kotlin files)
- **Web**: `mopro-ffi/bindings/web/` (JavaScript/WASM files)
- **Flutter**: `mopro-ffi/bindings/flutter/` (Dart files)
- **React Native**: `mopro-ffi/bindings/react-native/` (JavaScript files)

## Development

### Adding New Functions

1. Add the function to `mopro-ffi/src/lib.rs`
2. Add the function signature to `mopro-ffi/src/mopro_ffi.udl`
3. Add tests to `test-e2e/src/main.rs`
4. Rebuild bindings for all platforms

### Adding New Platforms

1. Create a new binary in `mopro-ffi/bin/`
2. Add the platform to the CLI in `cli/src/main.rs`
3. Update this README

## Dependencies

### mopro-ffi
- `uniffi` - FFI binding generation
- `wasm-bindgen` - WebAssembly bindings
- `wasm-bindgen-rayon` - Parallel processing for WASM
- `serde` - Serialization

### cli
- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `tracing` - Logging

### test-e2e
- `mopro-ffi` - The library being tested
- `tokio` - Async runtime
- `tracing` - Logging

## License

MIT License - see LICENSE file for details.
