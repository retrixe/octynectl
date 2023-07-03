use std::{collections::HashMap, process::exit};

use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;

use crate::utils::misc;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    #[serde(default)]
    error: String,
    #[serde(default)]
    cpu_usage: f64,
    #[serde(default)]
    memory_usage: i64,
    #[serde(default)]
    total_memory: i64,
    #[serde(default)]
    uptime: i64,
}

// TODO: Support multiple apps down the line
pub async fn status_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        status_cmd_help();
        return;
    } else if args.len() != 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "status")
        );
        exit(1);
    }

    let endpoint = format!("/server/{}", args[1]);
    let client = Client::unix();
    let uri = Uri::new(misc::default_octyne_path(), endpoint.as_str()).into();
    let response = client.get(uri).await;
    let (res, body) = crate::utils::request::read_str_or_exit(response).await;

    let json: Response = serde_json::from_str(body.trim()).unwrap_or_else(|e| {
        println!("Error: Received corrupt response from Octyne! {}", e);
        exit(1);
    });

    if res.status() != 200 && json.error.is_empty() {
        println!(
            "Error: Received status code {} from Octyne!",
            res.status().as_str()
        );
        exit(1);
    } else if !json.error.is_empty() {
        println!("Error: {}", json.error);
        exit(1);
    }

    println!("\nStatus of app `{}`:", args[1]);
    println!("================={}", "=".repeat(args[1].len()));
    println!("CPU usage: {:.2}%", json.cpu_usage);
    let memory_usage = json.memory_usage as f64 / 1024.0 / 1024.0;
    let total_memory = json.total_memory as f64 / 1024.0 / 1024.0;
    let memory_percentage = memory_usage / total_memory * 100.0;
    println!(
        "Memory usage: {:.2}% ({:.2} MB / {:.0} MB)",
        memory_percentage, memory_usage, total_memory
    );
    println!("Uptime: {}", parse_duration(json.uptime));
}

pub fn status_cmd_help() {
    println!(
        "Get the status of an app.

Usage: octynectl status [OPTIONS] [APP NAME]

Aliases: info

Options:
    -h, --help               Print help information"
    );
}

// Taken from https://github.com/retrixe/ecthelion statistics page /dashboard/[server]
fn parse_duration(duration_nano: i64) -> String {
    let duration = duration_nano as f64 / 1000000.0; // Convert to milliseconds
    let (days, hours, minutes, seconds);
    days = f64::floor(duration / (24.0 * 60.0 * 60.0 * 1000.0));
    let leftover_hours = duration % (24.0 * 60.0 * 60.0 * 1000.0);
    hours = f64::floor(leftover_hours / (60.0 * 60.0 * 1000.0));
    let leftover_minutes = duration % (60.0 * 60.0 * 1000.0);
    minutes = f64::floor(leftover_minutes / (60.0 * 1000.0));
    let leftover_seconds = duration % (60.0 * 1000.0);
    seconds = f64::floor(leftover_seconds / 1000.0);

    let mut res = String::new();
    if days == 1.0 {
        res.push_str(&format!("{} day ", days));
    } else if days != 0.0 {
        res.push_str(&format!("{} days ", days));
    }
    if hours == 1.0 {
        res.push_str(&format!("{} hour ", hours));
    } else if hours != 0.0 {
        res.push_str(&format!("{} hours ", hours));
    }
    if minutes == 1.0 {
        res.push_str(&format!("{} minute ", minutes));
    } else if minutes != 0.0 {
        res.push_str(&format!("{} minutes ", minutes));
    }
    if seconds == 1.0 {
        res.push_str(&format!("{} second ", seconds));
    } else if seconds != 0.0 {
        res.push_str(&format!("{} seconds ", seconds));
    }
    res.trim_end().to_owned()
}
