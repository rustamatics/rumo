use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufWriter};
use std;
use std::fmt;
// use std::process::exit;
use regex::Regex;
use config::Config;

use std::error::Error;
use std::convert::AsRef;
use std::str::FromStr;
use ndk;

#[derive(Debug)]
struct ScribeTransformError;

impl Error for ScribeTransformError {
    fn description(&self) -> &str {
        "Failed to transform into a ScribeVal"
    }
}

impl std::fmt::Display for ScribeTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

#[derive(Debug, Clone)]
struct ScribeVal {
    input: String,
}

impl Into<ScribeVal> for u32 {
    fn into(self) -> ScribeVal {
        ScribeVal { input: format!("{}", self) }
    }
}

impl Into<ScribeVal> for String {
    fn into(self) -> ScribeVal {
        ScribeVal { input: format!("\"{}\"", self) }
    }
}

impl FromStr for ScribeVal {
    type Err = ScribeTransformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scribed = ScribeVal { input: s.to_owned() };
        Ok(scribed)
    }
}

impl<'a> From<&'a str> for ScribeVal {
    fn from(s: &'a str) -> ScribeVal {
        ScribeVal { input: s.to_owned() }
    }
}

impl Into<ScribeVal> for Vec<ndk::Arch> {
    fn into(self) -> ScribeVal {
        let mapped = self.iter()
            .map(|s: &ndk::Arch| format!("\"{}\"", s))
            .collect::<Vec<String>>()
            .join(", ");

        ScribeVal { input: mapped }
    }
}

impl std::fmt::Display for ScribeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.input)
    }
}

struct ScribeChain {
    contents: String,
}

impl AsRef<str> for ScribeChain {
    fn as_ref(&self) -> &str {
        &*self.contents
    }
}

impl ScribeChain {
    // Scribe gradle format
    fn gradle<T: Into<ScribeVal>>(&mut self, var: &str, val: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        let r = format!("{}[\\s+](?P<val>.*)?", var);
        let regex = Regex::new(&*r).unwrap();
        let replace = &*format!("{} {}", var, val.into().input);
        let data = self.contents.clone();
        let out = regex.replace(&data[..], replace);

        self.contents = out.into_owned();
        self
    }

    // Scribe xml format
    fn xml<T: Into<ScribeVal>>(&mut self, var: &str, tag: &str, val: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        let scribe_val = str::replace(&val.into().input[..], "\"", "");
        println!("scribe_val: {}", scribe_val);

        let replace = &*format!("<{} name=\"{}\">{}</{}>", tag, var, scribe_val, tag);
        let r = format!("<{} name=\"{}\">(.*)</{}>", tag, var, tag);
        let regex = Regex::new(&*r).unwrap();
        let data = self.contents.clone();

        let out = regex.replace(&data[..], replace);

        self.contents = out.into_owned();
        self
    }

    fn replace(&mut self, var: &str, val: String) -> &mut Self {
        let r = format!("{}", var);
        let regex = Regex::new(&*r).unwrap();
        let data = self.contents.clone();
        let out = regex.replace(&data[..], &val[..]);

        self.contents = out.into_owned();
        self
    }
}

pub fn turtle_shell(config: &Config) {
    let root = config.project_path_str();
    app_gradle(config, root);
    android_resource_strings(config, root);
    android_manifest(config, root);
}

fn app_gradle(config: &Config, root: &str) {
    let path = &*format!("{}/target/android-shell/app/build.gradle", root);

    let mut gradle_build_chain = ScribeChain { contents: file_contents(path) };

    let targets: ScribeVal = config.build_targets.clone().into();

    gradle_build_chain
        .gradle("compileSdkVersion", config.compile_sdk_version)
        .gradle("buildToolsVersion", config.build_tools_version.clone())
        .gradle("minSdkVersion", config.min_sdk_version)
        .gradle("targetSdkVersion", config.target_sdk_version)
        .gradle("versionCode", config.version_code)
        .gradle("versionName", config.package_version.clone())
        .gradle("applicationId", config.package_name.clone())
        .gradle("abiFilters", targets.clone())
        .gradle("include", targets);

    // write!(&mut stdout(), "{}", app_gradle_chain.contents)
    //     .expect("Failed to write scribe output to app.gradle");

    file_write(&mut gradle_build_chain.contents[..], path)
        .expect("Failed to write gradle.build to android shell");

}
fn android_manifest(config: &Config, root: &str) {
    let manifest = &*format!(
        "{}/target/android-shell/app/src/main/AndroidManifest.xml",
        root
    );

    let mut manifest_chain = ScribeChain { contents: file_contents(manifest) };

    manifest_chain.replace("com.rumo.shell", config.package_name.clone());
    manifest_chain.replace("[version_name]", config.package_version.clone());
    manifest_chain.replace("[version_code]", config.version_code.to_string());

    if let Some(package_icon) = config.package_icon.clone() {
        manifest_chain.replace("@mipmap/ic_launcher", package_icon);
    }

    file_write(&mut manifest_chain.contents[..], manifest)
        .expect("Failed to write AndroidManifest.xml in android-shell");
}

fn android_resource_strings(config: &Config, root: &str) {
    let values_dir = format!("{}/target/android-shell/app/src/main/res/values", root);

    let colors_path = &*format!("{}/colors.xml", values_dir);
    let strings_path = &*format!("{}/strings.xml", values_dir);
    // let styles_path = &*format!( "{}/styles.xml", values_dir);
    // let mut styles_res_chain = ScribeChain { contents: file_contents(styles_path) }
    // file_write(&*mut styles_res_chain.contents, styles_path);

    let mut colors_res_chain = ScribeChain { contents: file_contents(colors_path) };
    let mut strings_res_chain = ScribeChain { contents: file_contents(strings_path) };
    let meta_copy = config.meta.clone();

    if let Some(meta) = meta_copy {
        if let Some(attrs) = meta.application_attributes {
            let colors = ["color_primary", "color_primary_dark", "color_accent"];
            for color in &colors {
                if let Some(c) = attrs.get(&color[..]) {
                    colors_res_chain.xml(color, "color", c.clone());
                }
            }

            let strings = ["app_name"];
            for string in &strings {
                if let Some(s) = attrs.get(&string[..]) {
                    strings_res_chain.xml(string, "string", s.clone());
                }
            }
        }
    }


    file_write(&mut colors_res_chain.contents[..], colors_path)
        .expect("Failed to write colors.xml to android/res/values");

    file_write(&mut strings_res_chain.contents[..], strings_path)
        .expect("Failed to write strings.xml to android/res/values");
}

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
