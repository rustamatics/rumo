#![allow(dead_code)]

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate term;
extern crate toml;

extern crate clap;

use std::path::Path;
use std::path::PathBuf;
// use std::process::exit;
use std::process::Command;

use clap::{Arg, App, SubCommand};

mod build;
mod config;
mod install;
mod termcmd;

#[derive(Debug)]
enum ManifestLoadError {
    CargoLoadFailure,
    MissingPath
}
type ManifestResult = Result<PathBuf, ManifestLoadError>;

fn main() {
    // Provide a visual line break for development mode
    #[cfg(debug_assertions)]
    print!("\n\n");

    // Initialize the environment logger only once.
    // This should be the first thing to happen.
    if let Err(e) = env_logger::init() {
        error!("Failed to initialise environment logger because {}", e);
    }

    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const DESC: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

    let mut app = App::new("Cargo Android")
        .version(VERSION.unwrap_or("unknown"))
        .about(DESC.unwrap_or("--"))

        .arg(Arg::with_name("project-dir")
             .long("project-dir")
             .short("d")
             .takes_value(true))

        .arg(Arg::with_name("release")
             .long("release")
             .help("Trigger release build optimizations"))

        .arg(Arg::with_name("target")
             .long("target")
             .help("Specify target from multiple [[bin]] in Cargo.toml")
             .takes_value(true))

        .subcommand(SubCommand::with_name("build")
             .help("Compile Project into a APK\n"))

        .subcommand(SubCommand::with_name("install")
             .help("Install Android APK onto Device\n"));

    let matches = app.clone().get_matches();
    let current_manifest: PathBuf = match matches.value_of("project-dir") {
        Some(p) => PathBuf::from(p),
        None => { current_manifest_path().unwrap() }
    };

    debug!("Determined manifest file: {:?}", current_manifest);

    // // Fetching the configuration for the build.
    let mut config = config::load(&current_manifest);
    config.release = matches.is_present("release");

    if let Some(target) = matches.value_of("target") {
        config.target = Some(target.to_owned());
    }

    if let Some(sub) = matches.subcommand_matches("build") {
        debug!("build triggered")
        // build::build(&current_manifest, &config);
    }

    else if let Some(sub) = matches.subcommand_matches("install") {
        debug!("install triggered")
        // install::install(&current_manifest, &config);
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
    Ok(Path::new(&decoded.root).to_owned())
}
