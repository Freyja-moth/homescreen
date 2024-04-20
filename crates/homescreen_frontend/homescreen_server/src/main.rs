use dioxus::{
    dioxus_core::VirtualDom,
    prelude::{server_fn::axum::register_explicit, DioxusRouterExt, ServeConfig},
};
use homescreen_components::prelude::*;
use homescreen_server_functions::prelude::*;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.01:8080")
        .await
        .unwrap();

    register_explicit::<GetWebsites>();

    axum::serve(
        listener,
        axum::Router::new()
            .serve_dioxus_application(ServeConfig::builder().build(), || VirtualDom::new(App))
            .await
            .into_make_service(),
    )
    .await
    .unwrap()
}
