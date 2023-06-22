use std::{collections::HashMap, process::exit};

use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde_json::{Map, Value};

use crate::utils::misc;

// TODO: cleanup with serde::Deserialize struct like start/stop/kill
// TODO: add --format=json/csv flag, a flag to filter results would be neat too
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

    let url = Uri::new(misc::default_octyne_path(), "/servers").into();
    let client = Client::unix();
    let response = client.get(url).await;
    let (res, body) = crate::utils::request::read_str_or_exit(response).await;

    let parsed: Map<String, Value> = serde_json::from_str(body.trim()).unwrap_or_else(|e| {
        println!("Error: Received corrupt response from Octyne! {}", e);
        exit(1);
    });

    if res.status() != 200 {
        let default = format!(
            "Error: Received error status code {} from Octyne!",
            res.status().as_str()
        );
        let error = parsed.get("error");
        match error {
            Some(value) => println!("Error: {}", value.as_str().unwrap_or(default.as_str())),
            None => println!("{}", default),
        }
        exit(1);
    }

    let servers = parsed
        .get("servers")
        .unwrap_or_else(|| {
            println!("Error: Received corrupt response from Octyne!");
            exit(1);
        })
        .as_object()
        .unwrap_or_else(|| {
            println!("Error: Received corrupt response from Octyne!");
            exit(1);
        });

    if servers.is_empty() {
        println!("No apps are running under the local Octyne instance.");
        return;
    }

    // TODO: a table would look nice here
    println!("Apps running under the local Octyne instance:\n");
    let longest_name = servers.keys().map(|s| s.len()).max().unwrap_or(0);
    for server in servers {
        let (name, status_value) = server;
        let status = status_to_text(status_value.as_i64().unwrap_or(-1));

        println!("    {}{} - {}", name, pad_name(name, longest_name), status);
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
    -h, --help               Print help information"
    );
}
