use hyper::{Body, Client, Method, Request};
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;

use crate::utils::misc;

use super::common::ActionResponse;

#[derive(Debug)]
pub enum PostServerAction {
    Start,
    Kill,
    Term,
}

impl std::fmt::Display for PostServerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub async fn post_server(server_name: String, action: PostServerAction) -> Result<(), String> {
    let endpoint = format!("/server/{}", server_name);
    let client = Client::unix();
    let req = Request::builder()
        .method(Method::POST)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Body::from(action.to_string().to_uppercase()))
        .expect("request builder");
    let response = client.request(req).await;
    let (res, body) = match crate::utils::request::read_str(response).await {
        Ok((res, body)) => (res, body),
        Err(e) => {
            return Err(e);
        }
    };

    let json: ActionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!(
                "Error: Received corrupt response from Octyne! {}",
                e
            ));
        }
    };

    if res.status() != 200 && json.error.is_empty() {
        return Err(format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        ));
    } else if !json.error.is_empty() {
        return Err(format!("Error: {}", json.error));
    } else if !json.success {
        return Err(format!(
            "Error: Octyne failed to {} the app!",
            action.to_string().to_lowercase()
        ));
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetServerResponse {
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub cpu_usage: f64,
    #[serde(default)]
    pub memory_usage: i64,
    #[serde(default)]
    pub total_memory: i64,
    #[serde(default)]
    pub uptime: i64,
}

pub async fn get_server(server_name: String) -> Result<GetServerResponse, String> {
    let endpoint = format!("/server/{}", server_name);
    let client = Client::unix();
    let uri = Uri::new(misc::default_octyne_path(), endpoint.as_str()).into();
    let response = client.get(uri).await;
    let (res, body) = match crate::utils::request::read_str(response).await {
        Ok((res, body)) => (res, body),
        Err(e) => {
            return Err(e);
        }
    };

    let json: GetServerResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Error: Received corrupt response from Octyne! {}", e));
        }
    };

    if res.status() != 200 && json.error.is_empty() {
        return Err(format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        ));
    } else if !json.error.is_empty() {
        return Err(format!("Error: {}", json.error));
    }
    Ok(json)
}
