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
/// The section used to determine where a website should be placed on the webpage
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
/// A representation of a website
pub struct Website {
    website_name: String,
    website_link: String,
    #[cfg_attr(feature = "poison_wasm", sqlx(try_from = "String"))]
    section: WebsiteSection,
}
impl Website {
    /// Creates a website from a name, link and section
    ///
    /// # Errors
    /// [ServerError::WebsiteLinkIncludesTransferProtocol]: If the wesbite link includes http:// or
    /// https://, which would mess with the way website favicons are retrieved
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
    /// Validates a link to make sure that it works properly on the frontend
    ///
    /// # Errors
    /// [ServerError::WebsiteLinkIncludesTransferProtocol]: If the wesbite link includes http:// or
    /// https://, which would mess with the way website favicons are retrieved
    pub fn validate_link(website_link: String) -> HomescreenResult<String> {
        let protocol_specified =
            website_link.starts_with("https://") || website_link.starts_with("http://");

        if protocol_specified {
            Err(ServerError::WebsiteLinkIncludesTransferProtocol.into())
        } else {
            Ok(website_link)
        }
    }
    /// Retrieves the website name
    pub fn name(&self) -> &str {
        &self.website_name
    }
    /// Retrieves the website link
    pub fn link(&self) -> &str {
        &self.website_link
    }
    /// Formats the website link into an icon link
    pub fn icon_link(&self) -> String {
        format!("https://icons.duckduckgo.com/ip3/{}.ico", self.website_link)
    }
    /// Retrieves the website section
    pub fn section(&self) -> &WebsiteSection {
        &self.section
    }
}

// Used because importing sqlx will cause the frontend to fail compilation as they rely on a non
// wasm friendly crate
#[cfg(feature = "poison_wasm")]
impl Website {
    /// Retrieves all websites from the database
    ///
    /// # Errors
    /// [ServerError::CannotRetrieveWebsites]: If any of the databases query fail
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
    /// Gets all websites that belong to the coding catagory
    ///
    /// # Errors
    /// [ServerError::CannotRetrieveWebsites]: If the database fails
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
    /// Gets all websites that belong to the fun catagory
    ///
    /// # Errors
    /// [ServerError::CannotRetrieveWebsites]: If the database fails
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
    /// Gets all websites that belong to the editing catagory
    ///
    /// # Errors
    /// [ServerError::CannotRetrieveWebsites]: If the database fails
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
    /// Creates or updates a website in the database
    ///
    /// # Errors
    /// [ServerError::CannotInsertWebsite]: if the query fails
    pub async fn create_or_update_website(self, database: &MySqlPool) -> HomescreenResult {
        sqlx::query("INSERT INTO websites(website_name, website_link, section) VALUES(?, ?, ?) ON DUPLICATE KEY UPDATE website_link=?, section=?")
            .bind(self.website_name)
            .bind(&self.website_link)
            .bind(self.section.to_string().to_lowercase())
            .bind(&self.website_link)
            .bind(self.section.to_string().to_lowercase())
            .execute(database)
            .await
            .map_err(ServerError::CannotInsertWebsite)
            .map_err(HomescreenError::from)
            .inspect_err(|err| error!("Cannot insert website, {err}"))?;

        Ok(())
    }
    /// Deletes a website using it's id
    ///
    /// # Errors
    /// [ServerError::CannotDeleteWebsite]: If the query fails
    /// [ServerError::CannotDeleteNonExistantWebsite]: If the website does not exist
    pub async fn delete_websites(website_name: &str, database: &MySqlPool) -> HomescreenResult {
        sqlx::query("DELETE FROM websites WHERE website_name = ?")
            .bind(website_name)
            .execute(database)
            .await
            .map_err(ServerError::CannotDeleteWebsite)
            .and_then(|rows| {
                if rows.rows_affected() == 0 {
                    Err(ServerError::CannotDeleteNonExistantWebsite)
                } else {
                    Ok(())
                }
            })
            .map_err(HomescreenError::from)
            .inspect_err(|err| error!("Cannot delete website, {err}"))
    }
}
