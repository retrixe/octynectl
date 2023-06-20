use std::process::exit;

use hyper::{body::HttpBody, Body, Error, Response};

pub async fn read_data_or_exit(
    response: Result<Response<Body>, Error>,
) -> (Response<Body>, Vec<u8>) {
    match response {
        Ok(mut response) => {
            let mut bytes: Vec<u8> = Vec::new();
            while let Some(next) = response.data().await {
                match next {
                    Ok(chunk) => {
                        bytes.extend_from_slice(&chunk);
                    }
                    Err(e) => {
                        println!("Error reading from Octyne socket: {}", e);
                        exit(1);
                    }
                }
            }
            (response, bytes)
        }
        Err(e) => {
            println!("Error requesting info from Octyne socket: {}", e);
            exit(1);
        }
    }
}

pub async fn read_str_or_exit(response: Result<Response<Body>, Error>) -> (Response<Body>, String) {
    let (res, bytes) = read_data_or_exit(response).await;
    let parsed = String::from_utf8(bytes).unwrap_or_else(|e| {
        println!("Error: Received corrupt response from Octyne! {}", e);
        exit(1);
    });
    (res, parsed)
}
