use std::process::exit;
use std::path::Path;
use std::fmt::Display;
use std::fmt;
use std::result::Result;

use config::Config;
use termcmd::TermCmd;

#[allow(non_camel_case_types)]
pub enum Arch {
    ARM,
    ARM64,
    X86,
    X86_64,
    MIPS,
    MIPS64,
}

impl Arch {
    pub fn from_str<S: AsRef<str>>(s: S) -> Arch
    where
        S: Display,
    {
        match &s.as_ref().to_lowercase()[..] {
            "arm" => Arch::ARM,
            "arm64" => Arch::ARM64,
            "x86" => Arch::X86,
            "x86_64" => Arch::X86_64,
            "mips" => Arch::MIPS,
            "mips64" => Arch::MIPS64,
            _ => {
                panic!(
                    "Unknown arch: {}\
                         Valid architectures:\
                         arm, arm64, x86, x86_64, mips, mips64
                         ",
                    s
                )
            }
        }
    }
}

impl Into<String> for Arch {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Arch::ARM => write!(f, "arm"),
            &Arch::ARM64 => write!(f, "arm64"),
            &Arch::X86 => write!(f, "x86"),
            &Arch::X86_64 => write!(f, "x86_64"),
            &Arch::MIPS => write!(f, "mips"),
            &Arch::MIPS64 => write!(f, "mips64"),
        }
    }
}

pub fn install_standalone(config: &Config) {
    let make_tool = &*format!(
        "{}/build/tools/make_standalone_toolchain.py",
        config.ndk_path.to_str().unwrap()
    );

    if !Path::new(make_tool).exists() {
        error!(
            "Could not find make_standalone_toolchain.py in your $NDK_HOME \
                Please confirm you have set a valid NDK_HOME path"
        );
        exit(1);
    }

    if config.enable_arm {
        install_toolchain(make_tool, config, Arch::ARM);
    }

    if config.enable_arm64 {
        install_toolchain(make_tool, config, Arch::ARM64);
    }

    if config.enable_x86 {
        install_toolchain(make_tool, config, Arch::X86);
    }

    if config.enable_x86_64 {
        install_toolchain(make_tool, config, Arch::X86_64);
    }

    if config.enable_mips {
        install_toolchain(make_tool, config, Arch::MIPS);
    }

    if config.enable_mips_64 {
        install_toolchain(make_tool, config, Arch::MIPS64);
    }
}

fn install_toolchain(make_tool_path: &str, config: &Config, arch: Arch) {
    print!("One-Time Install of Standalone NDK toolchain for: {}", arch);
    let toolchain_dir = &*format!(
        "{}/.ndk-toolchain-{}",
        config.toolchain_target_dir.clone(),
        arch
    );

    if !Path::new(toolchain_dir).exists() {
        TermCmd::new("make_standalone_toolchain", make_tool_path)
            .inherit_stdouterr()
            .argp("install-dir", toolchain_dir)
            .argp("api", config.android_api.clone())
            .argp("arch", arch)
            .execute();
    } else {
        debug!("NDK Toolchain confirmed: {}", arch);
    }
}
