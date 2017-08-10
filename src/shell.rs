use std::path::{Path};
use std::fs::remove_dir_all;
use config::Config;
use utils;
use std::process::exit;

pub const SHELL_ZIP: &'static [u8] = include_bytes!("../target/android-shell.zip");

/// Checks to see if <project>/target/android-shell directory exists,
/// if not then the zip is unloaded upon the <project>/target directory
pub fn embed_if_not_present(config: &Config) {
    let project_path = config.project_path_str();
    let target_dir = &format!("{}/target", project_path)[..];

    match utils::mkdirp(target_dir) {
        Err(_) => panic!("Unable to create target directory at: {}", target_dir),
        Ok(_) => (),
    }

    if ! android_shell_exists(project_path) {
        let shell_dst = android_shell_dir(project_path);
        if ! utils::unzip_shell(SHELL_ZIP, shell_dst) {
            error!("Unable to extract android shell to: {}", project_path);
            exit(1);
        }
    }
}

/// Cleans the directory `android-shell` from the projects
/// target directory.
pub fn clean(config: &Config) {
    let project_path = config.project_path_str();

    if android_shell_exists(project_path) {
        match remove_dir_all(android_shell_dir(project_path)) {
            Ok(_) => println!("Removed Android Shell successfully"),
            Err(e) => println!("Failed to remove Android Shell: {}", e)
        }
    }
}

fn android_shell_exists(project_path: &str) -> bool {
    Path::new(&android_shell_dir(project_path)[..]).exists()
}

fn android_shell_dir(project_path: &str) -> String {
    format!("{}/target/android-shell", project_path)
}

fn android_shell_zip(project_path: &str) -> String {
    format!("{}/target/android-shell.zip", project_path)
}
