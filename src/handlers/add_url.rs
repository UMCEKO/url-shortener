use crate::utils::api::responses::BodyBuilder;
use crate::utils::rand::{generate_random_string};
use crate::AppState;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct NewUrlBody {
    link: String,
    shortened_url: Option<String>
}

#[post("/new-url")]
async fn add_url(Json(body): Json<NewUrlBody>, state: Data<AppState>) -> HttpResponse {
    let shortened_url = body.shortened_url.unwrap_or(String::from(generate_random_string(5)));
    let result = sqlx::query!(r#"INSERT INTO links(target_url, shortened_url) VALUES ($1, $2)"#, body.link, shortened_url).fetch_all(&state.db_client).await;
    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error occured: {:?}", e);
            return HttpResponse::Forbidden().json(BodyBuilder::new("Link already exists."))
        }
    }
    HttpResponse::Ok().json(BodyBuilder::new("Successfully generated the shortened url.").add_data(json!({
        "url": format!("{}/{}", state.shortener_url, shortened_url)
    })))
}