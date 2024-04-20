use homescreen_components::prelude::*;
use tracing::level_filters::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::OFF.into_level().unwrap()).expect("Cannot initialize logger");
    dioxus::launch(App);
}
