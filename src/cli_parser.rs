//! This module contains the CLI parser and the commands that can be executed by the CLI

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::login::flows::Flow;

#[derive(Subcommand)]
/// Add an identity provider
pub enum IdpCommand {
    Google {
        /// The path of the file describing a google IdP, see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-google-provider)
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// The Zitadel CLI is a command line interface to interact with the Zitadel API to manage different resources
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
/// The commands that can be executed by the Zitadel CLI
pub enum Commands {
    /// Logs the user in
    Login {
        /// Opens the browser automatically
        #[arg(short, long)]
        open: bool,
        #[arg(short, long, value_enum, default_value = "client-credentials")]
        flow: Flow,
    },
    /// Adds an organization
    AddOrg {
        /// The path of the file describing an org, see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization) for more details
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
    /// Adds a human user
    AddHumanUser {
        /// The path of the file describing a human user, see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-add-human-user) for more details
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },
    /// Adds a project
    AddProject {
        /// The path of the file describing a human user, see [the Zitadel documentation](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project)
        #[arg(short, long, required(true))]
        file_path: PathBuf,
    },

    AddIdp {
        #[command(subcommand)]
        command: IdpCommand,
    },
    /// Logs the user out
    Logout {},
}
