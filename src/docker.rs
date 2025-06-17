use anyhow::{Context, Result};
use colored::*;
use docker_api::{
    opts::{ContainerCreateOpts, ContainerStopOpts, LogsOpts, PullOpts},
    Docker,
};
use futures::StreamExt;

const RIPPLED_IMAGE: &str = "legleux/rippled_smart_escrow:bb9bb5f5";
const CONTAINER_NAME: &str = "craft-rippled";

pub struct DockerManager {
    docker: Docker,
}

impl DockerManager {
    pub fn new() -> Result<Self> {
        // Try to connect to Docker - check multiple possible socket locations
        let docker = if cfg!(target_os = "macos") {
            // On macOS, try multiple possible socket locations
            let mut possible_sockets = vec![
                "/var/run/docker.sock".to_string(),
            ];
            
            // Also check HOME-based paths
            if let Ok(home) = std::env::var("HOME") {
                possible_sockets.push(format!("{}/.colima/default/docker.sock", home));
                possible_sockets.push(format!("{}/.colima/docker.sock", home));
                possible_sockets.push(format!("{}/.docker/run/docker.sock", home));
            }
            
            
            // Find first existing socket
            for socket in &possible_sockets {
                if std::path::Path::new(socket).exists() {
                    return Ok(Self { docker: Docker::unix(socket) });
                }
            }
            
            // Default fallback
            Docker::unix("/var/run/docker.sock")
        } else {
            Docker::unix("/var/run/docker.sock")
        };
        
        Ok(Self { docker })
    }
    
    fn diagnose_docker_issue(&self) {
        use std::process::Command;
        
        println!("{}", "\nDiagnosing Docker issue...".yellow());
        
        // Check Docker context
        if let Ok(output) = Command::new("docker")
            .args(&["context", "ls"])
            .output()
        {
            if let Ok(contexts) = String::from_utf8(output.stdout) {
                for line in contexts.lines() {
                    if line.contains("*") {
                        println!("{}", format!("Active Docker context: {}", line).cyan());
                        if line.contains("colima") && line.contains("/Users") {
                            println!("{}", "Colima context is active but connection failed.".yellow());
                        }
                    }
                }
            }
        }
        
        // Check which sockets exist
        let mut sockets_to_check = vec![
            "/var/run/docker.sock".to_string(),
        ];
        
        if let Ok(home) = std::env::var("HOME") {
            sockets_to_check.push(format!("{}/.colima/default/docker.sock", home));
            sockets_to_check.push(format!("{}/.colima/docker.sock", home));
            sockets_to_check.push(format!("{}/.docker/run/docker.sock", home));
        }
        
        println!("{}", "Checking Docker socket locations:".cyan());
        for socket in &sockets_to_check {
            if std::path::Path::new(socket).exists() {
                println!("{}", format!("  ✓ Found: {}", socket).green());
            } else {
                println!("{}", format!("  ✗ Missing: {}", socket).red());
            }
        }
    }
    
