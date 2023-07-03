use hyper::{body::HttpBody, Body, Error, Response};

pub async fn read_data(
    response: Result<Response<Body>, Error>,
) -> Result<(Response<Body>, Vec<u8>), String> {
    match response {
        Ok(mut response) => {
            let mut bytes: Vec<u8> = Vec::new();
            while let Some(next) = response.data().await {
                match next {
                    Ok(chunk) => {
                        bytes.extend_from_slice(&chunk);
                    }
                    Err(e) => {
                        return Err(format!("Error reading from Octyne socket: {}", e));
                    }
                }
            }
            Ok((response, bytes))
        }
        Err(e) => Err(format!("Error requesting info from Octyne socket: {}", e)),
    }
}

pub async fn read_str(
    response: Result<Response<Body>, Error>,
) -> Result<(Response<Body>, String), String> {
    match read_data(response).await {
        Ok((res, bytes)) => match String::from_utf8(bytes) {
            Ok(parsed) => Ok((res, parsed)),
            Err(e) => Err(format!(
                "Error: Received corrupt response from Octyne! {}",
                e
            )),
        },
        Err(e) => Err(e),
    }
}
