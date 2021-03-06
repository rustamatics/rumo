use std::process::exit;
use std::path::Path;
use std::fmt::Display;
use std::fmt;
use std::result::Result;

use config::Config;
use termcmd::TermCmd;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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

    pub fn triple(&self) -> String {
        match self {
            &Arch::ARM => "arm-linux-androideabi",
            &Arch::ARM64 => "aarch64-linux-android",
            &Arch::X86 => "i686-linux-android",
            &Arch::X86_64 => "x86_64-linux-android",
            &Arch::MIPS => "mips-unknown-linux-gnu",
            &Arch::MIPS64 => "mips64-unknown-linux-gnuabi64",
        }.to_owned()
    }

    pub fn all() -> Vec<Arch> {
        vec![
            Arch::ARM,
            Arch::ARM64,
            Arch::X86,
            Arch::X86_64,
            Arch::MIPS,
            Arch::MIPS64,
        ]
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
            "Could not find make_standalone_toolchain.py in your $NDK_HOME \n\
                Please confirm you have set NDK_HOME to a valid path.\n"
        );
        exit(1);
    }

    for arch in &config.build_targets {
        install_toolchain(make_tool, config, arch.clone());
    }
}

fn install_toolchain(make_tool_path: &str, config: &Config, arch: Arch) {
    let toolchain_dir = &*format!(
        "{}/ndk-toolchain-{}",
        config.toolchain_target_dir.clone(),
        arch
    );

    if !Path::new(toolchain_dir).exists() {
        print!(
            "One-Time Install of Standalone NDK toolchain for: {}\n",
            arch
        );
        TermCmd::new("make_standalone_toolchain", make_tool_path)
            .inherit_stdouterr()
            .argp("install-dir", toolchain_dir)
            .argp("api", format!("{}", config.android_version.clone()))
            .argp("arch", arch)
            .execute();
    } else {
        debug!("NDK Toolchain confirmed: {}", arch);
    }
}
