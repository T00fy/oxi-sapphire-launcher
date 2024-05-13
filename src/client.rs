use anyhow::anyhow;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use crate::settings::{CoreSettings, LoginSettings, RegisterSettings};

#[derive(Deserialize, Debug)]
pub(crate) struct LoginServerResponse {
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

pub(crate) async fn send_login_request(core_settings: &CoreSettings, login_settings: &LoginSettings, password: &str) -> anyhow::Result<LoginServerResponse> {
    println!("Sending login request");
    send_request(core_settings, &login_settings.username, password, &login_settings.endpoint).await
}

pub(crate) async fn send_register_request(core_settings: &CoreSettings, register_settings: &RegisterSettings, password: &str) -> anyhow::Result<LoginServerResponse> {
    println!("Sending registration request");
    send_request(core_settings, &register_settings.username, password, &register_settings.endpoint).await
}

/// Function to send HTTP POST request
async fn send_request(core_settings: &CoreSettings, username: &str, password: &str, endpoint: &str) -> anyhow::Result<LoginServerResponse> {
    let url = format!(
        "{}://{}:{}{}",
        core_settings.frontier_scheme,
        core_settings.frontier_ip,
        core_settings.frontier_port,
        endpoint
    );

    let json_data = json!({
        "username": username,
        "pass": password,
    });

    let client = Client::new();
    let res = client.post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .json(&json_data)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let login_response = response.json::<LoginServerResponse>().await
                    .map_err(|e| anyhow!("Failed to deserialize response body: {}", e))?;
                Ok(login_response)
            } else {
                Err(anyhow!("Request failed with status: {}", response.status()))
            }
        },
        Err(e) => Err(anyhow!("Error sending request: {}", e)),
    }
}
