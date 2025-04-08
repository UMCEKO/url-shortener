use crate::utils::api::responses::BodyBuilder;
use crate::AppState;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn get_url(req: HttpRequest, state: Data<AppState>) -> HttpResponse {
    #[derive(Serialize, Deserialize)]
    struct ShortenerResponse {
        target_url: String
    }

    let url = req.path()[1..].to_string();
    let result = sqlx::query_as!(ShortenerResponse, r#"SELECT target_url FROM links WHERE shortened_url = $1"#, url).fetch_optional(&state.db_client).await;
    let target_url = match result {
        Ok(Some(res)) => res.target_url,
        Ok(None) => {
            return HttpResponse::NotFound().finish()
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return HttpResponse::InternalServerError().json(BodyBuilder::new("There was an error getting the page."))
        }
    };
    HttpResponse::MovedPermanently().insert_header(("location", target_url)).finish()
}