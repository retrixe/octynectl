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
    USAGE.replace("{0}", msg).replace("{1}", subcommand)
}

pub const HELP_STR: &str = "Command-line interface to control Octyne.

Usage: octynectl [OPTIONS] [SUBCOMMAND]

Options:
    -v, --version        Print version info and exit
    -h, --help           Print help information

Subcommands:
    list, list-servers    List all servers (WIP)
    start                 Start a server (WIP)
    stop                  Stop a server (WIP)
    kill                  Kill a server (WIP)
    restart               Restart a server (WIP)
    status                Get the status of a server (WIP)
    logs                  Get the logs of a server (WIP)
    help                  Print this help message and exit
";
