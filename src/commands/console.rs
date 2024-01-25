use std::{collections::HashMap, process::exit};

use crate::api::server::connect_to_server_console;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{FramedRead, LinesCodec};

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
    let (write, read) = socket.split();

    // The threads don't close each other because if the read thread errors, it's unlikely writing
    // a close message will work, and vice versa? TODO: But then the read thread doesn't get flushed
    // Construct pager
    /* let mut output = if no_interactive {
        Option::Some(minus::Pager::new())
    } else {
        Option::None
    }; */
    // Create read thread
    // let (tx, rx) = tokio::sync::oneshot::channel::<()>();
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
        // tx.send(()).unwrap(); // Non-issue, this channel just ensures read is flushed before exit
    });
    // Create write thread
    let mut write = write;
    let stdin = tokio::io::stdin();
    let mut stdin = FramedRead::new(stdin, LinesCodec::new());
    while let Some(line) = stdin.next().await {
        let line = line.unwrap_or_else(|e| {
            println!("Write error: {}", e);
            exit(1);
        });
        if line.is_empty() {
            continue;
        }
        write.send(Message::Text(line)).await.unwrap_or_else(|e| {
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
    // Wait for the read thread to exit
    /* rx.await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    }); */
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
