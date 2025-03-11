use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum IdpProvider {
    Google {
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum IdpCommand {
    Add {
        #[command(subcommand)]
        provider: IdpProvider,
    },
}
