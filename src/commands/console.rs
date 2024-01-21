use std::fmt::Write;
use std::{collections::HashMap, env, process::exit};

use crate::api::server::connect_to_server_console;
use futures_util::StreamExt;
use minus::MinusError;
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

fn minus_page_lines(lines: &str) -> Result<(), MinusError> {
    let mut output = minus::Pager::new();
    output.set_run_no_overflow(true)?;
    writeln!(output, "{}", lines)?;
    minus::page_all(output)?;
    Ok(())
}

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
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "logs")
        );
        exit(1);
    }
    let no_pager = opts.contains_key("no-pager") || env::var("NOPAGER").eq(&Ok("true".to_string()));
    let use_minus =
        opts.contains_key("use-builtin-pager") || env::var("PAGER").eq(&Ok(String::new()));

    // Connect to WebSocket over Unix socket
    let socket = connect_to_server_console(args[1].clone())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });

    // FIXME: Implement proper `console` command, everything so far is just copy paste from `logs`..
    // Split the socket and then read a single message from it
    let (write, read) = socket.split();
    let (item, read) = read.into_future().await;
    let item = item
        .unwrap_or_else(|| {
            println!("Error: Received no message from Octyne!");
            exit(1);
        })
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    if item.is_close() {
        println!("Error: Received close message from Octyne!");
        exit(1);
    }
    let item = item.to_text().unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });

    // Close the WebSocket connection.
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

    // Log the output.
    if no_pager {
        return println!("{}", item);
    }
    #[cfg(target_family = "unix")]
    if !use_minus {
        pager::Pager::with_default_pager("less").setup();
        println!("{}", item);
        exit(0);
    }
    minus_page_lines(item).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });
}

pub fn console_cmd_help() {
    println!(
        "Interact with an app's console and send input.

If you only want the app's output logs, and don't want to send any input to it,
use the `logs` command instead.

Usage: octynectl console [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information
    --no-interactive         Don't setup an interactive console for an end user,
                             just accept stdin and log output to stdout"
    );
}
