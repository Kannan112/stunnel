use tonic::transport::Server;
use stunnel_space::{Config, StunnelServer};
use stunnel_space::stunnel::stunnel_manager_server::StunnelManagerServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists (optional)
    dotenv::dotenv().ok();
    
    // Load configuration from environment with error handling
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration Error: {}", e);
            eprintln!("\nPlease set the required environment variables:");
            eprintln!("  export STUNNEL_CONFIG=/path/to/stunnel.conf");
            eprintln!("  export STUNNEL_PID_FILE=/path/to/stunnel.pid");
            eprintln!("  export GRPC_PORT=50055");
            eprintln!("\nOr use: source export.sh");
            eprintln!("Or create a .env file from .env.example");
            std::process::exit(1);
        }
    };
    
    // Print configuration
    config.print_config();
    
    // Parse gRPC address
    let addr = config.get_grpc_address().parse()?;
    
    // Create stunnel server with config values
    let stunnel_server = StunnelServer::new(
        config.config_path.clone(), 
        config.pid_file.clone()
    );

    println!("\nStarting gRPC server on {}", addr);

    // Start the gRPC server
    Server::builder()
        .add_service(StunnelManagerServer::new(stunnel_server))
        .serve(addr)
        .await?;

    Ok(())
}