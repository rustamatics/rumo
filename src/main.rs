#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate term;
extern crate toml;

extern crate clap;

use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

use clap::{Arg, App, SubCommand};

mod build;
mod config;
mod install;
mod termcmd;

enum ManifestLoadError {
    CargoLoadFailure,
    MissingPath
}
type ManifestResult = Result<PathBuf, ManifestLoadError>;

fn main() {
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

        .subcommand(SubCommand::with_name("build")
             .help("Compile Project into a APK\n"))

        .subcommand(SubCommand::with_name("install")
             .help("Install Android APK onto Device\n"));

    let matches = app.clone().get_matches();

    // let current_manifest = current_manifest_path();

   let current_manifest = if let Some(explicit_dir) = matches.value_of("project-dir") {
        explicit_dir
    } else {
        current_manifest_path()
    };


    // // Fetching the configuration for the build.
    let mut config = config::load(&current_manifest);
    config.release = matches.is_present("release");

    // if let Some(target_arg_index) = env::args().position(|s| &s[..] == "--bin") {
    //     config.target = env::args().skip(target_arg_index + 1).next();
    // }

    if let Some(sub) = matches.subcommand_matches("build") {
        // build::build(&current_manifest, &config);
    }

    else if let Some(sub) = matches.subcommand_matches("install") {
         // install::install(&current_manifest, &config);
    }

    else {
        app.print_help();
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
