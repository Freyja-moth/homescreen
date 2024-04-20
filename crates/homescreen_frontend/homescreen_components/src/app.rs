use crate::prelude::*;
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx!(
        Header {}
        section {
            id: "traichu",
            SearchBar {}
            QuickSites {}
        }
    )
}
