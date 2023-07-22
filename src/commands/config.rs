use std::{collections::HashMap, path::Path, process::exit};

use pathsearch::find_executable_in_path;
use tempfile::NamedTempFile;

use crate::api::config::{get_config, get_config_reload};

pub async fn config_cmd(args: Vec<String>, top_level_opts: HashMap<String, String>) {
    let mut args = args.clone();
    let opts = crate::utils::options::parse_options(&mut args, false);
    if args.is_empty()
        && (top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help"))
    {
        config_cmd_help();
    } else if args.is_empty() {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "config")
        );
        exit(1);
    } else if args.len() == 1 {
        config_cmd_help();
    } else if args[1] == "view" || args[1] == "show" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            config_view_cmd_help();
            return;
        } else if args.len() != 2 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "config view")
            );
            exit(1);
        }

        let config = get_config().await.unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(1);
        });
        println!("{}", config.trim_end());
    } else if args[1] == "edit" || args[1] == "modify" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            config_edit_cmd_help();
            return;
        } else if args.len() != 2 && args.len() != 3 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "config edit")
            );
            exit(1);
        }

        if args.len() == 3 {
            let path = Path::new(&args[2]);
            if path.is_dir() {
                println!("Error: {} is a directory!", path.display());
                exit(1);
            } else if !path.is_file() {
                println!(
                    "Error: {} cannot be accessed! Does it exist?",
                    path.display()
                );
                exit(1);
            }
            let config = match std::fs::read_to_string(path) {
                Ok(config) => config,
                Err(e) => {
                    println!("Error: {}", e);
                    exit(1);
                }
            };
            match crate::api::config::patch_config(config).await {
                Ok(_) => println!(
                    "Successfully saved new config copied from file: {}!",
                    args[2]
                ),
                Err(err) => {
                    println!("Error: {}", err);
                    exit(1);
                }
            }
        } else {
            let editor = match std::env::var("EDITOR") {
                Ok(editor) => editor,
                Err(_) => find_executable_in_path("nano")
                    .or_else(|| find_executable_in_path("vi"))
                    .or_else(|| find_executable_in_path("notepad.exe"))
                    .unwrap_or_else(|| {
                        println!(
                            "Error: No editor found! Please set $EDITOR to your preferred editor."
                        );
                        exit(1);
                    })
                    .to_str()
                    .unwrap()
                    .to_owned(),
            };
            let config = match get_config().await {
                Ok(config) => config,
                Err(e) => {
                    println!("Error retrieving config: {}", e);
                    exit(1);
                }
            };
            let temp_file = NamedTempFile::new().unwrap_or_else(|e| {
                println!("Error creating temp file: {}", e);
                exit(1);
            });
            let temp_file_path = temp_file.path().to_owned();
            match std::fs::write(temp_file_path.clone(), config.clone()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error writing config to temp file: {}", e);
                    exit(1);
                }
            };
            let status = match std::process::Command::new(editor)
                .arg(temp_file_path.clone())
                .status()
            {
                Ok(status) => status,
                Err(e) => {
                    println!("Error opening editor: {}", e);
                    exit(1);
                }
            };
            if !status.success() {
                println!("Error: Failed to open editor!");
                exit(1);
            }
            let new_config = match std::fs::read_to_string(temp_file_path.clone()) {
                Ok(new_config) => new_config,
                Err(e) => {
                    println!("Error reading temp file: {}", e);
                    exit(1);
                }
            };
            if new_config == config {
                println!("No changes made to config! Exiting...");
            } else {
                match crate::api::config::patch_config(new_config).await {
                    Ok(_) => println!("Successfully saved new config!"),
                    Err(err) => {
                        println!("Error loading config: {}", err);
                        exit(1);
                    }
                };
            }
            match std::fs::remove_file(temp_file_path.clone()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error removing temp file: {}", e);
                    exit(1);
                }
            };
        }
    } else if args[1] == "reload" {
        if top_level_opts.contains_key("h")
            || top_level_opts.contains_key("help")
            || opts.contains_key("h")
            || opts.contains_key("help")
        {
            config_reload_cmd_help();
            return;
        } else if args.len() != 2 {
            println!(
                "{}",
                crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "config reload")
            );
            exit(1);
        }

        match get_config_reload().await {
            Ok(_) => println!("Successfully reloaded config!"),
            Err(err) => {
                println!("Error: {}", err);
                exit(1);
            }
        }
    } else {
        println!(
            "{}",
            crate::help::invalid_usage(crate::help::INCORRECT_USAGE, "accounts")
        );
        exit(1);
    }
}

pub fn config_cmd_help() {
    println!(
        "Edit/view/reload Octyne's config.

Usage: octynectl config [OPTIONS] [SUBCOMMAND]

Subcommands:
    view, show           Show Octyne's config in terminal
    edit, modify         Modify Octyne's config in a text editor/read from disk
    reload               Have Octyne reload its config from disk

Options:
    -h, --help           Print help information"
    );
}

pub fn config_view_cmd_help() {
    println!(
        "Show Octyne's config in terminal.

Usage: octynectl config view [OPTIONS]

Aliases: show

Options:
    -h, --help           Print help information"
    );
}

pub fn config_edit_cmd_help() {
    println!(
        "Modify Octyne's config in a text editor.
If a file is specified, then the file will be used as the new config instead of opening text editor.
$EDITOR will be used to select the text editor, else octynectl will fallback to nano, vi or notepad.

Usage: octynectl config edit [OPTIONS] (FILE)

Aliases: modify

Options:
    -h, --help           Print help information"
    );
}

pub fn config_reload_cmd_help() {
    println!(
        "Have Octyne reload its config from disk.

Usage: octynectl config reload [OPTIONS]

Options:
    -h, --help           Print help information"
    );
}
