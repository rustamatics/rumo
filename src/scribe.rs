use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufWriter, stdout};
use std;
use std::process::exit;
use std::convert::{AsRef, Into};
use regex::Regex;
use config::Config;


struct ScribeVal {
    val: String,
}

impl From<u32> for ScribeVal {
    fn from(val: u32) -> ScribeVal {
        ScribeVal { val: format!("{}", val) }
    }
}

impl From<String> for ScribeVal {
    fn from(val: String) -> ScribeVal {
        ScribeVal { val: val }
    }
}

impl<'a> From<&'a String> for ScribeVal {
    fn from(val: &String) -> ScribeVal {
        ScribeVal { val: val.clone() }
    }
}

struct ScribeChain {
    contents: String,
}

impl ScribeChain {
    fn replace<T: Into<ScribeVal>>(&mut self, var: &str, val: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        let regex = Regex::new(r"buildToolsVersion[\s+](?P<ver>\d+)").unwrap();
        let replace = &*format!("{} {}", var, val);
        let data = self.contents.clone();
        let out = &*regex.replace(&*data, replace);
        self.contents = out.to_owned();
        self
    }
}

pub fn turtle_shell(config: &Config) {
    let root = config.project_path_str();
    app_gradle(config, root);
    android_strings(config, root);
    exit(1);
}

fn app_gradle(config: &Config, root: &str) {
    let path = &*format!(
        "{}/target/android-shell/app/build.gradle",
        config.project_path_str()
    );

    let mut app_gradle_chain = ScribeChain { contents: file_contents(path) };

    app_gradle_chain
        .replace("compileSdkVersion", config.compile_sdk_version)
        .replace("buildToolsVersion", config.build_tools_version)
        .replace("minSdkVersion", config.min_sdk_version)
        .replace("targetSdkVersion", config.target_sdk_version)
        .replace("versionCode", &config.package_version)
        .replace("applicationId", &config.package_name);


    // compileSdkVersion
    // buildToolsVersion
    // applicationId
    // minSdkVersion
    // targetSdkVersion
    // versionCode
    // versionName
    // abiFilters

    write!(&mut stdout(), "{}", app_gradle_chain.contents)
        .expect("Failed to write scribe output to app.gradle");
}


fn android_strings(config: &Config, root: &str) {}

fn file_contents(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open .cargo/config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&*format!(
        "unable to read turtle shell file: {}",
        path
    ));

    contents
}

fn file_write(contents: &mut str, path: &str) -> Result<(), std::io::Error> {
    debug!("writing to: {}", path);
    let mut options = OpenOptions::new();
    options.write(true);
    let file = options.open(path).unwrap();
    let mut file_buffer = BufWriter::new(file);
    file_buffer.write_all(contents.as_bytes()).expect(
        "Failed to write content bytes into turtle buffer",
    );

    file_buffer.flush()
}
