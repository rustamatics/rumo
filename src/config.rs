use std::collections::btree_map::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use toml;
use ndk::Arch;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TomlPackage {
    name: String,
    version: String,
    pub metadata: Option<TomlMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TomlMetadata {
    pub android: Option<TomlAndroid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlAndroid {
    pub application_attributes: Option<BTreeMap<String, String>>,
    pub activity_attributes: Option<BTreeMap<String, String>>,
    pub resources_dir: Option<String>,

    package_name: Option<String>,
    version_code: Option<u32>,
    label: Option<String>,
    icon: Option<String>,
    assets: Option<String>,
    res: Option<String>,
    compile_sdk_version: Option<u32>,
    min_sdk_version: Option<u32>,
    target_sdk_version: Option<u32>,
    build_tools_version: Option<String>,
    android_version: Option<u32>,
    fullscreen: Option<bool>,
    build_targets: Option<Vec<String>>,
    opengles_version_major: Option<u8>,
    opengles_version_minor: Option<u8>,
}

pub struct Config {
    /// Path to the root of the Android SDK.
    // pub sdk_path: PathBuf,
    /// Path to the root of the Android NDK.
    pub ndk_path: PathBuf,
    pub meta: Option<TomlAndroid>,
    pub resources_dir: String,

    /// The path to the root of the Cargo application.
    /// This comes from the cargo locate-project method or via
    /// a command line flag for out of source use.
    pub project_path: PathBuf,

    /// Root directory to place the collection of
    /// Standalone NDK toolchains, which are generated during a one-time initialisation.
    pub toolchain_target_dir: String,

    /// Name that the package will have on the Android machine.
    /// This is the key that Android uses to identify your package, so it should be unique for
    /// for each application and should contain the vendor's name.
    pub package_name: String,
    pub package_version: String,

    /// Incrementing number for android versionCode
    /// (different than version_name)
    pub version_code: u32,

    /// Name of the project to feed to the SDK. This will be the name of the APK file.
    /// Should be a "system-ish" name, like `my-project`.
    pub project_name: String,
    pub project_name_underscore: String,

    /// Label for the package.
    pub package_label: String,

    /// Name of the launcher icon.
    /// Versions of this icon with different resolutions have to reside in the res folder
    pub package_icon: Option<String>,

    pub ignore_linker_config: bool,

    /// List of targets to build the app for. Eg. `arm-linux-androideabi`.
    pub build_targets: Vec<Arch>,

    pub build_tools_version: String,
    pub compile_sdk_version: u32,
    pub target_sdk_version: u32,
    pub min_sdk_version: u32,

    /// Version of android for which to compile.
    /// TODO: ensure that >=18 because Rustc only supports 18+
    pub android_version: u32,

    /// If `Some`, a path that contains the list of assets to ship as part of the package.
    ///
    /// The assets can later be loaded with the runtime library.
    pub assets_path: Option<PathBuf>,

    /// If `Some`, a path that contains the list of resources to ship as part of the package.
    ///
    /// The resources can later be loaded with the runtime library.
    /// This folder contains for example the launcher icon,
    /// the styles and resolution dependent images.
    pub res_path: Option<PathBuf>,

    /// Should we build in release mode?
    pub release: bool,

    /// Should this app be in fullscreen mode (hides the title bar)?
    pub fullscreen: bool,

    /// Appends this string to the application attributes in the AndroidManifest.xml
    pub application_attributes: Option<String>,

    /// Appends this string to the activity attributes in the AndroidManifest.xml
    pub activity_attributes: Option<String>,

    /// The name of the executable to compile.
    pub target: Option<String>,

    /// The OpenGL ES major version in the AndroidManifest.xml
    pub opengles_version_major: u8,

    /// The OpenGL ES minor version in the AndroidManifest.xml
    pub opengles_version_minor: u8,
}

impl Config {
    #[inline]
    pub fn project_path_str(&self) -> &str {
        self.project_path.to_str().unwrap()
    }
}

pub fn load(manifest_path: &Path) -> Config {
    // Determine the name of the package and the Android-specific metadata from the Cargo.toml
    let (package_name, package_version, manifest_content) = {
        let content = {
            let mut file = File::open(manifest_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        };

        let toml = content.parse::<toml::Value>().unwrap();
        let decoded: TomlPackage = toml["package"].clone().try_into::<TomlPackage>().unwrap();

        let package_name = decoded.name.clone();
        let package_version = decoded.version.clone();
        (
            package_name,
            package_version,
            decoded.metadata.and_then(|m| m.android),
        )
    };

    let ndk_path = env::var("NDK_HOME").expect(
        "Please set the path to the Android NDK with the \
                                                $NDK_HOME environment variable.",
    );

    let manifest_parent = str::replace(&manifest_path.to_str().unwrap()[..], "/Cargo.toml", "");
    let project_path = Path::new(&manifest_parent[..]).to_owned();


    let n = &package_name.clone()[..];
    let project_name_underscore = str::replace(&str::replace(n, "-", "_")[..], "rust.", "");

    Config {
        ndk_path: Path::new(&ndk_path).to_owned(),

        toolchain_target_dir: format!("{}/target", project_path.to_str().unwrap()),
        resources_dir: manifest_content
            .as_ref()
            .and_then(|a| a.resources_dir.clone())
            .unwrap_or("resources".to_owned()),

        project_path: project_path,
        package_name: manifest_content
            .as_ref()
            .and_then(|a| a.package_name.clone())
            .unwrap_or_else(|| format!("rust.{}", package_name)),
        package_version: package_version,
        version_code: manifest_content
            .as_ref()
            .and_then(|a| a.version_code)
            .unwrap_or(1),

        project_name: package_name.clone(),
        project_name_underscore: project_name_underscore.clone(),
        package_label: manifest_content
            .as_ref()
            .and_then(|a| a.label.clone())
            .unwrap_or_else(|| package_name.clone()),

        package_icon: manifest_content.as_ref().and_then(|a| a.icon.clone()),

        ignore_linker_config: false,

        build_targets: vec![],

        compile_sdk_version: manifest_content
            .as_ref()
            .and_then(|a| a.compile_sdk_version)
            .unwrap_or(26),

        build_tools_version: manifest_content
            .as_ref()
            .and_then(|a| a.build_tools_version.clone())
            .unwrap_or("26.0.1".to_owned()),

        min_sdk_version: manifest_content
            .as_ref()
            .and_then(|a| a.min_sdk_version)
            .unwrap_or(15),

        target_sdk_version: manifest_content
            .as_ref()
            .and_then(|a| a.min_sdk_version)
            .unwrap_or(18),

        android_version: manifest_content
            .as_ref()
            .and_then(|a| a.android_version)
            .unwrap_or(18),

        assets_path: manifest_content
            .as_ref()
            .and_then(|a| a.assets.as_ref())
            .map(|p| manifest_path.parent().unwrap().join(p)),

        res_path: manifest_content.as_ref().and_then(|a| a.res.as_ref()).map(
            |p| {
                manifest_path.parent().unwrap().join(p)
            },
        ),
        release: false,
        fullscreen: manifest_content
            .as_ref()
            .and_then(|a| a.fullscreen.clone())
            .unwrap_or(false),
        application_attributes: manifest_content.as_ref().and_then(|a| {
            map_to_string(a.application_attributes.clone())
        }),

        activity_attributes: manifest_content.as_ref().and_then(|a| {
            map_to_string(a.activity_attributes.clone())
        }),

        target: None,
        opengles_version_major: manifest_content
            .as_ref()
            .and_then(|a| a.opengles_version_major)
            .unwrap_or(2),
        opengles_version_minor: manifest_content
            .as_ref()
            .and_then(|a| a.opengles_version_minor)
            .unwrap_or(0),
        meta: manifest_content,
    }
}

fn map_to_string(input_map: Option<BTreeMap<String, String>>) -> Option<String> {
    // TODO rewrite this in functional style
    if let Some(map) = input_map {
        let mut result = String::new();
        for (key, val) in map {
            result.push_str(&format!("\n{}=\"{}\"", key, val))
        }
        Some(result)
    } else {
        None
    }
}
