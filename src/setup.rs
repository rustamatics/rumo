use config;
use std::path::PathBuf;


pub fn embed<P: Into<PathBuf>>(manifest: P, config: &config::Config) {
    let shell_zip = include_bytes!("../target/shell.zip");
}
