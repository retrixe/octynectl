use hyper::{Body, Client, Method, Request};
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Serialize;
use serde_json::Value;

use crate::utils::misc;

use super::common::{ActionResponse, ErrorResponse};

pub async fn get_accounts() -> Result<Vec<String>, String> {
    let client = Client::unix();
    let uri = Uri::new(misc::default_octyne_path(), "/accounts").into();
    let response = client.get(uri).await;
    let (res, body) = match crate::utils::request::read_str(response).await {
        Ok((res, body)) => (res, body),
        Err(e) => {
            return Err(e);
        }
    };

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
            return Err(format!("Received corrupt response from Octyne!"));
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
        Ok(accounts) => return Ok(accounts),
        Err(err) => return Err(format!("Received corrupt response from Octyne! {}", err)),
    }
}

pub async fn create_account(username: String, password: String) -> Result<(), String> {
    match post_account(username, password, Method::POST).await {
        Ok(ok) => {
            if !ok {
                return Err("Octyne failed to create the account!".to_owned());
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub async fn patch_account(username: String, password: String) -> Result<(), String> {
    match post_account(username, password, Method::PATCH).await {
        Ok(ok) => {
            if !ok {
                return Err("Octyne failed to change the account password!".to_owned());
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[derive(Serialize, Debug)]
struct PostAccountRequest {
    username: String,
    password: String,
}

async fn post_account(username: String, password: String, method: Method) -> Result<bool, String> {
    let body = serde_json::to_string(&PostAccountRequest {
        username: username,
        password: password,
    });
    if body.is_err() {
        return Err(body.unwrap_err().to_string());
    }
    let endpoint = format!("/accounts");
    let client = Client::unix();
    let req = Request::builder()
        .method(method)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Body::from(body.unwrap()))
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
    let client = Client::unix();
    let req = Request::builder()
        .method(Method::DELETE)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Body::empty())
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
