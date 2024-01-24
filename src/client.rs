use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct LoginResponse {
    #[serde(rename = "sId")]
    pub(crate) s_id: String,
    #[serde(rename = "lobbyHost")]
    pub(crate) lobby_host: String,
    #[serde(rename = "frontierHost")]
    pub(crate) frontier_host: String,
}

pub(crate) struct LoginAuthResponse {
    pub(crate) sid: String,
    pub(crate) lobby_host: String,
    pub(crate) frontier_host: String,
    pub(crate) region: i32,
    pub(crate) max_expansion: i32,
    pub(crate) language: i32,
}

impl Default for LoginAuthResponse {
    fn default() -> Self {
        LoginAuthResponse {
            sid: String::new(),
            lobby_host: String::new(),
            frontier_host: String::new(),
            region: 3,
            max_expansion: 1,
            language: 1,
        }
    }
}
