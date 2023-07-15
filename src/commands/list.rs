use std::{collections::HashMap, process::exit};

use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::utils::misc;

// TODO: Move API calls into api/servers.rs file
#[derive(Deserialize, Debug)]
struct Response {
    #[serde(default)]
    servers: Map<String, Value>,
    #[serde(default)]
    error: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerExtraInfo {
    #[serde(default)]
    online: i64,
    #[serde(default)]
    to_delete: bool,
}

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

    let url = Uri::new(misc::default_octyne_path(), "/servers?extrainfo=true").into();
    let client = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str(response)
        .await
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });

    let json: Response = serde_json::from_str(body.trim()).unwrap_or_else(|e| {
        println!("Error: Received corrupt response from Octyne! {}", e);
        exit(1);
    });

    if !json.error.is_empty() {
        println!("Error: {}", json.error);
        exit(1);
    } else if res.status() != 200 {
        let default = format!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        );
        println!("{}", default);
        exit(1);
    }

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&json.servers).unwrap());
        return;
    } else if format == "csv" {
        println!("name,status,toDelete");
        for server in json.servers {
            let (name, status_value) = server;
            let status = parse_status_value(status_value);
            let online_status = status_to_text(status.online).to_lowercase();
            println!("{},{},{}", name, online_status, status.to_delete);
        }
        return;
    }

    if json.servers.is_empty() {
        println!("No apps are running under the local Octyne instance.");
        return;
    }

    println!("Apps running under the local Octyne instance:\n");
    let longest_name = json.servers.keys().map(|s| s.len()).max().unwrap_or(0);
    for server in json.servers {
        let (name, status_value) = server;
        let status = parse_status_value(status_value);
        let mut info = status_to_text(status.online);
        if status.to_delete {
            info += " (marked for deletion)";
        }

        println!("    {}{} | {}", name, pad_name(&name, longest_name), info);
    }
}

fn parse_status_value(status_value: Value) -> ServerExtraInfo {
    match status_value.as_i64() {
        Some(status) => {
            ServerExtraInfo {
                online: status,
                to_delete: false,
            }
        }
        None => {
            serde_json::from_value(status_value).unwrap_or(ServerExtraInfo {
                online: -1,
                to_delete: false,
            })
        }
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
