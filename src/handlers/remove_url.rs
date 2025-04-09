use actix_web::{delete, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use crate::AppState;
use crate::utils::api::responses::BodyBuilder;

#[derive(Serialize, Deserialize)]
struct RemoveUrlRequestBody {
    url: String
}

#[delete("/remove-url")]
pub async fn remove_url(state: Data<AppState>, Json(body): Json<RemoveUrlRequestBody>) -> HttpResponse {
    match sqlx::query!(r#"DELETE FROM links WHERE shortened_url = $1"#, body.url).execute(&state.db_client).await {
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().json(BodyBuilder::new("There was an error deleting the url. Please try again later."))
        },
        Ok(result) if result.rows_affected() == 1 => {
            HttpResponse::Ok().json(BodyBuilder::new("Successfully deleted the url."))
        }
        Ok(_) => {
            HttpResponse::NotFound().json(BodyBuilder::new("The url has not been found."))
        }
    }
}