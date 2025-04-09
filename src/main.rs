mod handlers;
mod utils;

use std::os::linux::raw::stat;
use crate::utils::api::responses::BodyBuilder;
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct AppState {
    shortener_url: String,
    db_client: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Dotenv failed to initialize.");
    let db_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL environment variable to be set.");
    let state = Data::new(AppState {
        shortener_url: std::env::var("SHORTENER_URL").unwrap(),
        db_client: PgPoolOptions::new().max_connections(10).connect(&db_url).await.unwrap()
    });

    sqlx::migrate!("./migrations").run(&state.db_client).await.expect("Could not run migrations.");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(handlers::configure)
            .service(health)
    })
        .bind(format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or("3000".to_string())))?
        .run()
        .await
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(BodyBuilder::new("Healthy"))
}