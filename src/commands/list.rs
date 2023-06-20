use std::{collections::HashMap, process::exit};

pub async fn list_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    if top_level_opts.contains_key("h") || top_level_opts.contains_key("help") {
        list_cmd_help();
        return;
    } else if args.len() != 1 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "list")
        );
        exit(1);
    }

    println!("Not implemented yet."); // TODO
}

pub fn list_cmd_help() {
    println!(
        "List all apps running under this Octyne instance.

Usage: octynectl list-apps [OPTIONS]

Aliases: list, apps

Options:
    -h, --help               Print help information"
    );
}
