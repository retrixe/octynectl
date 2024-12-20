use std::{collections::HashMap, process::exit};

use crate::api::server::connect_to_server_console;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{FramedRead, LinesCodec};

// TODO: Support console-v2 (send periodic keep-alive pings, receive JSON formatted messages)
// https://github.com/retrixe/octyne/blob/main/API.md#ws-serveridconsoleticketticket
pub async fn console_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        console_cmd_help();
        return;
    } else if args.len() != 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "console")
        );
        exit(1);
    }
    let no_interactive = opts.contains_key("no-interactive") // --no-interactive is set
        || !atty::is(atty::Stream::Stdout); // TTY is not present

    // Connect to WebSocket over Unix socket
    let socket = connect_to_server_console(args[1].clone())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    let (mut write, read) = socket.split();
    // Create a channel, if reading fails, terminate writing.
    // TODO: Ideally we should have no sudden exits in the code...
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // Construct pager
    /* let mut output = if no_interactive {
        Option::Some(minus::Pager::new())
    } else {
        Option::None
    }; */
    // Create read thread
    tokio::spawn(async move {
        let mut read = read;
        while let Some(item) = read.next().await {
            let item = item.unwrap_or_else(|e| {
                println!("Read error: {}", e);
                exit(1);
            });
            if item.is_close() {
                println!("Read error: Received close message from Octyne!");
                exit(1);
            }
            let item = item.to_text().unwrap_or_else(|e| {
                println!("Read error: {}", e);
                exit(1);
            });
            if no_interactive {
                println!("{}", item);
                continue;
            }
            // FIXME: Interactive session
            /* minus_page_lines(item).unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            }); */
            println!("{}", item);
        }
        tx.send(()).unwrap(); // Signal write thread to terminate
    });

    // Create write thread
    let mut stdin = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
    while let Some(line) = stdin.next().await {
        let line = line.unwrap_or_else(|e| {
            println!("Write error: {}", e);
            exit(1);
        });
        if line.is_empty() {
            continue;
        }
        write
            .send(Message::Text(line.into()))
            .await
            .unwrap_or_else(|e| {
                println!("Write error: {}", e);
                exit(1);
            });
    }

    // Gracefully exit on EOF
    write
        .send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "Done".into(),
        })))
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    write.close().await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });
    rx.await.unwrap(); // Wait for the read thread to finish reading and exit
    exit(0);
}

pub fn console_cmd_help() {
    println!(
        "Interact with an app's console and send input.

This opens an interactive terminal UI, which can be disabled using the
`--no-interactive` flag. The interactive UI will be disabled automatically if
the command output is not being sent to a TTY (terminal) session.

If you only want the app's output logs, and don't want to send any input to it,
use the `logs` command instead.

Usage: octynectl console [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information
    --no-interactive         Don't setup an interactive console for an end user,
                             just accept stdin and log output to stdout"
    );
}
