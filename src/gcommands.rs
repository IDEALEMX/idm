use std::{process::Command, vec};
use uuid::Uuid;

use crate::config::Config;

//TODO: This whole section is a mess, I will have to clean it up eventually
pub fn handle_gpush(config_struct: &Config) {
    let repo_path = String::from(shellexpand::tilde(&config_struct.dotfile_path));
    
    if let Some(pre_g_push_program) = config_struct.run_pre_gpush.as_ref() {
        let out = Command::new("sh")
            .arg("-c")
            .arg(pre_g_push_program)
            // TODO: handle more gracefully
            .output().unwrap();
        print!("{}", String::from_utf8_lossy(&out.stdout));
        print!("{}", String::from_utf8_lossy(&out.stderr));
    }

    let out = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .args(vec!["add", "-A"])
        // TODO: handle more gracefully
        .output().unwrap();
    print!("{}", String::from_utf8_lossy(&out.stdout));
    print!("{}", String::from_utf8_lossy(&out.stderr));

    let out = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .args(vec!["commit", "-m", Uuid::new_v4().to_string().as_str()])
        // TODO: handle more gracefully
        .output().unwrap();
    print!("{}", String::from_utf8_lossy(&out.stdout));
    print!("{}", String::from_utf8_lossy(&out.stderr));

    let out = Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .args(vec!["push", "origin"])
        // TODO: handle more gracefully
        .output().unwrap();
    print!("{}", String::from_utf8_lossy(&out.stdout));
    print!("{}", String::from_utf8_lossy(&out.stderr));
}
