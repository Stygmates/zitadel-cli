pub(crate) mod human_user;
pub(crate) mod idp;
pub(crate) mod org;
pub(crate) mod project;

use clap::{Parser, Subcommand};
use human_user::HumanUserCommand;
use idp::IdpCommand;
use org::OrgCommand;
use project::ProjectCommand;
use tracing::{error, info};

use crate::{
    commands::{add::handle_add_entity, flows::Flow},
    logout,
    payloads::user::NewHumanUser,
    NewGoogleIdp, NewOrganization, NewProject,
};

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
    HumanUser {
        #[command(subcommand)]
        human_user_command: HumanUserCommand,
    },
    Project {
        #[command(subcommand)]
        project_command: ProjectCommand,
    },
    Org {
        #[command(subcommand)]
        org_command: OrgCommand,
    },
    Idp {
        #[command(subcommand)]
        idp_command: IdpCommand,
    },
    /// Logs the user out
    Logout {},
}

pub(crate) async fn parse_cli() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { open, flow } => {
            let login_result = flow.clone().login(*open).await;
            match login_result {
                Ok(_) => (),
                Err(error) => {
                    info! {"Failed to log in: {error}"};
                }
            }
        }
        Commands::Logout {} => match logout::logout() {
            Ok(()) => {
                info! {"Successfully logged out"}
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    info! {"Successfully logged out"}
                }
                error => {
                    error! {"An unexpected error occured: {error}"};
                }
            },
        },
        Commands::HumanUser { human_user_command } => match human_user_command {
            HumanUserCommand::Add { file_path } => {
                handle_add_entity::<NewHumanUser>(file_path, "/v2/users/human").await
            }
        },
        Commands::Project { project_command } => match project_command {
            ProjectCommand::Add { file_path } => {
                handle_add_entity::<NewProject>(file_path, "/management/v1/projects").await
            }
        },
        Commands::Org { org_command } => match org_command {
            OrgCommand::Add { file_path } => {
                handle_add_entity::<NewOrganization>(file_path, "/v2/organizations").await
            }
        },
        Commands::Idp { idp_command } => match idp_command {
            IdpCommand::Add { provider } => match provider {
                idp::IdpProvider::Google { file_path } => {
                    handle_add_entity::<NewGoogleIdp>(file_path, "/management/v1/idps/google").await
                }
            },
        },
    }
}
