use std::{collections::HashMap, ffi::OsStr, fs::File, io::Read, path::PathBuf};

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "dotfile-path")]
    pub dotfile_path: String,

    #[serde(rename = "run-pre-gpush")]
    pub run_pre_gpush: String,

    // path -> modlist
    pub paths: HashMap<String, 
        // mod -> properties
        HashMap<String, 
            HashMap<String, String>>>,
}

pub fn config_to_struct(config_file: &mut File) -> Result<Config, &str> {
    let mut buf: String = String::default();
    if let Err(_) = config_file.read_to_string(&mut buf) {
        return Err("idm: Error ocurred while reading configuration file!");
    };

    match serde_json::from_str(buf.as_str()) {
        Ok(config_struct) => {
            return Ok(config_struct);
        },

        Err(_) => {
            return Err("idm: failed to parse JSON syntax in config!");
        }
    }
}

pub fn open_config_path(config_path: &PathBuf) -> Result<File, &str> {
    if !config_path.exists() {
        return Err("idm: Error, configuration file does not exist!");
    }

    if !config_path.is_file() {
        return Err("idm: Error, configuration must be a JSON file!");
    }

    if config_path.extension().and_then(OsStr::to_str) != Some("json") {
        return Err("idm: Error, configuration must be a JSON file!");
    }

    match File::open(config_path) {
        Ok(f) => {return Ok(f);},
        Err(_) => {
            return Err("idm: Error ocurred while opening configuration file!");
        },
    };
}
