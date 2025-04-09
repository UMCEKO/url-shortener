mod get_url;
mod add_url;
mod remove_url;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(add_url::add_url)
        .service(remove_url::remove_url)
        .service(actix_web::web::resource("/{tail:.*}").get(get_url::get_url));
}

