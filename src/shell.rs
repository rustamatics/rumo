use std::path::{Path};
use std::fs::remove_dir_all;
use config::Config;
use utils::unzip_shell;
use std::process::exit;

pub const SHELL_ZIP: &'static [u8] = include_bytes!("../target/shell.zip");

/// Checks to see if <project>/target/android-shell directory exists,
/// if not then the zip is unloaded upon the <project>/target directory
pub fn embed_if_not_present(config: &Config) {
    let project_path = config.project_path_str();
    if ! android_shell_exists(project_path) {
        if unzip_shell(android_shell_dir(project_path)) {
            println!("Extracted Android Shell successfully");
        } else {
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
