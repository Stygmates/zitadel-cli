use clap::Parser;
use cli_parser::{Cli, Commands};
use commands::add::add_resource;
use commands::login::load_access_token;
use commands::logout;
use payloads::idp::google::NewGoogleIdp;
use payloads::user::NewHumanUser;
use payloads::{organization::NewOrganization, project::NewProject};
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
        Commands::Add { ressource } => {
            let token = load_access_token();
            match token {
                Ok(token) => {
                    let api_call = match ressource {
                        cli_parser::Ressource::Org { file_path } => {
                            add_resource::<NewOrganization>(token, "/v2/organizations", &file_path)
                                .await
                        }
                        cli_parser::Ressource::HumanUser { file_path } => {
                            add_resource::<NewHumanUser>(token, "/v2/users/human", file_path).await
                        }
                        cli_parser::Ressource::Project { file_path } => {
                            add_resource::<NewProject>(token, "/management/v1/projects", file_path)
                                .await
                        }
                        cli_parser::Ressource::Idp { idp } => match idp {
                            cli_parser::IdpCommand::Google { file_path } => {
                                add_resource::<NewGoogleIdp>(
                                    token,
                                    "/management/v1/idps/google",
                                    file_path,
                                )
                                .await
                            }
                        },
                    };

                    match api_call {
                        Ok(_) => {
                            info! {"Ressource created successfully"}
                        }
                        Err(error) => {
                            error! {"Failed to create ressource: {error}"}
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
