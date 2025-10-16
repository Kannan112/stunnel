#!/bin/bash

# Stunnel Manager Environment Configuration
# Source this file to set up the environment: source export.sh

# === Core Configuration (REQUIRED) ===

# Path to stunnel configuration file - REQUIRED
if [ -z "$STUNNEL_CONFIG" ]; then
    echo "WARNING: STUNNEL_CONFIG not set. Using example: /etc/stunnel/stunnel.conf"
    export STUNNEL_CONFIG="/etc/stunnel/stunnel.conf"
fi

# Path to stunnel PID file - REQUIRED
if [ -z "$STUNNEL_PID_FILE" ]; then
    echo "WARNING: STUNNEL_PID_FILE not set. Using example: /tmp/stunnel.pid"
    export STUNNEL_PID_FILE="/tmp/stunnel.pid"
fi

# gRPC server port - REQUIRED
if [ -z "$GRPC_PORT" ]; then
    echo "WARNING: GRPC_PORT not set. Using example: 50055"
    export GRPC_PORT="50056"
fi

# Log level (debug, info, warn, error)
export LOG_LEVEL="${LOG_LEVEL:-info}"

# === Optional Configuration ===

# Path to SSL certificates directory
export SSL_CERT_DIR="${SSL_CERT_DIR:-/etc/stunnel/certs}"

# Default stunnel accept port
export STUNNEL_ACCEPT_PORT="${STUNNEL_ACCEPT_PORT:-443}"

# Default stunnel connect host
export STUNNEL_CONNECT_HOST="${STUNNEL_CONNECT_HOST:-127.0.0.1}"

# Default stunnel connect port
export STUNNEL_CONNECT_PORT="${STUNNEL_CONNECT_PORT:-8443}"

# Enable foreground mode for stunnel
export STUNNEL_FOREGROUND="${STUNNEL_FOREGROUND:-yes}"

# === Development Configuration ===

# Rust backtrace for debugging
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

# Rust log level
export RUST_LOG="${RUST_LOG:-stunnel_space=info}"

# === Helper Functions ===

# Print current configuration
print_config() {
    echo "=== Stunnel Manager Configuration ==="
    echo "STUNNEL_CONFIG: $STUNNEL_CONFIG"
    echo "STUNNEL_PID_FILE: $STUNNEL_PID_FILE"
    echo "GRPC_PORT: $GRPC_PORT"
    echo "LOG_LEVEL: $LOG_LEVEL"
    echo "SSL_CERT_DIR: $SSL_CERT_DIR"
    echo "STUNNEL_ACCEPT_PORT: $STUNNEL_ACCEPT_PORT"
    echo "STUNNEL_CONNECT_HOST: $STUNNEL_CONNECT_HOST"
    echo "STUNNEL_CONNECT_PORT: $STUNNEL_CONNECT_PORT"
    echo "STUNNEL_FOREGROUND: $STUNNEL_FOREGROUND"
    echo "RUST_BACKTRACE: $RUST_BACKTRACE"
    echo "RUST_LOG: $RUST_LOG"
    echo "====================================="
}

# Validate configuration
validate_config() {
    local valid=true
    
    # Check if config file exists
    if [ ! -f "$STUNNEL_CONFIG" ]; then
        echo "Warning: Config file not found: $STUNNEL_CONFIG"
        valid=false
    fi
    
    # Check if PID file directory is writable
    local pid_dir=$(dirname "$STUNNEL_PID_FILE")
    if [ ! -w "$pid_dir" ]; then
        echo "Warning: PID file directory not writable: $pid_dir"
        valid=false
    fi
    
    # Check if port is numeric
    if ! [[ "$GRPC_PORT" =~ ^[0-9]+$ ]]; then
        echo "Error: GRPC_PORT must be numeric: $GRPC_PORT"
        valid=false
    fi
    
    if [ "$valid" = true ]; then
        echo "Configuration validated successfully!"
    else
        echo "Configuration validation failed. Please check the warnings above."
        return 1
    fi
}

# Run the server with current configuration
run_server() {
    echo "Starting Stunnel Manager with current configuration..."
    print_config
    cargo run --release
}

# Run server in development mode
run_dev() {
    echo "Starting Stunnel Manager in development mode..."
    export RUST_LOG="stunnel_space=debug"
    export LOG_LEVEL="debug"
    print_config
    cargo run
}

# Build Docker image
build_docker() {
    echo "Building Docker image..."
    docker build -t stunnel-manager .
}

# Run Docker container with environment variables
run_docker() {
    echo "Running Docker container..."
    docker run -d \
        --name stunnel-manager \
        -p ${GRPC_PORT}:${GRPC_PORT} \
        -p 50000-50010:50000-50010 \
        -e STUNNEL_CONFIG="$STUNNEL_CONFIG" \
        -e STUNNEL_PID_FILE="$STUNNEL_PID_FILE" \
        -e GRPC_PORT="$GRPC_PORT" \
        -e LOG_LEVEL="$LOG_LEVEL" \
        -v $(pwd)/stunnel.conf:/etc/stunnel/stunnel.conf \
        stunnel-manager
}

# Show usage
usage() {
    echo "Usage: source export.sh"
    echo ""
    echo "Available commands after sourcing:"
    echo "  print_config    - Display current configuration"
    echo "  validate_config - Validate configuration settings"
    echo "  run_server      - Run the server with current config"
    echo "  run_dev         - Run in development mode"
    echo "  build_docker    - Build Docker image"
    echo "  run_docker      - Run Docker container"
    echo ""
    echo "Environment variables:"
    echo "  STUNNEL_CONFIG     - Path to stunnel config file"
    echo "  STUNNEL_PID_FILE   - Path to stunnel PID file"
    echo "  GRPC_PORT          - gRPC server port"
    echo "  LOG_LEVEL          - Log level (debug/info/warn/error)"
}

# Print config on source
echo "Stunnel Manager environment loaded!"
echo "Run 'usage' for available commands"
echo ""
print_config