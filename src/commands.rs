use config::{Config};
use termcmd::TermCmd;

pub fn build(config: &Config) {
    let project_path = config.project_path_str();

    let n = &config.package_name.clone()[..];
    let app_name = r(&r(n, "-", "_")[..], "rust.", "");

    TermCmd::new("build", "target/android-shell/bin/build")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", app_name)
        .execute();
}

pub fn install(config: &Config) {
    TermCmd::new("install", "target/android-shell/bin/install")
        .current_dir(config.project_path_str())
        .inherit_stdouterr()
        .execute();
}

pub fn clean(config: &Config) {
    TermCmd::new("clean", "target/android-shell/bin/clean")
        .current_dir(config.project_path_str())
        .inherit_stdouterr()
        .execute();
}


fn r(src: &str, a: &str, b: &str) -> String {
    str::replace(src, a, b)
}
