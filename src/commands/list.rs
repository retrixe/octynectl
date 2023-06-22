use std::{collections::HashMap, process::exit};

use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::utils::misc;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(default)]
    servers: Map<String, Value>,
    #[serde(default)]
    error: String,
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

    let url = Uri::new(misc::default_octyne_path(), "/servers").into();
    let client = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str_or_exit(response).await;

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
        println!("name,status");
        for server in json.servers {
            let (name, status_value) = server;
            let status = status_to_text(status_value.as_i64().unwrap_or(-1)).to_lowercase();
            println!("{},{}", name, status);
        }
        return;
    }

    if json.servers.is_empty() {
        println!("No apps are running under the local Octyne instance.");
        return;
    }

    // TODO: a table would look nice here
    println!("Apps running under the local Octyne instance:\n");
    let longest_name = json.servers.keys().map(|s| s.len()).max().unwrap_or(0);
    for server in json.servers {
        let (name, status_value) = server;
        let status = status_to_text(status_value.as_i64().unwrap_or(-1));

        println!("    {}{} | {}", name, pad_name(&name, longest_name), status);
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
