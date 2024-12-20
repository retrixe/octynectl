use http_body_util::{Empty, Full};
use hyper::{body::Bytes, Method, Request};
use hyper_util::client::legacy::Client;
use hyperlocal_with_windows::{UnixClientExt, UnixConnector, Uri};
use serde::Serialize;
use serde_json::Value;

use crate::utils::misc;

use super::common::{ActionResponse, ErrorResponse};

pub async fn get_accounts() -> Result<Vec<String>, String> {
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let uri = Uri::new(misc::default_octyne_path(), "/accounts").into();
    let response = client.get(uri).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: Value = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Received corrupt response from Octyne! {}", e));
        }
    };

    if json.is_object() {
        let resp: ErrorResponse = match serde_json::from_value(json) {
            Ok(res) => res,
            Err(e) => return Err(format!("Received corrupt response from Octyne! {}", e)),
        };
        if resp.error.is_empty() {
            return Err("Received corrupt response from Octyne!".to_string());
        } else {
            return Err(resp.error);
        }
    } else if res.status() != 200 {
        return Err(format!(
            "Received status code {} from Octyne!",
            res.status().as_str()
        ));
    }

    match serde_json::from_value(json) {
        Ok(accounts) => Ok(accounts),
        Err(err) => Err(format!("Received corrupt response from Octyne! {}", err)),
    }
}

pub async fn post_account(username: String, password: String) -> Result<(), String> {
    let ok = post_patch_account(None, username, password, Method::POST).await?;
    if !ok {
        return Err("Octyne failed to create the account!".to_owned());
    }
    Ok(())
}

pub async fn patch_account(
    old_user: Option<String>,
    username: String,
    password: String,
) -> Result<(), String> {
    let ok = post_patch_account(old_user, username, password, Method::PATCH).await?;
    if !ok {
        return Err("Octyne failed to modify the account!".to_owned());
    }
    Ok(())
}

#[derive(Serialize, Debug)]
struct PostAccountRequest {
    username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    password: String,
}

async fn post_patch_account(
    old_user: Option<String>,
    username: String,
    password: String,
    method: Method,
) -> Result<bool, String> {
    let body = serde_json::to_string(&PostAccountRequest { username, password });
    if let Err(err) = body {
        return Err(err.to_string());
    }
    let mut endpoint = "/accounts".to_string();
    if old_user.is_some() {
        endpoint = format!("/accounts?username={}", old_user.unwrap());
    }
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let req = Request::builder()
        .method(method)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Full::from(body.unwrap()))
        .expect("request builder");
    let response = client.request(req).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: ActionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Received corrupt response from Octyne! {}", e));
        }
    };

    if res.status() != 200 && json.error.is_empty() {
        return Err(format!(
            "Received status code {} from Octyne!",
            res.status().as_str()
        ));
    } else if !json.error.is_empty() {
        return Err(json.error);
    }
    Ok(json.success)
}

pub async fn delete_account(username: String) -> Result<(), String> {
    let endpoint = format!("/accounts?username={}", username);
    let client: Client<UnixConnector, Empty<Bytes>> = Client::unix();
    let req = Request::builder()
        .method(Method::DELETE)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Empty::new())
        .expect("request builder");
    let response = client.request(req).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: ActionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Received corrupt response from Octyne! {}", e));
        }
    };

    if res.status() != 200 && json.error.is_empty() {
        return Err(format!(
            "Received status code {} from Octyne!",
            res.status().as_str()
        ));
    } else if !json.error.is_empty() {
        return Err(json.error);
    } else if !json.success {
        return Err("Octyne failed to delete the account!".to_owned());
    }
    Ok(())
}
