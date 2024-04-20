use actix_web::{
    delete, get, put,
    web::{Data, Form, Path},
    HttpResponse,
};
use homescreen_data::prelude::*;
use homescreen_errors::prelude::*;
use log::{error, info};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct WebsiteForm {
    website_name: String,
    website_link: String,
    section: String,
}
impl TryFrom<WebsiteForm> for Website {
    type Error = HomescreenError;

    fn try_from(
        WebsiteForm {
            website_name,
            website_link,
            section,
        }: WebsiteForm,
    ) -> Result<Self, Self::Error> {
        Self::new(website_name, website_link, section)
    }
}

#[get("/websites")]
pub async fn get_websites(database: Data<Box<MySqlPool>>) -> HomescreenResult<HttpResponse> {
    info!("Retrieving websites");

    let websites = Website::get_websites(&database)
        .await
        .inspect_err(|err| error!("Unable to get websites, {err}"))?;

    Ok(HttpResponse::Ok().json(websites))
}

#[get("/websites/coding")]
pub async fn get_coding_websites(database: Data<Box<MySqlPool>>) -> HomescreenResult<HttpResponse> {
    info!("Retrieving coding websites");

    let websites = Website::get_coding_websites(&database)
        .await
        .inspect_err(|err| error!("Unable to get coding websites, {err}"))?;

    Ok(HttpResponse::Ok().json(websites))
}

#[get("/websites/fun")]
pub async fn get_fun_website(database: Data<Box<MySqlPool>>) -> HomescreenResult<HttpResponse> {
    info!("Retrieving fun websites");

    let websites = Website::get_fun_websites(&database)
        .await
        .inspect_err(|err| error!("Unable to get fun websites, {err}"))?;

    Ok(HttpResponse::Ok().json(websites))
}

#[get("/websites/editing")]
pub async fn get_editing_websites(
    database: Data<Box<MySqlPool>>,
) -> HomescreenResult<HttpResponse> {
    info!("Retrieving editing websites");

    let websites = Website::get_editing_websites(&database)
        .await
        .inspect_err(|err| error!("Unable to get editing websites, {err}"))?;

    Ok(HttpResponse::Ok().json(websites))
}

#[put("/websites")]
pub async fn create_or_update_website(
    Form(website): Form<WebsiteForm>,
    database: Data<Box<MySqlPool>>,
) -> HomescreenResult<HttpResponse> {
    info!("Inserting website");

    let website: Website = website
        .try_into()
        .inspect_err(|err| error!("Unable to parse website from form, {err}"))?;

    website.create_or_update_website(&database).await?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/websites/{website_name}")]
pub async fn delete_website(
    website_name: Path<String>,
    database: Data<Box<MySqlPool>>,
) -> HomescreenResult<HttpResponse> {
    info!("Deleting websites");

    Website::delete_websites(website_name.as_str(), &database)
        .await
        .inspect_err(|err| {
            error!(
                "Unable to delete website with name {}, {err}",
                website_name.as_str()
            )
        })?;

    Ok(HttpResponse::Ok().finish())
}
