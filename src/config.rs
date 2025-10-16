use std::env;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub config_path: String,
    pub pid_file: String,
    pub grpc_port: String,
    pub log_level: String,
}

#[derive(Debug)]
pub struct ConfigError {
    missing_vars: Vec<String>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "Missing required environment variables: {}",
            self.missing_vars.join(", ")
        )
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut missing_vars = Vec::new();

        // Get config path - REQUIRED
        let config_path = match env::var("STUNNEL_CONFIG") {
            Ok(path) => path,
            Err(_) => {
                missing_vars.push("STUNNEL_CONFIG".to_string());
                String::new()
            }
        };

        // Get PID file path - REQUIRED
        let pid_file = match env::var("STUNNEL_PID_FILE") {
            Ok(path) => path,
            Err(_) => {
                missing_vars.push("STUNNEL_PID_FILE".to_string());
                String::new()
            }
        };

        // Get gRPC port - REQUIRED
        let grpc_port = match env::var("GRPC_PORT") {
            Ok(port) => port,
            Err(_) => {
                missing_vars.push("GRPC_PORT".to_string());
                String::new()
            }
        };

        // Get log level - OPTIONAL with default
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string());

        // If any required variables are missing, return error
        if !missing_vars.is_empty() {
            return Err(ConfigError { missing_vars });
        }

        Ok(Config {
            config_path,
            pid_file,
            grpc_port,
            log_level,
        })
    }

    pub fn get_grpc_address(&self) -> String {
        format!("[::]:{}", self.grpc_port)
    }

    pub fn print_config(&self) {
        println!("=== Server Configuration ===");
        println!("gRPC Port: {}", self.grpc_port);
        println!("Config Path: {}", self.config_path);
        println!("PID File: {}", self.pid_file);
        println!("Log Level: {}", self.log_level);
        println!("===========================");
    }
}