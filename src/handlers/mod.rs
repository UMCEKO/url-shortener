mod get_url;
mod add_url;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(add_url::add_url)
        .service(actix_web::web::resource("/{tail:.*}").to(get_url::get_url));
}

