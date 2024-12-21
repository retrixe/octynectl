use std::process::exit;

mod api;
mod commands;
mod help;
mod utils;

// TODO: we need man pages down the line
// TODO: the opt parser is primitive, only allow certain opts to be parsed, add strings to opt
#[tokio::main]
async fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    // Parse top-level options.
    let top_level_opts = crate::utils::options::parse_options(&mut args, true);
    if top_level_opts.contains_key("v") || top_level_opts.contains_key("version") {
        println!("octynectl version {}", env!("CARGO_PKG_VERSION"));
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
        "help" => crate::help::help_cmd(args),
        "list" | "list-apps" | "apps" => {
            crate::commands::list::list_cmd(args, top_level_opts).await
        }
        "start" => crate::commands::start::start_cmd(args, top_level_opts).await,
        "stop" => crate::commands::stop::stop_cmd(args, top_level_opts).await,
        "kill" => crate::commands::kill::kill_cmd(args, top_level_opts).await,
        "restart" => crate::commands::restart::restart_cmd(args, top_level_opts).await,
        "status" | "info" => crate::commands::status::status_cmd(args, top_level_opts).await,
        "logs" => crate::commands::logs::logs_cmd(args, top_level_opts).await,
        "console" => crate::commands::console::console_cmd(args, top_level_opts).await,
        "config" => crate::commands::config::config_cmd(args, top_level_opts).await,
        "account" | "accounts" => {
            crate::commands::accounts::accounts_cmd(args, top_level_opts).await
        }
        "version" => crate::commands::version::version_cmd(args, top_level_opts).await,
        _ => {
            println!(
                "{}",
                help::invalid_usage(help::unknown_subcommand(subcommand).as_str(), "")
            );
            exit(1);
        }
    }
}
