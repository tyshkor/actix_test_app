mod db;
mod errors;
mod models;
mod utils;
mod handlers;
mod config;


use actix_web::{web::post, web, error, App, HttpResponse, HttpServer};
use crate::config::Config;

use crate::{
    handlers::index,
    db::DB,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::from_env().expect("Server configuration");

    // we would panic in case of no connection to Db, 
    // as the app becomes unsable after this point
    let db = DB::new(config.database_url).unwrap();

    HttpServer::new(move || {

        App::new()
        .data(db.clone())
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                      error::InternalError::from_response(
                          "",
                          HttpResponse::BadRequest()
                              .content_type("application/json")
                              .body(format!(r#"{{"error":"{}"}}"#, err)),
                      )
                      .into()
        }))
        .route("/", post().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}