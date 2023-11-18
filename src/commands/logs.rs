use std::fmt::Write;
use std::{collections::HashMap, env, process::exit};

use crate::api::server::connect_to_server_console;
#[cfg(target_family = "unix")]
use futures_util::StreamExt;
use minus::MinusError;
#[cfg(target_family = "unix")]
use pager::Pager;
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

fn minus_page_lines(lines: &str) -> Result<(), MinusError> {
    let mut output = minus::Pager::new();
    output.set_run_no_overflow(true)?;
    writeln!(output, "{}", lines)?;
    minus::page_all(output)?;
    Ok(())
}

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
    let use_minus =
        opts.contains_key("use-builtin-pager") || env::var("PAGER").eq(&Ok(String::new()));

    // Connect to WebSocket over Unix socket
    #[allow(unused_mut)] // Windows needs this.
    let mut socket = connect_to_server_console(args[1].clone())
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });

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
    // Everything here onwards is OS-independent.
    let item = item.unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(1);
    });
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
        return println!("{}", item);
    } else if use_minus || cfg!(target_family = "windows") {
        return minus_page_lines(item).unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
    }
    #[cfg(target_family = "unix")]
    {
        Pager::with_default_pager("less").setup();
        println!("{}", item);
        exit(0);
    }
}

pub fn logs_cmd_help() {
    println!(
        "Get the output logs of an app.

If $PAGER is set, it will be used to display the logs, else, less will be used.
On Windows, a built-in pager library will be used, even if $PAGER is set. You
can use it on Unix-like systems too (e.g. Linux, macOS) by passing the
`--use-builtin-pager` flag, or setting the $PAGER env variable to empty string.

The pager can be disabled entirely by setting the $NOPAGER environment variable
to `true`, or by using the `--no-pager` flag.

Usage: octynectl logs [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information
    --no-pager               Don't use a pager to display logs
    --use-builtin-pager      Use the built-in pager to display logs"
    );
}
