use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyperlocal_with_windows::{UnixClientExt, UnixConnector, Uri};
use serde::Deserialize;

use crate::utils::misc;

#[derive(Deserialize, Debug)]
struct VersionResponse {
    #[serde(default)]
    version: String,
}

pub async fn get_version() -> Result<String, String> {
    let client: Client<UnixConnector, Full<Bytes>> = Client::unix();
    let response = client
        .get(Uri::new(misc::default_octyne_path(), "/").into())
        .await;
    let (res, body) = crate::utils::request::read_str(response).await?;

    if body == "Hi, octyne is online and listening to this port successfully!" {
        return Ok("1.0.x".to_string());
    } else if res.status() != 200 {
        return Err(format!(
            "Received status code {} from Octyne!",
            res.status().as_str()
        ));
    }

    let json: VersionResponse = match serde_json::from_str(body.trim()) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Received corrupt response from Octyne! {}", e));
        }
    };

    Ok(json.version)
}
