# Stunnel Space - Rust gRPC Manager for Stunnel

A Rust implementation of a gRPC-based stunnel configuration manager, providing programmatic control over stunnel instances.

## Project Structure

```
stunnel-space/
├── Cargo.toml           # Rust project configuration
├── Cargo.lock           # Dependency lock file
├── build.rs             # Build script for proto generation
├── Makefile             # Build automation
├── Dockerfile           # Container build configuration
├── README.md            # This file
├── .gitignore           # Git ignore patterns
├── stunnel.conf         # Sample stunnel configuration
├── proto/
│   └── stunnel.proto    # gRPC service definitions
└── src/
    └── main.rs          # Main application code
```

## Generated Files

During build, the proto files are compiled to:
- `target/debug/build/stunnel-space-*/out/stunnel.rs` (debug builds)
- `target/release/build/stunnel-space-*/out/stunnel.rs` (release builds)

These are automatically included via the `tonic::include_proto!` macro.

## Building

### Local Build
```bash
# Debug build
make build
# or
cargo build

# Release build
make release
# or
cargo build --release

# Generate proto files (happens automatically during build)
make protogen
```

### Docker Build
```bash
docker build -t stunnel-manager .
```

## Running

### Local
```bash
# Default configuration
cargo run

# Custom configuration
STUNNEL_CONFIG=/path/to/stunnel.conf STUNNEL_PID_FILE=/path/to/stunnel.pid cargo run
```

### Docker
```bash
# Run with default port mappings
docker run -p 50055:50055 -p 50000-50010:50000-50010 stunnel-manager

# Or use the helper function from export.sh
source export.sh
run_docker
```

## Port Configuration

The application exposes the following ports:
- **50055**: gRPC server (configurable via GRPC_PORT)
- **50000-50010**: Stunnel service ports for SSL/TLS tunneling

These ports allow you to configure multiple stunnel services, each listening on a different port in the 50000-50010 range.

## gRPC API

The service listens on port `50055` and provides:

- **ReloadConfig**: Validate and reload stunnel configuration
- **GetStatus**: Check stunnel status and active connections
- **UpdateConfig**: Update configuration with validation
- **GenerateConfig**: Generate new stunnel configuration
- **AddProvider**: Add new service providers to existing config

## Development

### Prerequisites
- Rust 1.73+
- Protocol Buffers compiler (protoc)
- Stunnel (for testing)

### Testing
```bash
make test
```

### Code Formatting
```bash
make fmt
```

### Linting
```bash
make clippy
```

## Environment Variables

The application can be configured using environment variables. You have three options:

### Option 1: Using .env file
```bash
# Copy the example file
cp .env.example .env

# Edit the .env file with your configuration
nano .env

# Run the application (will auto-load .env)
cargo run
```

### Option 2: Using export.sh
```bash
# Source the export script
source export.sh

# View current configuration
print_config

# Run the server
run_server
```

### Option 3: Direct environment variables
```bash
export STUNNEL_CONFIG=/path/to/stunnel.conf
export STUNNEL_PID_FILE=/path/to/stunnel.pid
export GRPC_PORT=8080
cargo run
```

### Available Variables

- `STUNNEL_CONFIG`: Path to stunnel configuration file (default: `./stunnel.conf`)
- `STUNNEL_PID_FILE`: Path to stunnel PID file (default: `/tmp/stunnel.pid`)
- `GRPC_PORT`: gRPC server port (default: `50055`)
- `LOG_LEVEL`: Log level - debug, info, warn, error (default: `info`)
- `SSL_CERT_DIR`: Path to SSL certificates directory
- `STUNNEL_ACCEPT_PORT`: Default stunnel accept port
- `STUNNEL_CONNECT_HOST`: Default stunnel connect host
- `STUNNEL_CONNECT_PORT`: Default stunnel connect port
- `RUST_LOG`: Rust log configuration (default: `stunnel_space=info`)

See `.env.example` for a complete list of available variables

## License

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.73%2B-orange)
![Docker](https://img.shields.io/badge/Docker-ready-blue)
