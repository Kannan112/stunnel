use std::fs;
use std::path::Path;
use std::process::Command;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use crate::stunnel::Connection;

/// Read the PID from a file and check if the process is running
pub fn get_stunnel_pid(pid_file: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let pid_content = fs::read_to_string(pid_file)?;
    let pid: i32 = pid_content.trim().parse()?;
    
    // Check if process is running by sending signal 0
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => Ok(pid),
        Err(_) => Err("Process not running".into()),
    }
}

/// Get active stunnel connections using netstat
pub fn get_active_connections() -> Vec<Connection> {
    let mut connections = Vec::new();
    
    // Run netstat to get TCP connections
    let output = Command::new("netstat")
        .args(&["-tnp"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("stunnel") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 5 {
                    connections.push(Connection {
                        service_name: String::new(),
                        local_address: fields[3].to_string(),
                        remote_address: fields[4].to_string(),
                        bytes_sent: 0,
                        bytes_received: 0,
                    });
                }
            }
        }
    }
    
    connections
}

/// Validate stunnel configuration file
pub fn validate_stunnel_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("stunnel")
        .args(&["-fd", "0", "-test", config_path])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Config validation failed: {}", error).into());
    }
    
    Ok(())
}

/// Create a backup of a file
pub fn backup_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let backup_path = format!("{}.backup", path);
    if Path::new(path).exists() {
        fs::copy(path, &backup_path)?;
    }
    Ok(backup_path)
}

/// Send SIGHUP signal to reload stunnel configuration
pub fn reload_stunnel(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    signal::kill(Pid::from_raw(pid), Signal::SIGHUP)?;
    Ok(())
}

/// Start a new stunnel process
pub fn start_stunnel(config_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let child = Command::new("stunnel")
        .arg(config_path)
        .spawn()?;
    
    Ok(child.id() as i32)
}