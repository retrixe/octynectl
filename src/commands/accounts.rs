use std::{collections::HashMap, process::exit};

use crate::api::accounts::{create_account, delete_account, get_accounts, patch_account};

pub async fn accounts_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if args.is_empty()
        && (top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help"))
    {
        accounts_cmd_help();
    } else if args.is_empty() {
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

        let accounts = get_accounts().await.unwrap_or_else(|e| {
            println!("Error: {}", e);
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

        let pass = match rpassword::prompt_password("New password for account: ") {
            Ok(pass) => pass,
            Err(err) => {
                println!("Error: Failed to read password! {}", err);
                exit(1);
            }
        };
        match rpassword::prompt_password("Confirm password: ") {
            Ok(confirm_pass) => {
                if confirm_pass != pass {
                    println!("Error: Passwords do not match!");
                    exit(1);
                }
            }
            Err(err) => {
                println!("Error: Failed to read password! {}", err);
                exit(1);
            }
        };
        create_account(args[2].to_owned(), pass)
            .await
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });
        println!("Successfully created account {}", args[2]);
    } else if args[1] == "delete" || args[1] == "remove" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            accounts_delete_cmd_help();
            return;
        } else if args.len() < 3 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts delete")
            );
            exit(1);
        }

        let mut any_errored = false;
        for username in args[2..].iter() {
            match delete_account(username.to_string()).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error deleting account {}: {}", username, e);
                    any_errored = true;
                }
            }
        }
        if any_errored {
            exit(1);
        } else {
            println!("Successfully deleted all accounts!");
        }
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

        let pass = match rpassword::prompt_password("New password for account: ") {
            Ok(pass) => pass,
            Err(err) => {
                println!("Error: Failed to read password! {}", err);
                exit(1);
            }
        };
        match rpassword::prompt_password("Confirm password: ") {
            Ok(confirm_pass) => {
                if confirm_pass != pass {
                    println!("Error: Passwords do not match!");
                    exit(1);
                }
            }
            Err(err) => {
                println!("Error: Failed to read password! {}", err);
                exit(1);
            }
        };
        patch_account(args[2].to_owned(), pass)
            .await
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                exit(1);
            });
        println!("Successfully changed password for account {}", args[2]);
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

Aliases: account, users, user

Subcommands:
    list, show           List all accounts
    create, add          Create a new account
    delete, remove       Delete accounts
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

Usage: octynectl accounts create [OPTIONS] [USERNAME]

Aliases: add

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_delete_cmd_help() {
    println!(
        "Delete Octyne accounts.

Usage: octynectl accounts delete [OPTIONS] [USERNAMES...]

Aliases: remove

Options:
    -h, --help           Print help information"
    );
}

pub fn accounts_passwd_cmd_help() {
    println!(
        "Change the password of an existing Octyne account.

Usage: octynectl accounts passwd [OPTIONS] [USERNAME]

Options:
    -h, --help           Print help information"
    );
}
