use std::{collections::HashMap, process::exit};

use crate::api::version::get_version;

pub async fn version_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        version_cmd_help();
        return;
    } else if args.len() != 1 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "version")
        );
        exit(1);
    }

    println!("octynectl version {}", env!("CARGO_PKG_VERSION"));
    match get_version().await {
        Ok(version) => {
            println!("octyne version {}", version);
        }
        Err(e) => {
            println!("failed to retrieve octyne version: {}", e);
        }
    };
}

pub fn version_cmd_help() {
    println!(
        "Get the version of octynectl and octyne.

Usage: octynectl version [OPTIONS]

Aliases: info

Options:
    -h, --help               Print help information"
    );
}
