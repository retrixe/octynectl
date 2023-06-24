use std::{collections::HashMap, process::exit};

use hyper::{Body, Client, Method, Request};
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;

use crate::utils::misc;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    error: String,
}

pub async fn stop_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if top_level_opts.contains_key("h")
        || top_level_opts.contains_key("help")
        || opts.contains_key("h")
        || opts.contains_key("help")
    {
        stop_cmd_help();
        return;
    } else if args.len() != 2 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "stop")
        );
        exit(1);
    }

    let endpoint = format!("/server/{}", args[1]);
    let client = Client::unix();
    let req = Request::builder()
        .method(Method::POST)
        .uri(Uri::new(misc::default_octyne_path(), endpoint.as_str()))
        .body(Body::from("TERM"))
        .expect("request builder");
    let response = client.request(req).await;
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
    } else if !json.success {
        println!("Error: Octyne failed to stop the app!");
        exit(1);
    }
}

pub fn stop_cmd_help() {
    println!(
        "Gracefully stop an app managed by Octyne.

Usage: octynectl stop [OPTIONS] [APP NAME]

Options:
    -h, --help               Print help information"
    );
}