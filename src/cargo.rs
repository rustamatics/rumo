use std::path::Path;
use std::fs::{File, OpenOptions, create_dir};
use std::io::{Read, Write, BufWriter};
use std;

use toml;
use toml::Value;
use toml::value::Table;

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

fn install_fresh_config(config: &Config, file: &str) {
    debug!("Installing fresh config: .cargo/config");
    let cargo_config_dir = &*format!("{}/.cargo", config.project_path_str());

    if !Path::new(cargo_config_dir).exists() {
        create_dir(cargo_config_dir).expect("Could not create .cargo directory");
    }

    File::create(file).expect("Unable to create .cargo/config file");

    let mut buffer = String::new();

    for arch in &config.build_targets {
        let triple = arch.triple();
        let s = &*format!("[target.{}]\n", triple);
        buffer.push_str(s);

        let linker_path = format!(
            "{}/ndk-toolchain-{}/bin/{}-gcc",
            config.toolchain_target_dir,
            arch,
            triple
        );

        buffer.push_str(&*format!("linker = \"{}\"", linker_path));
    }

    config_write(&mut *buffer, file).expect("config_write: Failed to write config data to file")
}

fn update_existing_config(config: &Config, path: &str) {
    debug!("Updating existing config: .cargo/config");

    let mut data = match config_contents(path).parse::<Value>() {
        Ok(d) => d,
        Err(e) => {
            error!("Could not parse .cargo/config\n{}", e);
            return install_fresh_config(config, path);
        }
    };

    let tkey = "target";
    let linker_key = "linker";

    // Ensure [target] exists
    if None == data.get(tkey) {
        let data_table = data.as_table_mut().unwrap();
        data_table.insert(tkey.to_owned(), Value::Table(Table::new()));
    }

    for arch in &config.build_targets {
        let triple = &*arch.triple();
        debug!("processing arch: {}", triple);

        // Insert "target.triple" if not present
        if None == data[tkey].get(triple) {
            debug!("{} does not contain {}", tkey, triple);
            let targets_table = data[tkey].as_table_mut().unwrap();
            targets_table.insert(triple.to_owned(), Value::Table(Table::new()));
        }

        // Insert "target.triple.linker"
        {
            let triple_entry = data[tkey][triple].as_table_mut().unwrap();
            let linker_path = format!(
                "{}/ndk-toolchain-{}/bin/{}-gcc",
                config.toolchain_target_dir,
                arch,
                triple
            );

            triple_entry.insert(linker_key.to_owned(), Value::String(linker_path));
        }

        match toml::to_string(&data) {
            Ok(mut s) => {
                match config_write(&mut *s, path) {
                    Ok(_) => {
                        debug!("config_write succeeded");
                    }
                    Err(e) => {
                        panic!("Failed to write cargo config file: {}", e);
                    }
                }
            }
            Err(e) => {
                panic!(
                    "Could not deserialize with toml::to_string\n:{}\nDue to error: {}",
                    data,
                    e
                )
            }
        }
    }
}

fn validate_linker_config() -> bool {
    true
}

fn config_contents(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open .cargo/config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "unable to read .cargo/config file",
    );

    contents
}

fn config_write(contents: &mut str, path: &str) -> Result<(), std::io::Error> {
    debug!("writing to: {}", path);
    let mut options = OpenOptions::new();
    options.write(true);
    let file = options.open(path).unwrap();
    let mut file_buffer = BufWriter::new(file);
    file_buffer.write_all(contents.as_bytes()).expect(
        "Failed to write contents to file buffer",
    );
    file_buffer.flush()
}
