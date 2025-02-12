use std::{collections::HashMap, process::exit};

use crate::api::server::connect_to_server_console;
use crossterm::{execute, tty::IsTty};
use futures_util::{SinkExt, StreamExt};
use tokio::{select, signal};
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

    // Connect to WebSocket over Unix socket
    let (socket, _) = connect_to_server_console(args[1].clone())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    let (mut write, read) = socket.split();

    // Create a channel, if reading fails, terminate write thread and exit.
    // CancellationToken may be better?...
    let (tx, mut rx) = tokio::sync::mpsc::channel::<(i32, String)>(1);

    // If interactive, move to alternate screen
    let interactive = !opts.contains_key("no-interactive") // --no-interactive is unset
        && std::io::stdout().is_tty(); // TTY is present
    if interactive {
        execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    }

    // Create read thread
    tokio::spawn(async move {
        let mut read = read;
        while let Some(item) = read.next().await {
            let item = match item {
                Ok(message) => message,
                Err(e) => {
                    return tx.send((1, format!("Read error: {}", e))).await.unwrap();
                }
            };
            if item.is_close() {
                tx.send((1, "Read error: Received close message from Octyne!".into()))
                    .await
                    .unwrap();
                return;
            }
            match item.to_text() {
                Ok(item) => println!("{}", item),
                Err(e) => {
                    return tx.send((1, format!("Read error: {}", e))).await.unwrap();
                }
            };
        }
        tx.send((0, "Console closed by remote.".into()))
            .await
            .unwrap(); // Signal write thread to terminate
    });

    // Create write thread
    let exit_reason: (i32, String);
    let mut stdin = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
    loop {
        select! {
                Some(line) = stdin.next() => {
                    let line = match line {
                        Ok(line) => line,
                        Err(e) => break exit_reason = (1, format!("Write error: {}", e))
                    };
                    if line.is_empty() {
                        continue;
                    }
                    match write.send(Message::Text(line.into())).await {
                        Ok(()) => {}
                        Err(e) => break exit_reason = (1, format!("Write error: {}", e))
                    }
                }
                recv_exit_code = rx.recv() => break exit_reason = recv_exit_code.unwrap(),
                _ = signal::ctrl_c() => break exit_reason = (0, "".into())
        }
    }

    // Gracefully exit on EOF
    if interactive {
        execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    }
    if exit_reason.0 != 0 {
        println!("{}", exit_reason.1);
    }
    write
        .send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "Done".into(),
        })))
        .await
        .unwrap_or_else(|e| {
            if exit_reason.0 == 0 {
                println!("Close error: {}", e);
                exit(1);
            }
        });
    write.close().await.unwrap_or_else(|e| {
        if exit_reason.0 == 0 {
            println!("Close error: {}", e);
            exit(1);
        }
    });

    exit(exit_reason.0);
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
