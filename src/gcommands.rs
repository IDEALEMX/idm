use std::{path::PathBuf, process::Command, vec};
use uuid::Uuid;

use crate::config::Config;

pub fn handle_gpush(config_struct: &Config) {
    let repo_path = String::from(shellexpand::tilde(&config_struct.dotfile_path));

    let git_do = git_do_constructor(&repo_path);
    handle_hook(&config_struct.run_pre_gpush);
    
    git_do(vec!["add", "-A"]);
    git_do(vec!["commit", "-m", Uuid::new_v4().to_string().as_str()]);
    git_do(vec!["push", "origin"]);
    handle_hook(&config_struct.run_post_gpush);
}

pub fn handle_gpull(config_struct: &Config) {
    let repo_path = String::from(shellexpand::tilde(&config_struct.dotfile_path));
    let git_do = git_do_constructor(&repo_path);
    handle_hook(&config_struct.run_pre_gpull);

    git_do(vec!["pull", "origin"]);
    handle_hook(&config_struct.run_post_gpull);
}

fn git_do_constructor(repo_path: &String) -> impl Fn(Vec<&str>) {
    let repo_path = repo_path.clone();

    move |cmds: Vec<&str>| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&repo_path)
            .args(cmds)
            .output()
            .unwrap();

        print!("{}", String::from_utf8_lossy(&out.stdout));
        print!("{}", String::from_utf8_lossy(&out.stderr));
    }
}

fn handle_hook(hook: &Option<PathBuf>) {
    if let Some(pre_g_push_program) = hook.as_ref() {
        let out = Command::new("sh")
            .arg("-c")
            .arg(pre_g_push_program)
            // TODO: handle more gracefully
            .output().unwrap();
        print!("{}", String::from_utf8_lossy(&out.stdout));
        print!("{}", String::from_utf8_lossy(&out.stderr));
    }
}
