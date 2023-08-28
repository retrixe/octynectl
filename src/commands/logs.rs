use std::{collections::HashMap, process::exit};

use crate::utils::misc::default_octyne_path;
use futures_util::StreamExt;
use tokio::net::UnixStream;
use tokio_tungstenite::client_async;

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

    // Connect to WebSocket over Unix socket
    let stream = UnixStream::connect(default_octyne_path())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    let (socket, response) = client_async(
        format!("ws://localhost:42069/server/{}/console", args[1]).as_str(),
        stream,
    )
    .await
    .unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });

    if response.status() != 101 {
        println!(
            "Error: Received status code {} from Octyne!",
            response.status()
        );
        exit(1);
    }

    let (_, read) = socket.split();
    read.for_each(|message| async {
        println!("receiving...");
        match message {
            Ok(message) => println!("received: {}", message),
            Err(e) => println!("error: {}", e),
        }
    })
    .await;
    // TODO: Read the WebSocket message and then pipe it out to `less` or something.
}

pub fn logs_cmd_help() {
    println!(
        "Get the output logs of an app.

Usage: octynectl logs [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information"
    );
}
