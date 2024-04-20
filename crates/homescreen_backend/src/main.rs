use homescreen_backend::try_main;
use log::error;

#[actix_web::main]
async fn main() {
    env_logger::init();

    if let Err(err) = try_main().await {
        error!("{err}");
    }
}
