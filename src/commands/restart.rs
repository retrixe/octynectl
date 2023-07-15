use std::{collections::HashMap, process::exit};

use crate::api::server::{post_server, PostServerAction};

pub async fn restart_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        restart_cmd_help();
        return;
    } else if args.len() < 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "start")
        );
        exit(1);
    }

    let action = match opts.contains_key("k") || opts.contains_key("kill") {
        true => PostServerAction::Kill,
        false => PostServerAction::Term,
    };

    let mut any_errored = false;
    for server_name in args[1..].iter() {
        match post_server(server_name.to_string(), action.clone()).await {
            Ok(_) => match post_server(server_name.to_string(), PostServerAction::Start).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error starting {} after restart: {}", server_name, e);
                    any_errored = true;
                }
            },
            Err(e) => {
                println!("Error stopping {} before restart: {}", server_name, e);
                any_errored = true;
            }
        }
    }
    if any_errored {
        exit(1);
    }
}

pub fn restart_cmd_help() {
    println!(
        "Restart an app managed by Octyne.

Usage: octynectl restart [OPTIONS] [APP NAMES...]

Options:
    -h, --help        Print help information
    -k, --kill        Kill the app instead of gracefully stopping it before restarting"
    );
}
