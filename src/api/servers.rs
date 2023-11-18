use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::utils::misc;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(default)]
    servers: Map<String, Value>,
    #[serde(default)]
    error: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerExtraInfo {
    #[serde(default)]
    pub status: i64,
    #[serde(default)]
    pub to_delete: bool,
}

pub async fn get_servers(extra_info: bool) -> Result<Map<String, Value>, String> {
    let endpoint = if extra_info {
        "/servers?extrainfo=true"
    } else {
        "/servers"
    };
    let url = Uri::new(misc::default_octyne_path(), endpoint).into();
    let client = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: Response = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => return Err(format!("Received corrupt response from Octyne! {}", e)),
    };

    if !json.error.is_empty() {
        return Err(json.error);
    } else if res.status() != 200 {
        return Err(format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        ));
    }
    Ok(json.servers)
}

/* pub async fn get_servers_without_extra_info() -> Result<Map<String, bool>, String> {
    get_servers(false).await.map(|servers| {
        servers
            .into_iter()
            .map(|(name, _)| (name, true))
            .collect::<Map<String, bool>>()
    })
} */
