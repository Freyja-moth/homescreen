use std::collections::HashMap;

use dioxus::prelude::*;
use homescreen_data::prelude::{Website, WebsiteSection};
use homescreen_errors::prelude::*;

type WebsiteCollection = HashMap<WebsiteSection, Vec<Website>>;

#[server(GetWebsites)]
pub async fn get_websites() -> Result<WebsiteCollection, ServerFnError> {
    let response = reqwest::get("http://127.0.0.1:8888/websites")
        .await
        .map_err(FrontendError::CannotRetrieveWebsites)?
        .json::<WebsiteCollection>()
        .await
        .map_err(FrontendError::InvalidResponseRecieved)?;

    Ok(response)
}
