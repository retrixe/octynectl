use hyper::{Body, Client, Method, Request};
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;
#[cfg(target_family = "unix")]
use tokio::net::UnixStream;
#[cfg(target_family = "windows")]
use tokio_tungstenite::tungstenite::{client, WebSocket};
#[cfg(target_family = "unix")]
use tokio_tungstenite::{client_async, WebSocketStream};
#[cfg(target_family = "windows")]
use uds_windows::UnixStream;

use crate::{api::common::ErrorResponse, utils::misc};

use super::common::ActionResponse;

#[derive(Clone, Debug)]
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
        return Err(format!(
            "Octyne failed to {} the app!",
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
    pub status: i32,
    #[serde(default)]
    pub cpu_usage: f64,
    #[serde(default)]
    pub memory_usage: i64,
    #[serde(default)]
    pub total_memory: i64,
    #[serde(default)]
    pub uptime: i64,
    #[serde(default)]
    pub to_delete: bool,
}

pub async fn get_server(server_name: String) -> Result<GetServerResponse, String> {
    let endpoint = format!("/server/{}", server_name);
    let client = Client::unix();
    let uri = Uri::new(misc::default_octyne_path(), endpoint.as_str()).into();
    let response = client.get(uri).await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    let json: GetServerResponse = match serde_json::from_str(body.trim()) {
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
    Ok(json)
}

#[cfg(target_family = "unix")]
pub async fn connect_to_server_console(
    server_name: String,
) -> Result<WebSocketStream<UnixStream>, String> {
    // Connect to WebSocket over Unix socket
    let stream = UnixStream::connect(misc::default_octyne_path())
        .await
        .map_err(|e| format!("Error connecting to Unix domain socket! {}", e))?;
    let (socket, response) = client_async(
        format!("ws://localhost:42069/server/{}/console", server_name).as_str(),
        stream,
    )
    .await
    .map_err(|e| format!("Error connecting to WebSocket! {}", e))?;

    // If the server refused to upgrade to WebSocket
    check_websocket_conn_response(response)?;
    Ok(socket)
}

#[cfg(target_family = "windows")]
pub async fn connect_to_server_console(
    server_name: String,
) -> Result<WebSocket<UnixStream>, String> {
    // Connect to WebSocket over Unix socket
    let stream = UnixStream::connect(misc::default_octyne_path())
        .map_err(|e| format!("Error connecting to Unix domain socket! {}", e))?;
    let (socket, response) = client(
        format!("ws://localhost:42069/server/{}/console", server_name).as_str(),
        stream,
    )
    .map_err(|e| format!("Error connecting to WebSocket! {}", e))?;

    // If the server refused to upgrade to WebSocket
    check_websocket_conn_response(response)?;
    Ok(socket)
}

fn check_websocket_conn_response(response: hyper::Response<Option<Vec<u8>>>) -> Result<(), String> {
    if response.status() != 101 {
        return Err(format!(
            "Error: Received status code {} from Octyne!{}",
            response.status(),
            response.body().as_ref().map_or("".to_string(), |body| {
                " Reason: ".to_string()
                    + &serde_json::from_slice(body.as_slice())
                        .map(|json: ErrorResponse| json.error)
                        .unwrap_or("N/A".to_string())
            })
        ));
    }
    Ok(())
}
