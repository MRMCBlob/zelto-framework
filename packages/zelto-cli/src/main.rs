use anyhow::Result;
use clap::{Parser, Subcommand};

mod cmd_new;
mod cmd_dev;
mod cmd_build;

#[derive(Parser)]
#[command(name = "zelto", about = "Zelto — native Windows app framework", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new Zelto application
    New {
        /// Project name
        name: String,
    },
    /// Start development server with hot reload
    Dev {
        /// Path to the app (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Build a release binary
    Build {
        /// Path to the app (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Target triple
        #[arg(long, default_value = "x86_64-pc-windows-msvc")]
        target: String,
        /// Enable optimizations
        #[arg(long)]
        release: bool,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => cmd_new::run(&name),
        Commands::Dev { path } => cmd_dev::run(&path),
        Commands::Build { path, target, release } => cmd_build::run(&path, &target, release),
    }
}
