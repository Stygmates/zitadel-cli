use cli_parser::parse_cli;
use commands::logout;
use payloads::idp::google::NewGoogleIdp;
use payloads::{organization::NewOrganization, project::NewProject};

mod cli_parser;
mod commands;
mod env;
mod error;
mod payloads;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    parse_cli().await;
    Ok(())
}
