use std::fs;
use std::path::PathBuf;
use clap::{Parser as ClapParser, Subcommand};
use dadagen_core::*;

#[derive(Debug, ClapParser)]
#[command(name = "dadagen")]
#[command(about = "Random Data Generator", long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate data from a dadagen configuration file
    Generate {
        /// Path to the dadagen configuration file
        dadagen_config: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();
    
    let args = Cli::parse();

    match args.cmd {
        Command::Generate { dadagen_config } => {
            println!("Generating data from config: {:?}", dadagen_config);
            let content = fs::read_to_string(dadagen_config)?;
            
            // TODO: Use proper DSL engine once implemented
            println!("Configuration content:\n{}", content);
        }
    }

    Ok(())
}
