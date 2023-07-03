use hyper::{Body, Client, Method, Request};
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;

use super::misc;

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

#[derive(Deserialize, Debug)]
pub struct ActionResponse {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    error: String,
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
    return Ok(());
}
