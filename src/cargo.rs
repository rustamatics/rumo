use std::path::Path;
use std::fs::File;
use std::io::Read;

use toml::Value;

use config::Config;

pub fn install_config(config: &Config) {
    let config_file = &*format!("{}/.cargo/config", config.project_path_str());

    if !Path::new(config_file).exists() {
        install_fresh_config(config, config_file);
    } else {
        if !config.ignore_linker_config {
            update_existing_config(config, config_file);
        } else {
            debug!("Ignoring Linker config")
        }
    }
}

#[allow(unused_variables)]
fn install_fresh_config(config: &Config, file: &str) {
    debug!("Installing fresh config: .cargo/config")
}

#[allow(unused_variables)]
fn update_existing_config(config: &Config, path: &str) {
    debug!("Updating existing config: .cargo/config");

    let mut data = config_contents(path).parse::<Value>().unwrap();
    println!("{:#?}\n", data["target"])
}

fn validate_linker_config() -> bool {
    false
}

fn config_contents(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open .cargo/config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "unable to read .cargo/config file",
    );

    contents
}
