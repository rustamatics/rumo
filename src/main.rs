#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate term;
extern crate toml;
extern crate clap;
extern crate zip;

extern crate regex;

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use clap::{Arg, App, SubCommand};
use std::fs;

pub mod config;
pub mod shell;
pub mod termcmd;
pub mod utils;
pub mod commands;
pub mod cargo;
pub mod ndk;
pub mod scribe;

use ndk::Arch;

// use config::Config;

#[derive(Debug)]
enum ManifestLoadError {
    CargoLoadFailure,
    MissingPath,
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

    let mut app = App::new("Rumo")
        .version(VERSION.unwrap_or("unknown"))
        .about(DESC.unwrap_or("--"))
        .arg(
            Arg::with_name("project-dir")
                .long("project-dir")
                .short("d")
                .help(
                    "Override the default project directory provided by `cargo locate-project`",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ndk-toolchain-root")
                .long("ndk-toolchain-root")
                .help(
                    "Root directory where individual standalone NDK toolchains \
                    are placed during a One-Time install at build. \n\n\

                    For each architecture targeted, a standalone NDK toolchain \
                    will be created in this directory. \n\n\

                    Default = <project_root>/target\n",
                ),
        )
        .arg(
            Arg::with_name("ignore-linker-config")
                .long("ignore-linker-config")
                .help(
                    "Ignore Linker configuration inside .cargo/config. \n\n\
                    By default, Rumo will update the linker config for each \
                    ABI inside the .cargo/config.\n\
                    Use this flag to tell Rumo not to alter those settings.\n\n\
                    ",
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Create APK(s) from Project")
                .arg(Arg::with_name("all").short("a").long("all"))
                .arg(Arg::with_name("release").long("release").help(
                    "Build with cargo `release` mode optimizations",
                ))
                .arg(Arg::with_name("clean").long("clean").help(
                    "Clean build artifacts for a fresh compilation",
                ))
                .arg(Arg::with_name("arm").long("arm").help("build arm target"))
                .arg(Arg::with_name("arm64").long("arm64"))
                .arg(Arg::with_name("x86").long("x86"))
                .arg(Arg::with_name("x86_64").long("x86_64"))
                .arg(Arg::with_name("mips").long("mips"))
                .arg(Arg::with_name("mips64").long("mips64")),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install build onto device\n")
                .arg(Arg::with_name("android").long("--android"))
                .arg(Arg::with_name("ios").long("--ios")),
        );

    ///////////////////////////////////////////////////////////////////

    let matches = app.clone().get_matches();

    // Fetching the configuration for the build.
    let mut config = config::load(&match matches.value_of("project-dir") {
        Some(p) => PathBuf::from(p),
        None => current_manifest_path().unwrap(),
    });

    // Control Release / debug build mode
    config.release = matches.is_present("release");

    // Allow overriding default ndk standalone toolchain root directory.
    // Default is <project_root>/target
    match matches.value_of("ndk-toolchain-root") {
        Some(p) => {
            config.toolchain_target_dir = match fs::canonicalize(p.to_owned()) {
                Ok(path) => path.to_str().unwrap().to_owned(),
                Err(e) => {
                    panic!(
                        "Failed to get canonical path of: {} \n
                        Error reason: {}",
                        p,
                        e
                    )
                }
            }
        }
        None => (),
    };

    // Allow user to ignore linker config modifications
    // (only if you are sure you know what your doing!)
    config.ignore_linker_config = matches.is_present("ignore-linker-config");

    ///////////////////////////////////////////////////////////////////

    // Every thing looks good, let's execute the users cli switches.
    if let Some(bm) = matches.subcommand_matches("build") {

        // Build vector of architectures
        if bm.is_present("all") {
            config.build_targets = Arch::all();
        } else {
            if bm.is_present("arm") {
                config.build_targets.push(Arch::ARM);
            }

            if bm.is_present("arm64") {
                config.build_targets.push(Arch::ARM64);
            }

            if bm.is_present("x86") {
                config.build_targets.push(Arch::X86);
            }

            if bm.is_present("x86_64") {
                config.build_targets.push(Arch::X86_64);
            }

            if bm.is_present("mips") {
                config.build_targets.push(Arch::MIPS);
            }

            if bm.is_present("mips64") {
                config.build_targets.push(Arch::MIPS64);
            }


            // Build fallback to x86 if no archs specified
            if config.build_targets.is_empty() {
                config.build_targets.push(Arch::X86);
            }
        }

        // Provide a way to clean the shells
        bm.is_present("clean") && shell::clean(&config);

        // Check to see if we have the project shell embedded
        // Noticeably, this is done after clean has a chance to run.
        shell::embed_if_not_present(&config);

        // Scribe correct project data upon the turtle shell
        scribe::turtle_shell(&config);

        // Ensure we have access to Standalone NDK toolchains
        // for each ABI we wish to target
        ndk::install_standalone(&config);

        // Check to see if .cargo/config is properly setup.
        // Will install or patch existing config if necessary
        cargo::install_config(&config);

        commands::build(&config);

    } else if let Some(_) = matches.subcommand_matches("device-install") {
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
    struct Data {
        root: String,
    }
    let stdout = String::from_utf8(output.stdout).unwrap();
    let decoded: Data = serde_json::from_str(&stdout).unwrap();
    let path = Path::new(&decoded.root).to_owned();

    // debug!("manifest: {:?}", path);
    Ok(path)
}
