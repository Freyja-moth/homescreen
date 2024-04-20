pub type HomescreenResult<T = ()> = Result<T, HomescreenError>;
pub use crate::error::{ConfigError, FrontendError, HomescreenError, ServerError, StartupError};
