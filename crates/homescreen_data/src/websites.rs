// #[cfg(feature = "poison_wasm")]
use homescreen_errors::prelude::*;
#[cfg(feature = "poison_wasm")]
use log::error;
use serde::{Deserialize, Serialize};
#[cfg(feature = "poison_wasm")]
use sqlx::{prelude::FromRow, MySqlPool};
#[cfg(feature = "poison_wasm")]
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WebsiteSection {
    Code,
    Fun,
    Editing,
}
impl ToString for WebsiteSection {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}
impl TryFrom<String> for WebsiteSection {
    type Error = HomescreenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}
impl FromStr for WebsiteSection {
    type Err = HomescreenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "code" => Ok(Self::Code),
            "fun" => Ok(Self::Fun),
            "editing" => Ok(Self::Editing),
            _ => Err(ServerError::CannotParseWebsiteSection.into()),
        }
    }
}
impl WebsiteSection {
    pub const ALL: [Self; 3] = [Self::Code, Self::Fun, Self::Editing];
}

#[cfg_attr(feature = "poison_wasm", derive(FromRow))]
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Website {
    website_name: String,
    website_link: String,
    #[cfg_attr(feature = "poison_wasm", sqlx(try_from = "String"))]
    section: WebsiteSection,
}
impl Website {
    pub fn new(
        website_name: String,
        website_link: String,
        section: WebsiteSection,
    ) -> HomescreenResult<Self> {
        Self::validate_link(website_link).and_then(|website_link| {
            Ok(Self {
                website_name,
                website_link,
                section,
            })
        })
    }
    pub fn validate_link(website_link: String) -> HomescreenResult<String> {
        let protocol_specified =
            website_link.starts_with("https://") || website_link.starts_with("http://");

        if protocol_specified {
            Err(ServerError::WebsiteLinkIncludesTransferProtocol.into())
        } else {
            Ok(website_link)
        }
    }
    pub fn name(&self) -> &str {
        &self.website_name
    }
    pub fn link(&self) -> &str {
        &self.website_link
    }
    pub fn icon_link(&self) -> String {
        format!("https://icons.duckduckgo.com/ip3/{}.ico", self.website_link)
    }
    pub fn section(&self) -> &WebsiteSection {
        &self.section
    }
}

#[cfg(feature = "poison_wasm")]
impl Website {
    pub async fn get_websites(
        database: &MySqlPool,
    ) -> HomescreenResult<HashMap<WebsiteSection, Box<[Website]>>> {
        let coding = Self::get_coding_websites(database).await?;
        let fun = Self::get_fun_websites(database).await?;
        let editing = Self::get_editing_websites(database).await?;

        let mut websites = HashMap::new();

        websites.insert(WebsiteSection::Code, coding);
        websites.insert(WebsiteSection::Fun, fun);
        websites.insert(WebsiteSection::Editing, editing);

        Ok(websites)
    }
    pub async fn get_coding_websites(database: &MySqlPool) -> HomescreenResult<Box<[Self]>> {
        sqlx::query_as(
            "SELECT website_name, website_link, section FROM websites WHERE section = 'code'",
        )
        .fetch_all(database)
        .await
        .map_err(ServerError::CannotRetrieveWebsites)
        .map_err(HomescreenError::from)
        .inspect_err(|err| error!("Cannot retrieve coding websites, {err}"))
        .map(Vec::into_boxed_slice)
    }
    pub async fn get_fun_websites(database: &MySqlPool) -> HomescreenResult<Box<[Self]>> {
        sqlx::query_as(
            "SELECT website_name, website_link, section FROM websites WHERE section = 'fun'",
        )
        .fetch_all(database)
        .await
        .map_err(ServerError::CannotRetrieveWebsites)
        .map_err(HomescreenError::from)
        .inspect_err(|err| error!("Cannot retrieve fun websites, {err}"))
        .map(Vec::into_boxed_slice)
    }
    pub async fn get_editing_websites(database: &MySqlPool) -> HomescreenResult<Box<[Self]>> {
        sqlx::query_as(
            "SELECT website_name, website_link, section FROM websites WHERE section = 'editing'",
        )
        .fetch_all(database)
        .await
        .map_err(ServerError::CannotRetrieveWebsites)
        .map_err(HomescreenError::from)
        .inspect_err(|err| error!("Cannot retrieve editing websites, {err}"))
        .map(Vec::into_boxed_slice)
    }
    pub async fn create_or_update_website(self, database: &MySqlPool) -> HomescreenResult {
        sqlx::query("INSERT INTO websites(website_name, website_link, section) VALUES(?, ?, ?)")
            .bind(self.website_name)
            .bind(self.website_link)
            .bind(self.section.to_string().to_lowercase())
            .execute(database)
            .await
            .map_err(ServerError::CannotInsertWebsite)
            .map_err(HomescreenError::from)
            .inspect_err(|err| error!("Cannot insert website, {err}"))?;

        Ok(())
    }
    pub async fn delete_websites(website_name: &str, database: &MySqlPool) -> HomescreenResult {
        sqlx::query("DELETE FROM websites WHERE website_name = ?")
            .bind(website_name)
            .execute(database)
            .await
            .map_err(ServerError::CannotDeleteWebsite)
            .and_then(|rows| {
                if rows.rows_affected() == 0 {
                    Err(ServerError::CannotDeleteExistingWebsite)
                } else {
                    Ok(())
                }
            })
            .map_err(HomescreenError::from)
            .inspect_err(|err| error!("Cannot delete website, {err}"))
    }
}
