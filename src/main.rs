#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-compose-multiplatform", about = "Compose Multiplatform declarative UI runtime for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-compose-multiplatform: no entries configured."),
        Commands::Status => println!("sigmaos-compose-multiplatform: ready."),
    }
}
