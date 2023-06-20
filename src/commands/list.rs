use std::collections::HashMap;

pub fn list_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    if top_level_opts.contains_key("h") || top_level_opts.contains_key("help") {
        list_cmd_help();
        return;
    } else if args.len() != 1 {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "list")
        );
        return;
    }

    println!("Not implemented yet."); // TODO
}

pub fn list_cmd_help() {
    println!("Not implemented yet."); // TODO
}
