use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProjectCommand {
    Add {
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
}
