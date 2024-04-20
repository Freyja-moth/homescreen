use dioxus::prelude::*;

pub fn SearchBar() -> Element {
    rsx!(
        section { id: "search",
            h2 { class: "sr-only", "Search" }
            form {
                autocomplete: "off",
                action: "https://duckduckgo.com/",
                method: "get",
                input { autofocus: "false", name: "q", r#type: "text", id: "q" }
                button { tabindex: "-1", class: "sr-only", "Search" }
            }
        }
    )
}
