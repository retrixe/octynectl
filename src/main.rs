mod help;
mod options;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    // Parse top-level options.
    let top_level_opts_map = options::parse_options(&mut args, true);
    if top_level_opts_map.contains_key("v") || top_level_opts_map.contains_key("version") {
        println!("octynectl {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if top_level_opts_map.contains_key("h") || top_level_opts_map.contains_key("help") {
        // TODO: Send help for specific subcommands down the line?
        println!("{}", help::HELP_STR);
        return;
    }

    // Check for commands.
    if args.len() == 0 {
        println!("{}", help::invalid_usage(help::INCORRECT_USAGE, ""));
        return;
    }

    // Parse subcommand.
    let subcommand_tmp = args[0].clone();
    let subcommand = subcommand_tmp.as_str();
    match subcommand {
        "help" => {
            println!("{}", help::HELP_STR);
        },
        _ => {
            println!("{}", help::invalid_usage(help::unknown_subcommand(subcommand).as_str(), ""));
        }
    }
}
