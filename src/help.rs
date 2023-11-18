use std::process::exit;

pub const USAGE: &str = "{0}, run `octynectl help{1}` for more information.";

pub const INCORRECT_USAGE: &str = "Incorrect usage";

pub const TOO_MANY_ARGS: &str = "Too many arguments";

const UNKNOWN_SUBCOMMAND: &str = "Unknown subcommand";

pub fn unknown_subcommand(subcommand: &str) -> String {
    unknown_subcommand_str(subcommand.to_owned())
}

pub fn unknown_subcommand_str(subcommand: String) -> String {
    if subcommand.is_empty() {
        return UNKNOWN_SUBCOMMAND.to_string();
    }
    format!("{}: {}", UNKNOWN_SUBCOMMAND, subcommand)
}

pub fn invalid_usage(msg: &str, subcommand: &str) -> String {
    invalid_usage_str(msg.to_owned(), subcommand.to_owned())
}

pub fn invalid_usage_str(msg: String, subcommand: String) -> String {
    if subcommand.is_empty() {
        return USAGE.replace("{0}", &msg).replace("{1}", "");
    }
    USAGE
        .replace("{0}", &msg)
        .replace("{1}", (" ".to_owned() + &subcommand).as_str())
}

pub fn log_too_many_args(subcommand: String) {
    println!(
        "{}",
        invalid_usage_str(TOO_MANY_ARGS.to_string(), subcommand)
    );
}

// TODO: eventually have `nodes` and --node=NAME
pub const HELP_STR: &str = "Command-line interface to control Octyne.
This connects to your local Octyne instance over Unix socket, and lets you view
and control applications running under it.

Usage: octynectl [OPTIONS] [SUBCOMMAND]

Options:
    -v, --version            Print version info and exit
    -h, --help               Print help information

Subcommands:
    list, list-apps, apps    List all apps under Octyne
    start                    Start an app managed by Octyne
    stop                     Gracefully stop an app managed by Octyne
    kill                     Kill an app managed by Octyne
    restart                  Restart an app
    status                   Get the status of an app
    logs                     Get the output logs of an app
    console                  Open an app's console (NOT YET IMPLEMENTED)
    config                   Edit/view/reload Octyne's config (`help config` for more info)
    account(s), user(s)      Manage Octyne accounts (`help accounts` for more info)
    help                     Print this help message and exit
";

pub fn help_cmd(args: Vec<String>) {
    if args.len() > 1 {
        let subcommand_tmp = args[1].clone();
        let subcommand = subcommand_tmp.as_str();
        let first_level = vec![
            (
                "list,list-apps,apps",
                crate::commands::list::list_cmd_help as fn(),
            ),
            ("start", crate::commands::start::start_cmd_help as fn()),
            ("stop", crate::commands::stop::stop_cmd_help as fn()),
            ("kill", crate::commands::kill::kill_cmd_help as fn()),
            (
                "restart",
                crate::commands::restart::restart_cmd_help as fn(),
            ),
            (
                "status,info",
                crate::commands::status::status_cmd_help as fn(),
            ),
            ("logs", crate::commands::logs::logs_cmd_help as fn()),
            // FIXME: ("console", crate::commands::console::console_cmd_help),
            ("config", crate::commands::config::config_cmd_help as fn()),
            (
                "account,accounts",
                crate::commands::accounts::accounts_cmd_help as fn(),
            ),
        ];
        if args.len() == 2 {
            for (aliases, help) in first_level.iter() {
                for alias in aliases.split(',') {
                    if subcommand == alias {
                        return help();
                    }
                }
            }
            println!(
                "{}",
                invalid_usage(unknown_subcommand(subcommand).as_str(), "")
            );
            exit(1);
        }
        // For subsequent levels, we will still rely on custom logic for now.
        match subcommand {
            "config" => {
                if args.len() > 3 {
                    log_too_many_args(args[1].clone());
                } else if args[2] == "view" || args[2] == "show" {
                    crate::commands::config::config_view_cmd_help();
                } else if args[2] == "edit" || args[2] == "modify" {
                    crate::commands::config::config_edit_cmd_help();
                } else if args[2] == "reload" {
                    crate::commands::config::config_reload_cmd_help();
                } else {
                    println!(
                        "{}",
                        invalid_usage_str(
                            unknown_subcommand_str(subcommand.to_owned() + " " + &args[2]),
                            args[1].clone()
                        )
                    );
                }
            }
            "account" | "accounts" => {
                if args.len() > 3 {
                    log_too_many_args(args[1].clone());
                } else if args[2] == "list" || args[2] == "show" {
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
                        invalid_usage_str(
                            unknown_subcommand_str(subcommand.to_owned() + " " + &args[2]),
                            args[1].clone()
                        )
                    );
                }
            }
            _ => {
                println!(
                    "{}",
                    invalid_usage(unknown_subcommand(subcommand).as_str(), "")
                );
                exit(1);
            }
        }
        return;
    }
    println!("{}", HELP_STR)
}
