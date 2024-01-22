use serde::Deserialize;
use crate::settings::{CoreSettings, LoginSettings};

#[derive(Debug, Deserialize)]
pub(crate) struct OxiFileConfig {
    pub(crate) core: Option<CoreSettings>,
    pub(crate) login: Option<LoginSettings>
}
