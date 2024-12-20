use http_body_util::BodyExt;
use hyper::{body::Incoming, Response};
use hyper_util::client::legacy::Error;

pub async fn read_data(
    response: Result<Response<Incoming>, Error>,
) -> Result<(Response<Incoming>, Vec<u8>), String> {
    let mut response = match response {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to read response from Octyne! {}", e)),
    };
    let mut bytes: Vec<u8> = Vec::new();
    while let Some(next) = response.frame().await {
        let frame = match next {
            Ok(res) => res,
            Err(e) => return Err(format!("Failed to read response from Octyne! {}", e)),
        };
        if let Some(chunk) = frame.data_ref() {
            bytes.extend_from_slice(chunk);
        }
    }
    Ok((response, bytes))
}

pub async fn read_str(
    response: Result<Response<Incoming>, Error>,
) -> Result<(Response<Incoming>, String), String> {
    match read_data(response).await {
        Ok((res, bytes)) => match String::from_utf8(bytes) {
            Ok(parsed) => Ok((res, parsed)),
            Err(e) => Err(format!("Received corrupt response from Octyne! {}", e)),
        },
        Err(e) => Err(e),
    }
}
