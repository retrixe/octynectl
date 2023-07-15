use std::{collections::HashMap, process::exit};

use hyper::Client;
use hyperlocal_with_windows::{UnixClientExt, Uri};
use serde::Deserialize;
use serde_json::Value;

use crate::utils::misc;

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    #[serde(default)]
    error: String,
}

pub async fn accounts_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if args.len() == 0
        && (top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help"))
    {
        accounts_cmd_help();
        return;
    } else if args.len() == 0 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts")
        );
        exit(1);
    } else if args[1] == "list" || args[1] == "show" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            accounts_list_cmd_help();
            return;
        } else if args.len() != 2 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts list")
            );
            exit(1);
        }

        let url = Uri::new(misc::default_octyne_path(), "/accounts").into();
        let client = Client::unix();
        let response = client.get(url).await;
        let (res, body) = crate::utils::request::read_str(response)
            .await
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });

        let json: Value = serde_json::from_str(body.trim()).unwrap_or_else(|e| {
            println!("Error: Received corrupt response from Octyne! {}", e);
            exit(1);
        });

        if json.is_object() {
            let resp: ErrorResponse = serde_json::from_value(json).unwrap_or_else(|_| {
                println!("Error: Received corrupt response from Octyne!");
                exit(1);
            });
            if resp.error.is_empty() {
                println!("Error: Received corrupt response from Octyne!");
            } else {
                println!("Error: {}", resp.error);
            }
            exit(1);
        } else if res.status() != 200 {
            let default = format!(
                "Error: Received status code {} from Octyne!",
                res.status().as_str()
            );
            println!("{}", default);
            exit(1);
        }

        let accounts: Vec<String> = serde_json::from_value(json).unwrap_or_else(|e| {
            println!("Error: Received corrupt response from Octyne! {}", e);
            exit(1);
        });

        if accounts.is_empty() {
            println!("The local Octyne instance has no accounts!");
            return;
        }

        println!("Accounts registered with the local Octyne instance:");
        for account in accounts {
            println!("{}", account);
        }
    } else if args[1] == "create" || args[1] == "add" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            accounts_create_cmd_help();
            return;
        } else if args.len() != 3 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts create")
            );
            exit(1);
        }

        // TODO: Prompt for password, then POST, then log "Successfully created account {}"
    } else if args[1] == "delete" || args[1] == "remove" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            accounts_delete_cmd_help();
            return;
        } else if args.len() != 3 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts delete")
            );
            exit(1);
        }

        // TODO: Delete the account, then log "Successfully deleted account {}"
    } else if args[1] == "passwd" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            accounts_passwd_cmd_help();
            return;
        } else if args.len() != 3 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts passwd")
            );
            exit(1);
        }

        // TODO: Prompt for password, then POST, then log "Successfully changed password for account {}"
    } else {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts")
        );
        exit(1);
    }
}

pub fn accounts_cmd_help() {
    println!(
        "Manage Octyne accounts.

Usage: octynectl accounts [OPTIONS] [SUBCOMMAND]

Aliases: account

Subcommands:
    list, show           List all accounts
    create, add          Create a new account
    delete, remove       Delete an account
    passwd               Change password of an existing account

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_list_cmd_help() {
    println!(
        "List all Octyne accounts.

Usage: octynectl accounts list [OPTIONS]

Aliases: show

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_create_cmd_help() {
    println!(
        "Create a new Octyne account. You will be prompted for a password.

Usage: octynectl accounts create [OPTIONS]

Aliases: add

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_delete_cmd_help() {
    println!(
        "Delete an Octyne account.

Usage: octynectl accounts create [OPTIONS]

Aliases: remove

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_passwd_cmd_help() {
    println!(
        "Change the password of an existing Octyne account.

Usage: octynectl accounts passwd [OPTIONS]

Options:
    -h, --help           Print help information"
    );
}
