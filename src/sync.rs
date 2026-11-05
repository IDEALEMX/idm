use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::utils::ask_for_confirmation;

pub fn handle_sync(config_struct: &Config) {
    for (path_prefix, module) in config_struct.paths.iter() {
        let path_prefix_buf = {
            let module_expanded_path = shellexpand::tilde(&path_prefix);
            PathBuf::from(module_expanded_path.as_ref())
        };

        if !path_prefix_buf.exists() {
            eprintln!("idm: Error, path: target {} does not exist!", path_prefix_buf.display());
            continue;
        }

        for (module_head, attributes) in module.iter() {

            // If "from" attribute exists, change the dotfiles head so origin will
            // be valid
            let dotfiles_module_head = if attributes.contains_key("from") {
                &attributes["from"]
            } else {
                module_head
            };

            let full_module_path = {
                let concat_path = format!("{path_prefix}{module_head}");
                let module_expanded_path = shellexpand::tilde(&concat_path);
                PathBuf::from(module_expanded_path.as_ref())
            };

            let full_dotfile_module_path = {
                let concat_path = format!("{}{dotfiles_module_head}", config_struct.dotfile_path);
                let module_expanded_path = shellexpand::tilde(&concat_path);
                PathBuf::from(module_expanded_path.as_ref())
            };

            if !full_dotfile_module_path.exists() {
                eprintln!("idm: Error, path to module: {} does not exist!", full_dotfile_module_path.display());
                continue;
            }

            // Handle case where full_module_path is already occupied
            if full_module_path.exists() && !full_module_path.is_symlink() {
                println!("We got here for {}", module_head);
                module_path_occupied_non_symlink(&full_module_path, &full_dotfile_module_path);
            } else if full_module_path.exists() {
                module_path_occupied_symlink(&full_module_path, &full_dotfile_module_path);
            } else {
                create_symlink(&full_module_path, &full_dotfile_module_path);
            }
        }
    }
}

fn module_path_occupied_non_symlink(full_module_path: &PathBuf, full_dotfile_module_path: &PathBuf) {
    let prompt = format!("Path {} already exists and is not a symlink, what would you like to do?
1) Rename to {}.bak
2) Overwrite
3) Skip", full_module_path.display(), full_module_path.display());
    match ask_for_confirmation(prompt.as_str(), vec!["1", "2", "3"]).as_str().trim() {
        "1" => {
            if let Err(_) = fs::rename(full_module_path, format!("{}.bak", full_module_path.display())) {
                eprintln!("idm: Error, could not rename module!")
            };

            create_symlink(full_module_path, full_dotfile_module_path);
        }

        "2" => {
            if let Err(_) = fs::remove_dir_all(full_module_path) {
                if let Err(_) = fs::remove_file(full_module_path) {
                    eprintln!("idm: Error, could not remove module!")
                }
            };

            create_symlink(full_module_path, full_dotfile_module_path);
        }

        "3" => {}
        _ => panic!("Non valid option was somehow accepted, this should not be possible!")
    }
}

fn module_path_occupied_symlink(full_module_path: &PathBuf, full_dotfile_module_path: &PathBuf) {
    assert!(full_module_path.is_symlink());

    let symlink_target = match fs::read_link(full_module_path) {
        Ok(p) => {p},
        Err(_) => {
            eprintln!("idm: Error, symlink at {} was found but target could not be located!", full_module_path.display());
            return;
        }
    };

    if &symlink_target == full_dotfile_module_path {
        return
    }

    let prompt = format!("The following symlink: {} -> {} already exists, what would you like to do?
1) Overwrite
2) Skip", full_module_path.display(), symlink_target.display());
    match ask_for_confirmation(prompt.as_str(), vec!["1", "2"]).as_str().trim() {
        "1" => {
            if let Err(_) = fs::remove_file(&full_module_path) {
                eprintln!("idm: Error, could not remove symlink!")
            }

            create_symlink(full_module_path, full_dotfile_module_path);
        }

        "2" => {}
        _ => panic!("Non valid option was somehow accepted, this should not be possible!")
    }
}

fn create_symlink(full_module_path: &PathBuf, full_dotfile_module_path: &PathBuf) {
    if let Err(_) = std::os::unix::fs::symlink(&full_dotfile_module_path, &full_module_path) {
        eprintln!("idm: Error, could not create symlink for: {} -> {}", full_dotfile_module_path.display(), full_module_path.display());
    } else {
        println!("idm: Created symlink for: {} -> {}", full_dotfile_module_path.display(), full_module_path.display());
    };
}
