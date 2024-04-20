pub mod config;
pub mod prelude;
pub mod routes;

use crate::prelude::*;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use homescreen_errors::prelude::*;
use log::info;

pub async fn try_main() -> HomescreenResult {
    let config = Config::load()?;
    let database = config.connect_to_database().await?;

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .service(websites::get_websites)
            .service(websites::get_fun_website)
            .service(websites::get_coding_websites)
            .service(websites::get_editing_websites)
            .service(websites::create_or_update_website)
            .service(websites::delete_website)
            .wrap(cors)
            .app_data(Data::clone(&database))
    })
    .bind(("127.0.0.1", config.port()))
    .map_err(|err| StartupError::CannotBindToPort(err, config.port()))
    .inspect(|_| info!("Starting server on port {}", config.port()))?
    .run()
    .await
    .map_err(StartupError::CannotStartServer)?;

    Ok(())
}
