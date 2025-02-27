use http_body_util::Full;
use hyper::{body::Bytes, Method, Request};
use hyper_util::client::legacy::Client;
use hyperlocal_with_windows::{UnixClientExt, UnixConnector, Uri};

use crate::utils::misc;

use super::common::{ActionResponse, ErrorResponse};

pub async fn get_config() -> Result<(String, String), String> {
    let url = Uri::new(misc::default_octyne_path(), "/config").into();
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: ErrorResponse = serde_json::from_str(body.trim()).unwrap_or(ErrorResponse {
        error: "".to_string(),
    });

    if !json.error.is_empty() {
        return Err(json.error);
    } else if res.status() != 200 {
        let default = format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        );
        return Err(default);
    }

    let content_type = res.headers().get("Content-Type");
    let content_type = content_type.map(|v| v.to_str().unwrap_or("")).unwrap_or("");
    let (_, extension) = content_type.rsplit_once('/').unwrap_or(("", ""));
    let mut extension = extension.to_string();
    if extension.is_empty() {
        extension = "json".to_string();
    }
    Ok((body, extension))
}

pub async fn get_config_reload() -> Result<(), String> {
    let url = Uri::new(misc::default_octyne_path(), "/config/reload").into();
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: ActionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => return Err(format!("Received corrupt response from Octyne! {}", e)),
    };

    if !json.error.is_empty() {
        return Err(json.error);
    } else if res.status() != 200 {
        let default = format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        );
        return Err(default);
    } else if !json.success {
        return Err("Octyne failed to reload the config!".to_owned());
    }
    Ok(())
}

pub async fn patch_config(new_config: String) -> Result<(), String> {
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let req = Request::builder()
        .method(Method::PATCH)
        .uri(Uri::new(misc::default_octyne_path(), "/config"))
        .body(Full::from(new_config))
        .expect("Failed to build request!");
    let response = client.request(req).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: ActionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => return Err(format!("Received corrupt response from Octyne! {}", e)),
    };

    if !json.error.is_empty() {
        return Err(json.error);
    } else if res.status() != 200 {
        let default = format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        );
        return Err(default);
    } else if !json.success {
        return Err("Octyne failed to load the new config!".to_owned());
    }
    Ok(())
}
