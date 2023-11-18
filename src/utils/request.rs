use hyper::{body::HttpBody, Body, Error, Response};

pub async fn read_data(
    response: Result<Response<Body>, Error>,
) -> Result<(Response<Body>, Vec<u8>), Error> {
    let mut response = response?;
    let mut bytes: Vec<u8> = Vec::new();
    while let Some(next) = response.data().await {
        let chunk = next?;
        bytes.extend_from_slice(&chunk);
    }
    Ok((response, bytes))
}

pub async fn read_str(
    response: Result<Response<Body>, Error>,
) -> Result<(Response<Body>, String), String> {
    match read_data(response).await {
        Ok((res, bytes)) => match String::from_utf8(bytes) {
            Ok(parsed) => Ok((res, parsed)),
            Err(e) => Err(format!("Received corrupt response from Octyne! {}", e)),
        },
        Err(e) => Err(format!("Failed to read response from Octyne! {}", e)),
    }
}
