#![allow(dead_code)]

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate term;
extern crate toml;
extern crate clap;
extern crate zip;

use std::path::Path;
use std::path::PathBuf;
use std::process::{Command};
use clap::{Arg, App, SubCommand};

pub mod config;
pub mod shell;
pub mod termcmd;
pub mod utils;
pub mod commands;

#[derive(Debug)]
enum ManifestLoadError {
    CargoLoadFailure,
    MissingPath
}

type ManifestResult = Result<PathBuf, ManifestLoadError>;

fn main() {
    // Initialize the environment logger only once.
    // This should be the first thing to happen.
    if let Err(e) = env_logger::init() {
        error!("Failed to initialise environment logger because {}", e);
    }

    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const DESC: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

    let mut app = App::new("Kinito")
        .version(VERSION.unwrap_or("unknown"))
        .about(DESC.unwrap_or("--"))

        .arg(Arg::with_name("project-dir")
             .long("project-dir")
             .short("d")
             .help("Override the default project directory provided by `cargo locate-project`")
             .takes_value(true))

        .arg(Arg::with_name("release")
             .long("release")
             .help("Build with cargo `release` mode optimizations"))

        .arg(Arg::with_name("clean")
             .long("clean")
             .help("Clean build artifacts for a fresh compilation"))

        .subcommand(SubCommand::with_name("build")
             .help("Compile Project into a APK\n"))

        .subcommand(SubCommand::with_name("device-install")
             .help("Install Android APK onto Device\n"));

    let matches = app.clone().get_matches();

    // Fetching the configuration for the build.
    let mut config = config::load(&match matches.value_of("project-dir") {
        Some(p) => PathBuf::from(p),
        None => { current_manifest_path().unwrap() }
    });

    config.release = matches.is_present("release");

    // Provide a way to clean the shells
    matches.is_present("clean") && shell::clean(&config);

    // Always check to see if we have the project shell embedded
    // Noticeably, this is done after clean has a chance to run.
    shell::embed_if_not_present(&config);

    if let Some(_) = matches.subcommand_matches("build") {
        commands::build(&config);
    }

    else if let Some(_) = matches.subcommand_matches("device-install") {
        commands::install(&config);
    }

    // If we have not matched any sub command at this point,
    // fallback to application help screen.
    else {
        if let Err(e) = app.print_help() {
            error!("Failed to print application help because {}", e);
        }
    }
}


/// Returns the path of the `Cargo.toml` that we want to build.
fn current_manifest_path() -> ManifestResult {
    let locate_result = Command::new("cargo").arg("locate-project").output();
    let output = match locate_result {
        Ok(out) => out,
        Err(e) => {
            print!("Failure: {:?}", e);
            panic!("Cargo failure to target project via locate-project")
        }
    };

    if !output.status.success() {
        return Err(ManifestLoadError::CargoLoadFailure);
    }

    #[derive(Deserialize)]
    struct Data { root: String }
    let stdout = String::from_utf8(output.stdout).unwrap();
    let decoded: Data = serde_json::from_str(&stdout).unwrap();
    let path = Path::new(&decoded.root).to_owned();

    // debug!("manifest: {:?}", path);
    Ok(path)
}