    async fn check_docker_connection(&self) -> Result<()> {
        // Try to ping Docker to ensure it's running
        match self.docker.ping().await {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.to_string().contains("No such file or directory") || e.to_string().contains("Connection refused") {
                    // Check if this is macOS and offer to install Colima
                    if cfg!(target_os = "macos") {
                        // First, let's diagnose the issue
                        self.diagnose_docker_issue();
                        self.handle_macos_docker_missing().await
                    } else {
                        Err(anyhow::anyhow!(
                            "Docker is not running or not accessible.\n\n\
                            Please ensure Docker is installed and running:\n\
                            - On Linux: Run 'sudo systemctl start docker'\n\
                            - On Windows: Start Docker Desktop\n\n\
                            You can download Docker from: https://www.docker.com/products/docker-desktop"
                        ))
                    }
                } else {
                    Err(anyhow::anyhow!("Failed to connect to Docker: {}", e))
                }
            }
        }
    }
    
    async fn handle_macos_docker_missing(&self) -> Result<()> {
        use colored::*;
        use inquire::Confirm;
        use std::process::Command;
        
        println!("{}", "Docker is not installed or not running.".yellow());
        
        // Check if Homebrew is installed
        let brew_check = Command::new("which")
            .arg("brew")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
            
        if !brew_check {
            return Err(anyhow::anyhow!(
                "Docker is not installed. Please install Docker manually:\n\n\
                Option 1: Docker Desktop\n\
                - Download from: https://www.docker.com/products/docker-desktop/\n\n\
                Option 2: Install Homebrew first, then Colima\n\
                - Install Homebrew: /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"\n\
                - Then run: brew install colima docker && colima start"
            ));
        }
        
        // Check if Colima is already installed
        let colima_installed = Command::new("which")
            .arg("colima")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
            
        if colima_installed {
            // Check if Colima is actually running
            let colima_status = Command::new("colima")
                .arg("status")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
                
            let is_running = colima_status.contains("Running");
            
            if is_running {
                println!("{}", "Colima appears to be running but Docker is not accessible.".yellow());
                println!("{}", "This might be a socket permission issue.".yellow());
                
                // Try to restart Colima
                if Confirm::new("Would you like to restart Colima to fix this?")
                    .with_default(true)
                    .prompt()?
                {
                    println!("{}", "Stopping Colima...".cyan());
                    let _ = Command::new("colima")
                        .arg("stop")
                        .status();
                    
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    println!("{}", "Starting Colima...".cyan());
                    let status = Command::new("colima")
                        .arg("start")
                        .status()?;
                        
                    if status.success() {
                        println!("{}", "Colima restarted successfully!".green());
                        
                        // Wait for Docker to be ready with retries
                        println!("{}", "Waiting for Docker to be ready...".cyan());
                        for i in 0..10 {
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            
                            // Try to connect to Docker
                            if let Ok(_) = self.docker.ping().await {
                                println!("{}", "Docker is ready!".green());
                                return Ok(());
                            }
                            
                            if i < 9 {
                                print!(".");
                                use std::io::{self, Write};
                                io::stdout().flush().unwrap();
                            }
                        }
                        
                        println!();
                        return Err(anyhow::anyhow!("Docker failed to start properly after restart. Please check Colima logs with 'colima status -v'"));
                    } else {
                        return Err(anyhow::anyhow!("Failed to restart Colima"));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Docker is not accessible. Try running:\n\
                        colima stop && colima start"
                    ));
                }
            } else {
                println!("{}", "Colima is installed but not running.".cyan());
                
                if Confirm::new("Would you like to start Colima now?")
                    .with_default(true)
                    .prompt()?
                {
                    println!("{}", "Starting Colima...".cyan());
                    let status = Command::new("colima")
                        .arg("start")
                        .status()?;
                    
                if status.success() {
                    println!("{}", "Colima started successfully!".green());
                    
                    // Wait for Docker to be ready with retries
                    println!("{}", "Waiting for Docker to be ready...".cyan());
                    for i in 0..10 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        
                        // Try to connect to Docker
                        if let Ok(_) = self.docker.ping().await {
                            println!("{}", "Docker is ready!".green());
                            return Ok(());
                        }
                        
                        if i < 9 {
                            print!(".");
                            use std::io::{self, Write};
                            io::stdout().flush().unwrap();
                        }
                    }
                    
                    println!();
                    return Err(anyhow::anyhow!("Docker failed to start properly. Please try running 'colima start' manually."));
                } else {
                    return Err(anyhow::anyhow!("Failed to start Colima"));
                }
                }
            }
        } else {
            // Offer to install Colima
            println!("{}", "\nColima is a lightweight Docker runtime for macOS.".cyan());
            println!("{}", "It is free and uses less resources than Docker Desktop.".cyan());
            
            if Confirm::new("Would you like to install Colima (lightweight Docker)?")
                .with_default(true)
                .prompt()?
            {
                println!("{}", "Installing Colima and Docker CLI...".cyan());
                
                // Install Colima and Docker CLI
                let install_status = Command::new("brew")
                    .args(&["install", "colima", "docker"])
                    .status()?;
                    
                if !install_status.success() {
                    return Err(anyhow::anyhow!("Failed to install Colima"));
                }
                
                println!("{}", "Colima installed successfully!".green());
                println!("{}", "Starting Colima for the first time...".cyan());
                
                // Start Colima
                let start_status = Command::new("colima")
                    .arg("start")
                    .status()?;
                    
                if start_status.success() {
                    println!("{}", "Colima started successfully!".green());
                    println!("{}", "\nNote: Colima needs to be started after each reboot.".yellow());
                    println!("{}", "You can start it with: colima start".blue());
                    
                    // Wait for Docker to be ready with retries
                    println!("{}", "\nWaiting for Docker to be ready...".cyan());
                    for i in 0..10 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        
                        // Try to connect to Docker
                        if let Ok(_) = self.docker.ping().await {
                            println!("{}", "Docker is ready!".green());
                            return Ok(());
                        }
                        
                        if i < 9 {
                            print!(".");
                            use std::io::{self, Write};
                            io::stdout().flush().unwrap();
                        }
                    }
                    
                    println!();
                    return Err(anyhow::anyhow!("Docker failed to start properly. Please try running 'colima start' manually."));
                } else {
                    return Err(anyhow::anyhow!("Failed to start Colima"));
                }
            }
        }
        
        Err(anyhow::anyhow!(
            "Docker is required to run rippled. Please install one of:\n\
            - Colima: brew install colima docker && colima start\n\
            - Docker Desktop: https://www.docker.com/products/docker-desktop/"
        ))
    }

    pub async fn ensure_image_exists(&self) -> Result<()> {
        println!("{}", "Checking for rippled Docker image...".cyan());

        // Parse image name and tag
        let (image_name, tag) = if let Some(colon_pos) = RIPPLED_IMAGE.rfind(':') {
            let name = &RIPPLED_IMAGE[..colon_pos];
            let tag = &RIPPLED_IMAGE[colon_pos + 1..];
            (name, tag)
        } else {
            (RIPPLED_IMAGE, "latest")
        };
        
        let images = self.docker.images();
        let image_exists = match images.get(RIPPLED_IMAGE).inspect().await {
            Ok(_) => true,
            Err(e) => {
                // If error contains "404" or "not found", image doesn't exist
                if e.to_string().contains("404") || e.to_string().to_lowercase().contains("not found") {
                    false
                } else {
                    // For other errors, try the list approach as fallback
                    match images.list(&Default::default()).await {
                        Ok(image_list) => {
                            image_list.iter().any(|img| {
                                img.repo_tags.iter().any(|tag| tag == RIPPLED_IMAGE)
                            })
                        }
                        Err(_) => false,
                    }
                }
            }
        };

        if !image_exists {
            println!(
                "{}",
                format!("Pulling Docker image: {}", RIPPLED_IMAGE).yellow()
            );
            
            let pull_opts = PullOpts::builder()
                .image(image_name)
                .tag(tag)
                .build();

            let mut stream = images.pull(&pull_opts);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(_) => {
                        // Progress information is already displayed by docker-api
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("Failed to pull image: {}", e));
                    }
                }
            }
            println!("\n{}", "Image pulled successfully!".green());
        } else {
            println!("{}", "Docker image already exists.".green());
        }

        Ok(())
    }

    pub async fn start_rippled(&self, foreground: bool) -> Result<()> {
        // Check Docker connection first
        self.check_docker_connection().await?;
        
        self.ensure_image_exists().await?;

        // Check if container already exists
        let containers = self.docker.containers();
        let existing = containers.list(&Default::default()).await?;

        let container_exists = existing.iter().any(|c| {
            c.names
                .as_ref()
                .map(|names| {
                    names
                        .iter()
                        .any(|name| name.trim_start_matches('/') == CONTAINER_NAME)
                })
                .unwrap_or(false)
        });

        if container_exists {
            println!(
                "{}",
                "Container already exists. Checking if it's running...".cyan()
            );
            let container = containers.get(CONTAINER_NAME);
            let info = container.inspect().await?;

            if info.state.as_ref().and_then(|s| s.running).unwrap_or(false) {
                println!("{}", "rippled container is already running!".green());
                if foreground {
                    println!(
                        "{}",
                        "To see logs, use: docker logs -f craft-rippled".blue()
                    );
                }
                return Ok(());
            } else {
                println!("{}", "Starting existing container...".yellow());
                container.start().await?;
                println!("{}", "Container started successfully!".green());
                return Ok(());
            }
        }

        // Create and start new container
        println!("{}", "Creating new rippled container...".cyan());

        // Get the absolute paths for config files
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        let config_path = current_dir.join("reference/rippled-cfg/smart-escrow-rippled.cfg");
        let validators_path = current_dir.join("reference/rippled-cfg/validators.txt");
        
        // Check if config files exist
        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "rippled config file not found at: {}",
                config_path.display()
            ));
        }
        if !validators_path.exists() {
            return Err(anyhow::anyhow!(
                "validators file not found at: {}",
                validators_path.display()
            ));
        }
        
        // Create container with port mappings and volume mounts
        // Port mapping based on config:
        // - 80 (container) -> 6006 (host) for public WebSocket
        // - 6006 (container) -> 6006 (host) for admin WebSocket
        // - 5005 (container) -> 5005 (host) for admin RPC
        // -v /path/to/config:/etc/opt/ripple/rippled.cfg:ro
        // -v /path/to/validators:/etc/opt/ripple/validators.txt:ro
        let create_opts = ContainerCreateOpts::builder()
            .name(CONTAINER_NAME)
            .image(RIPPLED_IMAGE)
            .expose(docker_api::opts::PublishPort::tcp(80), 6006)     // Public WS on host:6006
            .expose(docker_api::opts::PublishPort::tcp(6006), 6007)   // Admin WS on host:6007
            .expose(docker_api::opts::PublishPort::tcp(5005), 5005)   // Admin RPC
            .volumes(vec![
                format!("{}:/etc/opt/ripple/rippled.cfg:ro", config_path.to_string_lossy()),
                format!("{}:/etc/opt/ripple/validators.txt:ro", validators_path.to_string_lossy()),
            ])
            // Override entrypoint to ensure rippled runs with correct arguments
            .entrypoint(vec!["/opt/ripple/bin/rippled"])
            .command(vec![
                "-a",      // Stand-alone mode flag
                "--start", // Start from a fresh ledger
                "--conf=/etc/opt/ripple/rippled.cfg"
            ])
            .build();

        let container = containers.create(&create_opts).await?;

        println!("{}", "Starting container...".cyan());
        container.start().await?;

        println!("{}", "rippled container started successfully in stand-alone mode!".green());
        println!("{}", "Public WebSocket: ws://localhost:6006".blue());
        println!("{}", "Admin WebSocket: ws://localhost:6007".blue());
        println!("{}", "Admin RPC API: http://localhost:5005".blue());
        println!("{}", "\nNote: Running in stand-alone mode (no peers, local ledger only)".yellow());

        if foreground {
            println!(
                "{}",
                "\nShowing container logs (Ctrl+C to stop)...".yellow()
            );
            let log_opts = LogsOpts::builder()
                .stdout(true)
                .stderr(true)
                .follow(true)
                .build();

            let mut logs = container.logs(&log_opts);
            while let Some(log) = logs.next().await {
                match log {
                    Ok(chunk) => print!("{}", String::from_utf8_lossy(&chunk)),
                    Err(e) => eprintln!("Error reading logs: {}", e),
                }
            }
        } else {
            println!("{}", "\nTo view logs: docker logs -f craft-rippled".blue());
            println!("{}", "To stop: craft stop-rippled".blue());
        }

        Ok(())
    }

    pub async fn stop_rippled(&self) -> Result<()> {
        // Check Docker connection first
        self.check_docker_connection().await?;
        
        println!("{}", "Stopping rippled container...".cyan());

        let containers = self.docker.containers();
        let container = containers.get(CONTAINER_NAME);

        let stop_opts = ContainerStopOpts::builder().build();
        match container.stop(&stop_opts).await {
            Ok(_) => {
                println!("{}", "Container stopped successfully!".green());
                Ok(())
            }
            Err(e) => {
                if e.to_string().contains("No such container") {
                    println!("{}", "No running container found.".yellow());
                    Ok(())
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn list_containers(&self) -> Result<()> {
        // Check Docker connection first
        self.check_docker_connection().await?;
        
        println!("{}", "Checking for rippled containers...".blue());

        let containers = self.docker.containers();
        let list = containers.list(&Default::default()).await?;

        let rippled_containers: Vec<_> = list
            .iter()
            .filter(|c| {
                c.image
                    .as_ref()
                    .map(|img| img.contains("rippled"))
                    .unwrap_or(false)
            })
            .collect();

        if rippled_containers.is_empty() {
            println!("{}", "No rippled containers found.".yellow());
        } else {
            println!(
                "{}",
                format!("Found {} rippled container(s):", rippled_containers.len()).green()
            );

            for container in rippled_containers {
                let name = container
                    .names
                    .as_ref()
                    .and_then(|names| names.first())
                    .map(|n| n.trim_start_matches('/'))
                    .unwrap_or("unnamed");
                let state = container.state.as_deref().unwrap_or("unknown");
                let status = container.status.as_deref().unwrap_or("unknown");

                println!("  - {} ({}): {}", name, state, status);
            }

            println!("\n{}", "Container management commands:".blue());
            println!("  View logs: {}", "docker logs -f craft-rippled".green());
            println!("  Stop container: {}", "craft stop-rippled".green());
            println!("  Remove container: {}", "docker rm craft-rippled".green());
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn check_server_info(&self) -> Result<bool> {
        use std::time::Duration;
        use tokio::time::timeout;

        // Check if container is running first
        let containers = self.docker.containers();
        let container = containers.get(CONTAINER_NAME);

        match container.inspect().await {
            Ok(info) => {
                if !info.state.as_ref().and_then(|s| s.running).unwrap_or(false) {
                    return Ok(false);
                }
            }
            Err(_) => return Ok(false),
        }

        // Try to connect to the JSON-RPC API
        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "method": "server_info",
            "params": [{}]
        });

        match timeout(
            Duration::from_secs(5),
            client
                .post("http://localhost:6006")
                .json(&request_body)
                .send(),
        )
        .await
        {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        // Check if there's an error in the response
                        if json.get("result").and_then(|r| r.get("error")).is_some() {
                            Ok(false)
                        } else {
                            Ok(true)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
    
    pub async fn advance_ledger(&self, count: u32) -> Result<()> {
        println!("{}", format!("Advancing ledger {} time(s)...", count).cyan());
        
        // First check if the container is running
        let containers = self.docker.containers();
        let container = containers.get(CONTAINER_NAME);
        
        match container.inspect().await {
            Ok(info) => {
                if !info.state.as_ref().and_then(|s| s.running).unwrap_or(false) {
                    return Err(anyhow::anyhow!("rippled container is not running. Start it with: craft start-rippled"));
                }
            }
            Err(_) => {
                return Err(anyhow::anyhow!("rippled container not found. Start it with: craft start-rippled"));
            }
        }
        
        // Use the admin RPC endpoint to send ledger_accept commands
        let client = reqwest::Client::new();
        
        for i in 1..=count {
            let request_body = serde_json::json!({
                "method": "ledger_accept",
                "params": [{}]
            });
            
            match client
                .post("http://localhost:5005")
                .json(&request_body)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(json) = response.json::<serde_json::Value>().await {
                            if let Some(error) = json.get("result").and_then(|r| r.get("error")) {
                                return Err(anyhow::anyhow!(
                                    "Failed to advance ledger: {}",
                                    error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error")
                                ));
                            }
                            
                            // Extract ledger info if available
                            if let Some(ledger_hash) = json.get("result").and_then(|r| r.get("ledger_hash")) {
                                println!(
                                    "{}", 
                                    format!("  [{}/{}] Advanced to ledger with hash: {}", 
                                        i, count, 
                                        ledger_hash.as_str().unwrap_or("unknown")
                                    ).green()
                                );
                            } else {
                                println!("{}", format!("  [{}/{}] Ledger advanced", i, count).green());
                            }
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "Failed to advance ledger: HTTP {}",
                            response.status()
                        ));
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to connect to rippled admin API at localhost:5005: {}",
                        e
                    ));
                }
            }
            
            // Small delay between advances if advancing multiple ledgers
            if i < count {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        
        println!("{}", "\nLedger(s) advanced successfully!".green());
        println!("{}", "Note: In stand-alone mode, ledgers only advance when explicitly commanded.".yellow());
        
        Ok(())
    }
}
