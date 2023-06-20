use std::process::exit;

mod commands;
mod help;
mod utils;

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
                match subcommand {
                    "list" | "list-apps" | "apps" => crate::commands::list::list_cmd_help(),
                    "start" => crate::commands::start::start_cmd_help(),
                    "stop" => crate::commands::stop::stop_cmd_help(),
                    "kill" => crate::commands::kill::kill_cmd_help(),
                    "restart" => println!("Not implemented yet!"), // TODO
                    "status" => println!("Not implemented yet!"),  // TODO
                    "logs" => println!("Not implemented yet!"),    // TODO
                    "console" => println!("Not implemented yet!"), // TODO
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
        "restart" => println!("Not implemented yet."), // TODO
        "status" => println!("Not implemented yet."),  // TODO
        "logs" => println!("Not implemented yet."),    // TODO
        "console" => println!("Not implemented yet."), // TODO
        _ => {
            println!(
                "{}",
                help::invalid_usage(help::unknown_subcommand(subcommand).as_str(), "")
            );
            exit(1);
        }
    }
}
