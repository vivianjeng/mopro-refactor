use clap::{Parser, Subcommand};
use std::process::Command;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "mopro")]
#[command(about = "Mopro FFI binding generator CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build FFI bindings for a specific platform
    Build {
        /// Target platform (ios, android, web, flutter, react-native)
        #[arg(value_enum)]
        platform: Platform,
        
        /// Output directory for generated bindings
        #[arg(short, long, default_value = "bindings")]
        output: String,
    },
    
    /// List available platforms
    Platforms,
    
    /// Run tests
    Test {
        /// Run end-to-end tests
        #[arg(long)]
        e2e: bool,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum Platform {
    Ios,
    Android,
    Web,
    Flutter,
    ReactNative,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Ios => write!(f, "ios"),
            Platform::Android => write!(f, "android"),
            Platform::Web => write!(f, "web"),
            Platform::Flutter => write!(f, "flutter"),
            Platform::ReactNative => write!(f, "react-native"),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { platform, output } => {
            build_bindings(platform, &output).await?;
        }
        Commands::Platforms => {
            list_platforms();
        }
        Commands::Test { e2e } => {
            if e2e {
                run_e2e_tests().await?;
            } else {
                run_unit_tests().await?;
            }
        }
    }
    
    Ok(())
}

async fn build_bindings(platform: Platform, _output: &str) -> anyhow::Result<()> {
    info!("Building bindings for platform: {}", platform);
    
    // Change to mopro-ffi directory
    std::env::set_current_dir("mopro-ffi")?;
    
    let status = Command::new("cargo")
        .args(["run", "--bin", &platform.to_string()])
        .status()?;
    
    if status.success() {
        info!("✅ Successfully built bindings for {}", platform);
    } else {
        error!("❌ Failed to build bindings for {}", platform);
        std::process::exit(1);
    }
    
    Ok(())
}

fn list_platforms() {
    println!("Available platforms:");
    println!("  ios          - Generate Swift bindings for iOS");
    println!("  android      - Generate Kotlin bindings for Android");
    println!("  web          - Generate JavaScript bindings for Web/WASM");
    println!("  flutter      - Generate Dart bindings for Flutter");
    println!("  react-native - Generate JavaScript bindings for React Native");
    println!();
    println!("Usage: mopro build <platform>");
}

async fn run_unit_tests() -> anyhow::Result<()> {
    info!("Running unit tests...");
    
    let status = Command::new("cargo")
        .args(["test"])
        .status()?;
    
    if status.success() {
        info!("✅ All unit tests passed");
    } else {
        error!("❌ Some unit tests failed");
        std::process::exit(1);
    }
    
    Ok(())
}

async fn run_e2e_tests() -> anyhow::Result<()> {
    info!("Running end-to-end tests...");
    
    // Change to test-e2e directory
    std::env::set_current_dir("test-e2e")?;
    
    let status = Command::new("cargo")
        .args(["test"])
        .status()?;
    
    if status.success() {
        info!("✅ All end-to-end tests passed");
    } else {
        error!("❌ Some end-to-end tests failed");
        std::process::exit(1);
    }
    
    Ok(())
}
