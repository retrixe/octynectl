pub const USAGE: &str = "{0}, run `octynectl help{1}` for more information.";

pub const INCORRECT_USAGE: &str = "Incorrect usage";

const UNKNOWN_SUBCOMMAND: &str = "Unknown subcommand";

pub fn unknown_subcommand(subcommand: &str) -> String {
    if subcommand.is_empty() {
        return UNKNOWN_SUBCOMMAND.to_string();
    }
    format!("{}: {}", UNKNOWN_SUBCOMMAND, subcommand)
}

pub fn invalid_usage(msg: &str, subcommand: &str) -> String {
    if subcommand.is_empty() {
        return USAGE.replace("{0}", msg).replace("{1}", "");
    }
    USAGE
        .replace("{0}", msg)
        .replace("{1}", (" ".to_owned() + subcommand).as_str())
}

pub const HELP_STR: &str = "Command-line interface to control Octyne.

Usage: octynectl [OPTIONS] [SUBCOMMAND]

Options:
    -v, --version            Print version info and exit
    -h, --help               Print help information

Subcommands:
    list, list-apps, apps    List all apps (WIP)
    start                    Start an app (WIP)
    stop                     Stop an app (WIP)
    kill                     Kill an app (WIP)
    restart                  Restart an app (WIP)
    status                   Get the status of an app (WIP)
    logs                     Get the logs of an app (WIP)
    help                     Print this help message and exit
";
