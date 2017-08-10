use config::{Config};
use termcmd::TermCmd;

pub fn build(config: &Config) {
    let project_path = config.project_path_str();
    TermCmd::new("build", "target/android-shell/bin/build")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.package_name_sanitized.clone())
        .execute();
}

pub fn install(config: &Config) {
    let project_path = config.project_path_str();
    TermCmd::new("install", "target/android-shell/bin/install")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.package_name_sanitized.clone())
        .execute();
}

pub fn clean(config: &Config) {
    let project_path = config.project_path_str();
    TermCmd::new("clean", "target/android-shell/bin/clean")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.package_name_sanitized.clone())
        .execute();
}


fn r(src: &str, a: &str, b: &str) -> String {
    str::replace(src, a, b)
}
