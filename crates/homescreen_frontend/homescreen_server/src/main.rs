use dioxus::{
    dioxus_core::VirtualDom,
    prelude::{server_fn::axum::register_explicit, DioxusRouterExt, ServeConfig},
};
use homescreen_components::prelude::*;
use homescreen_errors::prelude::*;
use homescreen_server_functions::prelude::*;
use log::error;

async fn try_main() -> HomescreenResult {
    let listener = tokio::net::TcpListener::bind("127.0.0.01:8080")
        .await
        .map_err(|err| FrontendError::UnableToBindToPort(8080, err))?;

    register_explicit::<GetWebsites>();

    axum::serve(
        listener,
        axum::Router::new()
            .serve_dioxus_application(ServeConfig::builder().build(), || VirtualDom::new(App))
            .await
            .into_make_service(),
    )
    .await
    .map_err(FrontendError::UnableToStartFrontendServer)
    .map_err(HomescreenError::from)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(err) = try_main().await {
        error!("{err}");
    }
}
