use std::{path::PathBuf, process};

use clap::{Parser};

mod sync;
mod config;
mod utils;

const IDM_VERSION: &str = "2.0.0";

#[derive(Parser, Debug)]
struct Args {
    // Load a specific config file
#[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    // Get current idm version
#[arg(short, long)]
    version: bool,

    action: String,
}

fn main() {
    let args = Args::parse();

    // Print version
    if args.version {
        println!("idm {}", IDM_VERSION);
        return;
    }

    // Optionally set alternative path
    let config_path = if let Some(config_path) = args.config {
        config_path
    } else {
        // TODO: Follow best practice in this regard using XDG variables probably
        PathBuf::from("/home/ideale/.config/idm/idm.json")
    };

    // Verify config validity
    let config_struct = match config::open_config_path(&config_path) {
        Ok(mut config_file) => {
            config::config_to_struct(&mut config_file).unwrap_or_else(|err_msg| {
                eprintln!("{err_msg}");
                process::exit(1);
            })
        },
        Err(err_msg) => {
            eprintln!("{err_msg}");
            process::exit(1);
        },
    };

    match args.action.as_str() {
        "sync" => sync::handle_sync(&config_struct),
        _ => {
            eprintln!("idm: Error, unrecognized command!");
            process::exit(1);
        }
    }
}
