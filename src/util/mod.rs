use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub mod service;

pub fn default_repo_work_path(repo_name: String) -> String {
    let mut out = String::new();
    if let Ok(cur_user) = whoami::username() {
        if cur_user.contains("root") {
            out = format!("/root/.cache/phantomCI/{}/", repo_name);
        } else {
            out = format!("/home/{}/.cache/phantomCI/{}/", cur_user, repo_name);
        }
        let _ = fs::create_dir_all(Path::new(&out));
    }
    out
}

pub fn default_config_path() -> String {
    let mut out = String::new();
    if let Ok(cur_user) = whoami::username() {
        if cur_user.contains("root") {
            out = "/root/.config/phantomCI/".to_string();
        } else {
            out = format!("/home/{}/.config/phantomCI/", cur_user);
        }
        let _ = fs::create_dir_all(Path::new(&out));
    }
    out
}

pub fn create_default_config(path: &String) {
    let default_config = r#"
##[sys-compare]
##path = "https://github.com/helloimalemur/sys-compare"
##target_branch = "master"

"#;
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(default_config.as_ref());
    }
}