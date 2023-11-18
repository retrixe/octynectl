use std::process::exit;

mod api;
mod commands;
mod help;
mod utils;

// TODO: we need man pages down the line
// TODO: the opt parser is primitive, only allow certain opts to be parsed, add strings to opt
// FIXME: error handling is a bit aggressive, use ?
#[tokio::main]
async fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    // Parse top-level options.
    let top_level_opts = crate::utils::options::parse_options(&mut args, true);
    if top_level_opts.contains_key("v") || top_level_opts.contains_key("version") {
        println!("octynectl {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Check for commands.
    if args.is_empty() {
        if top_level_opts.contains_key("h") || top_level_opts.contains_key("help") {
            println!("{}", help::HELP_STR);
        } else {
            println!("{}", help::invalid_usage(help::INCORRECT_USAGE, ""));
            exit(1);
        }
        return;
    }

    // Parse subcommand.
    let subcommand_tmp = args[0].clone();
    let subcommand = subcommand_tmp.as_str();
    match subcommand {
        "help" => {
            if args.len() > 1 {
                let subcommand_tmp = args[1].clone();
                let subcommand = subcommand_tmp.as_str();
                // FIXME: Everything except config/accounts should be limited to 1 param only!
                match subcommand {
                    "list" | "list-apps" | "apps" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::list::list_cmd_help()
                    }
                    "start" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::start::start_cmd_help()
                    }
                    "stop" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::stop::stop_cmd_help()
                    }
                    "kill" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::kill::kill_cmd_help()
                    }
                    "restart" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::restart::restart_cmd_help()
                    }
                    "status" | "info" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::status::status_cmd_help()
                    }
                    "logs" => {
                        if args.len() != 2 {
                            return help::log_too_many_args(args[1].clone());
                        }
                        crate::commands::logs::logs_cmd_help()
                    }
                    "console" => println!("Not implemented yet!"), // FIXME
                    "config" => {
                        if args.len() == 2 {
                            crate::commands::config::config_cmd_help();
                        } else if args.len() == 3 {
                            if args[2] == "view" || args[2] == "show" {
                                crate::commands::config::config_view_cmd_help();
                            } else if args[2] == "edit" || args[2] == "modify" {
                                crate::commands::config::config_edit_cmd_help();
                            } else if args[2] == "reload" {
                                crate::commands::config::config_reload_cmd_help();
                            } else {
                                println!(
                                    "{}",
                                    help::invalid_usage_str(
                                        help::unknown_subcommand_str(
                                            subcommand.to_owned() + " " + &args[2]
                                        ),
                                        args[1].clone()
                                    )
                                );
                            }
                        } else {
                            help::log_too_many_args(args[1].clone());
                        }
                    }
                    "account" | "accounts" => {
                        if args.len() == 2 {
                            crate::commands::accounts::accounts_cmd_help();
                        } else if args.len() == 3 {
                            if args[2] == "list" || args[2] == "show" {
                                crate::commands::accounts::accounts_list_cmd_help();
                            } else if args[2] == "create" || args[2] == "add" {
                                crate::commands::accounts::accounts_create_cmd_help();
                            } else if args[2] == "delete" || args[2] == "remove" {
                                crate::commands::accounts::accounts_delete_cmd_help();
                            } else if args[2] == "rename" {
                                crate::commands::accounts::accounts_rename_cmd_help();
                            } else if args[2] == "passwd" {
                                crate::commands::accounts::accounts_passwd_cmd_help();
                            } else {
                                println!(
                                    "{}",
                                    help::invalid_usage_str(
                                        help::unknown_subcommand_str(
                                            subcommand.to_owned() + " " + &args[2]
                                        ),
                                        args[1].clone()
                                    )
                                );
                            }
                        } else {
                            help::log_too_many_args(args[1].clone());
                        }
                    }
                    _ => {
                        println!(
                            "{}",
                            help::invalid_usage(help::unknown_subcommand(subcommand).as_str(), "")
                        );
                        exit(1);
                    }
                }
                return;
            }
            println!("{}", help::HELP_STR)
        }
        "list" | "list-apps" | "apps" => {
            crate::commands::list::list_cmd(args, top_level_opts).await
        }
        "start" => crate::commands::start::start_cmd(args, top_level_opts).await,
        "stop" => crate::commands::stop::stop_cmd(args, top_level_opts).await,
        "kill" => crate::commands::kill::kill_cmd(args, top_level_opts).await,
        "restart" => crate::commands::restart::restart_cmd(args, top_level_opts).await,
        "status" | "info" => crate::commands::status::status_cmd(args, top_level_opts).await,
        "logs" => crate::commands::logs::logs_cmd(args, top_level_opts).await,
        "console" => println!("Not implemented yet."), // FIXME
        "config" => crate::commands::config::config_cmd(args, top_level_opts).await,
        "account" | "accounts" => {
            crate::commands::accounts::accounts_cmd(args, top_level_opts).await
        }
        _ => {
            println!(
                "{}",
                help::invalid_usage(help::unknown_subcommand(subcommand).as_str(), "")
            );
            exit(1);
        }
    }
}
