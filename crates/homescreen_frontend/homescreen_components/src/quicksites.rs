use dioxus::prelude::*;
use homescreen_data::prelude::*;
use homescreen_server_functions::prelude::get_websites;

pub fn QuickSite(website: &Website) -> Element {
    rsx!(
        li {
            a {
                href: format!("https://{}", website.link()),
                img {
                    class: "favicon",
                    src: website.icon_link(),
                    alt: website.name()
                }
                { website.name() }
            },
        }
    )
}

pub fn QuickSiteColumn(websites: &[Website], section: &WebsiteSection) -> Element {
    rsx!(ul {
        id: format!("{}-quicksites", section.to_string().to_lowercase()),
        p {
            id: "sections",
            { section.to_string() }
        }
        { websites.iter().map(QuickSite) }
    })
}

pub fn QuickSiteColumns() -> Element {
    let response = use_server_future(get_websites)?;

    response
        .read_unchecked()
        .as_ref()
        .map(|result| match result {
            Ok(websites) => rsx!({
                WebsiteSection::ALL
                    .iter()
                    .filter_map(|website_section| {
                        websites.get(website_section).zip(Some(website_section))
                    })
                    .map(|(website, section)| QuickSiteColumn(website, section))
            }),
            Err(err) => {
                log::error!("{err}");
                rsx!(
                    h3 {
                        id: "website-error",
                        { format!("{err}") }
                    }
                )
            }
        })
        .or(rsx!(
            h2 {
                "Loading websites"
            }
        )
        .into())
        .flatten()
}

pub fn QuickSites() -> Element {
    rsx!(
        section {
            id: "bookmarks",
            h2 {
                class: "sr-only",
                { "Bookmarks" }
            }
            QuickSiteColumns {}
        }
    )
}
