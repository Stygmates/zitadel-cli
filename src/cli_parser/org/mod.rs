use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum OrgCommand {
    Add {
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
}
