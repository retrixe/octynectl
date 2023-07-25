use std::{collections::HashMap, process::exit};

use serde_json::Value;

use crate::api::servers::ServerExtraInfo;

// TODO: add --filter=glob
pub async fn list_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        list_cmd_help();
        return;
    } else if args.len() != 1 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "list")
        );
        exit(1);
    }

    // Check --format=json/csv/table flag
    let mut format = "table";
    if let Some(format_value) = opts.get("format") {
        if format_value != "json" && format_value != "csv" && format_value != "table" {
            println!(
                "Error: Invalid value for flag --format \"{}\"! (Valid values: json,csv,table)",
                format_value
            );
            exit(1);
        }
        format = format_value;
    }

    let servers = crate::api::servers::get_servers(true)
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&servers).unwrap());
        return;
    } else if format == "csv" {
        println!("name,status,toDelete");
        for server in servers {
            let (name, status_value) = server;
            let server_info = parse_server_info(status_value);
            let status = status_to_text(server_info.status).to_lowercase();
            println!("{},{},{}", name, status, server_info.to_delete);
        }
        return;
    }

    if servers.is_empty() {
        println!("No apps are running under the local Octyne instance.");
        return;
    }

    println!("Apps running under the local Octyne instance:\n");
    let longest_name = servers.keys().map(|s| s.len()).max().unwrap_or(0);
    for server in servers {
        let (name, server_info_value) = server;
        let server_info = parse_server_info(server_info_value);
        let mut info = status_to_text(server_info.status);
        if server_info.to_delete {
            info += " (marked for deletion)";
        }

        println!("    {}{} | {}", name, pad_name(&name, longest_name), info);
    }
}

fn parse_server_info(server_info_value: Value) -> ServerExtraInfo {
    match server_info_value.as_i64() {
        Some(status) => ServerExtraInfo {
            status: status,
            to_delete: false,
        },
        None => serde_json::from_value(server_info_value).unwrap_or(ServerExtraInfo {
            status: -1,
            to_delete: false,
        }),
    }
}

fn status_to_text(status: i64) -> String {
    match status {
        0 => "Offline",
        1 => "Online",
        2 => "Crashed",
        _ => "Unknown",
    }
    .to_string()
}

fn pad_name(name: &str, longest_name: usize) -> String {
    let mut padding = String::new();
    for _ in 0..(longest_name - name.len()) {
        padding.push(' ');
    }
    padding
}

pub fn list_cmd_help() {
    println!(
        "List all apps under Octyne.

Usage: octynectl list-apps [OPTIONS]

Aliases: list, apps

Options:
    -h, --help           Print help information
    --format=<format>    Format to print the list of apps in. Valid values: json,csv,table
                         Default: table"
    );
}
