#[cfg(feature = "poison_wasm")]
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use reqwest::Error as ReqwestError;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use thiserror::Error as ThisError;
use toml::de::Error as DeError;

#[derive(ThisError, Debug)]
pub enum HomescreenError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Startup(#[from] StartupError),
    #[error(transparent)]
    Server(#[from] ServerError),
    #[error(transparent)]
    Frontend(#[from] FrontendError),
}

#[cfg(feature = "poison_wasm")]
impl ResponseError for HomescreenError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Server(err) => err.status_code(),
            _ => StatusCode::IM_A_TEAPOT,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::plaintext())
            .body(format!("{self:?}"))
    }
}

#[derive(ThisError, Debug)]
pub enum ConfigError {
    #[error("Cannot find Config.toml in current directory, why: {0}")]
    CannotFindConfigFile(#[source] IoError),
    #[error("Cannot parse Config.toml, please check format, why: {0}")]
    CannotParseConfigFile(#[source] DeError),
}

#[derive(ThisError, Debug)]
pub enum StartupError {
    #[error("Cannot bind to port {1}, why: {0}")]
    CannotBindToPort(#[source] IoError, u16),
    #[error("Cannot start server, why: {0}")]
    CannotStartServer(#[source] IoError),
    #[error("Cannot connect to database with url {1}, why: {0}")]
    CannotConnectToDatabase(#[source] SqlxError, String),
}

#[derive(ThisError, Debug)]
pub enum ServerError {
    #[error("Cannot retrieve websites, why: {0}")]
    CannotRetrieveWebsites(#[source] SqlxError),
    #[error("Cannot insert website, why: {0}")]
    CannotInsertWebsite(#[source] SqlxError),
    #[error("Cannot delete website, why: {0}")]
    CannotDeleteWebsite(#[source] SqlxError),
    #[error("Unable to delete website that does not exist")]
    CannotDeleteNonExistantWebsite,
    #[error("Cannot parse website section")]
    CannotParseWebsiteSection,
    #[error("Website link includes transfer protocol")]
    WebsiteLinkIncludesTransferProtocol,
}
#[cfg(feature = "poison_wasm")]
impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::CannotInsertWebsite(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CannotRetrieveWebsites(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CannotDeleteWebsite(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CannotDeleteNonExistantWebsite => StatusCode::BAD_REQUEST,
            Self::WebsiteLinkIncludesTransferProtocol => StatusCode::BAD_REQUEST,
            Self::CannotParseWebsiteSection => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(ThisError, Debug)]
pub enum FrontendError {
    #[error("Cannot retrieve websites, why: {0}")]
    CannotRetrieveWebsites(#[source] ReqwestError),
    #[error("Cannot parse websites, why: {0}")]
    InvalidResponseRecieved(#[source] ReqwestError),
}
