#[cfg(target_family = "windows")]
use std::fmt::Write;
use std::{collections::HashMap, env, process::exit};

use crate::{api::common::ErrorResponse, utils::misc::default_octyne_path};
#[cfg(target_family = "unix")]
use futures_util::StreamExt;
#[cfg(target_family = "windows")]
use minus::Pager;
#[cfg(target_family = "unix")]
use pager::Pager;
#[cfg(target_family = "unix")]
use tokio::net::UnixStream;
#[cfg(target_family = "unix")]
use tokio_tungstenite::client_async;
#[cfg(target_family = "windows")]
use tokio_tungstenite::tungstenite::client;
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
#[cfg(target_family = "windows")]
use uds_windows::UnixStream;

pub async fn logs_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        logs_cmd_help();
        return;
    } else if args.len() != 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "logs")
        );
        exit(1);
    }
    let no_pager = opts.contains_key("no-pager") || env::var("NOPAGER").eq(&Ok("true".to_string()));

    // Connect to WebSocket over Unix socket
    #[cfg(target_family = "unix")]
    let stream = UnixStream::connect(default_octyne_path())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    #[cfg(target_family = "unix")]
    let (socket, response) = client_async(
        format!("ws://localhost:42069/server/{}/console", args[1]).as_str(),
        stream,
    )
    .await
    .unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });
    #[cfg(target_family = "windows")]
    let stream = UnixStream::connect(default_octyne_path()).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });
    #[cfg(target_family = "windows")]
    let (socket, response) = client(
        format!("ws://localhost:42069/server/{}/console", args[1]).as_str(),
        stream,
    )
    .unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });

    // If the server refused to upgrade to WebSocket
    if response.status() != 101 {
        let error: String = response.body().as_ref().map_or("".to_string(), |body| {
            return " ".to_string()
                + &serde_json::from_slice(body.as_slice())
                    .unwrap_or(ErrorResponse {
                        error: "".to_string(),
                    })
                    .error;
        });
        println!(
            "Error: Received status code {} from Octyne!{}",
            response.status(),
            error
        );
        exit(1);
    }

    // Split the socket and then read a single message from it
    #[cfg(target_family = "unix")]
    let (write, read) = socket.split();
    #[cfg(target_family = "unix")]
    let (item, read) = read.into_future().await;
    #[cfg(target_family = "unix")]
    if item.is_none() {
        println!("Error: Received no message from Octyne!");
        exit(1);
    }
    #[cfg(target_family = "unix")]
    let item = item.unwrap();
    #[cfg(target_family = "windows")]
    let item = socket.read();
    if item.is_err() {
        println!("Error: {}", item.err().unwrap());
        exit(1);
    }
    let item = item.unwrap();
    if item.is_close() {
        println!("Error: Received close message from Octyne!");
        exit(1);
    }
    let item = item.to_text().unwrap();

    // Close the WebSocket connection.
    #[cfg(target_family = "unix")]
    {
        let mut socket = read.reunite(write).unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
        socket
            .close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "Done".into(),
            }))
            .await
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });
    }
    #[cfg(target_family = "windows")]
    socket
        .close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "Done".into(),
        }))
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });

    // Log the output.
    if no_pager {
        println!("{}", item);
    } else {
        #[cfg(target_family = "unix")]
        {
            Pager::with_default_pager("less").setup();
            println!("{}", item);
            exit(0);
        }
        #[cfg(target_family = "windows")]
        {
            let mut output = Pager::new();
            writeln!(output, "{}", item).unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });
            minus::page_all(output).unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });
        }
    }
}

pub fn logs_cmd_help() {
    println!(
        "Get the output logs of an app.

Usage: octynectl logs [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information
    --no-pager               Don't use a pager to display logs"
    );
}
