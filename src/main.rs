mod handlers;
mod utils;
use std::num::ParseIntError;
use std::os::linux::raw::stat;
use crate::utils::api::responses::BodyBuilder;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::fmt::Debug;
#[derive(Debug)]
pub enum Error {
    EnvNotFound,
    ParseIntError
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Error::EnvNotFound
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError
    } 
}

pub type Result<T> = std::result::Result<T, Error>;
pub struct EnvironmentVars {
    database_url: String,
    shortener_url: String,
    port: String
}
fn get_env_vars() -> Result<EnvironmentVars> {
    Ok(EnvironmentVars {
        database_url: std::env::var("DATABASE_URL")?,
        shortener_url: std::env::var("SHORTENER_URL")?,
        port: std::env::var("PORT").unwrap_or("3000".to_string())
    })
}

pub struct AppState {
    shortener_url: String,
    db_client: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_vars = match get_env_vars() {
        Ok(vars) => vars,
        Err(_) => {
            dotenv::dotenv().expect("Dotenv failed to initialize.");
            get_env_vars().expect("Say whaat")
        }

    };

    let state = Data::new(AppState {
        shortener_url: env_vars.shortener_url,
        db_client: PgPoolOptions::new().max_connections(10).connect(&env_vars.database_url).await.unwrap()
    });

    sqlx::migrate!("./migrations").run(&state.db_client).await.expect("Could not run migrations.");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(handlers::configure)
            .service(health)
    })
        .bind(format!("0.0.0.0:{}", env_vars.port))?
        .run()
        .await
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(BodyBuilder::new("Healthy"))
}
