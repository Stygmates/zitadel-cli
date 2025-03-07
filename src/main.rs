use clap::Parser;
use cli_parser::{Cli, Commands};
use commands::add::add_ressource;
use commands::login::load_access_token;
use commands::logout;
use payloads::organization::NewOrganization;
use payloads::user::NewHumanUser;
use tracing::{error, info};

mod cli_parser;
mod commands;
mod env;
mod error;
mod payloads;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { open, flow } => {
            let login_result = flow.clone().login(*open).await;
            match login_result {
                Ok(_) => (),
                Err(error) => {
                    log::info! {"Failed to log in: {error}"};
                }
            }
        }
        Commands::Logout {} => match logout::logout() {
            Ok(()) => {
                log::info! {"Successfully logged out"}
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
        Commands::AddOrg { file_path } => {
            let token = load_access_token();
            match token {
                Ok(token) => {
                    match add_ressource::<NewOrganization>(token, "/v2/organizations", file_path)
                        .await
                    {
                        Ok(()) => {
                            info! {"Organization created successfully"};
                        }
                        Err(error) => {
                            error! {"Failed to create organization: {error}"};
                        }
                    }
                }
                Err(error) => {
                    error! {"Please try to log in again: {error}"};
                }
            }
        }
        Commands::AddHumanUser { file_path } => {
            let token = load_access_token();
            match token {
                Ok(token) => {
                    match add_ressource::<NewHumanUser>(token, "/v2/users/human", file_path).await {
                        Ok(()) => {
                            info! {"Human user created successfully"};
                        }
                        Err(error) => {
                            error! {"Failed to create human user: {error}"};
                        }
                    }
                }
                Err(error) => {
                    error! {"Please try to log in again: {error}"};
                }
            }
        }
    }
    Ok(())
}
