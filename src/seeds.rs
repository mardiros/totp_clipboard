use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::env;
use std::path::PathBuf;

use serde_json;
use libreauth::oath::TOTPBuilder;

pub type IOError = io::Error;

fn home_dir() -> io::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(IOError::new(
            ErrorKind::NotFound,
            "Missing Home Directory from environment",
        )),
    }
}

pub fn config_dir() -> io::Result<PathBuf> {
    let mut path = home_dir()?;
    path.push(".config");

    if !path.exists() {
        fs::create_dir_all(path.to_str().unwrap())?;
    } else if !path.is_dir() {
        return Err(IOError::new(
            ErrorKind::InvalidData,
            format!("{} should be a directory", path.to_str().unwrap()),
        ));
    }
    Ok(path)
}

pub fn config_file() -> io::Result<PathBuf> {
    let mut path = config_dir()?;
    path.push("totp-seeds.json");
    Ok(path)
}


pub fn read_file(filepath: &str) -> io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    let res = String::from_utf8(buf).unwrap(); // crash if UTF8 error
    Ok(res)
}



type Seed = String;
type SeedsMap = HashMap<String, Seed>;

#[derive(Debug, Clone)]
pub struct Seeds {
    seeds: SeedsMap
}


impl Seeds {
    pub fn new() -> Self {
        let seeds = SeedsMap::new();
        let seeds = Seeds {
            seeds: seeds
        };
        seeds
    }

    pub fn from_file() -> Result<Self, IOError> {
        let filepath = config_file()?;
        let filepath = filepath.to_str().unwrap();  // crash if ??
        info!("Try loading workspace from file {}", filepath);
        let cfg = read_file(filepath)?;
        debug!("File {} readed ({} chars.)", filepath, cfg.len());
        let seeds = serde_json::from_str::<SeedsMap>(cfg.as_str()).unwrap(); // crash if the format
        let seeds = Seeds {
            seeds: seeds
        };
        info!("Seeds loaded from file {}", filepath);
        Ok(seeds)
    }

    pub fn get_code(&self, name:&str) -> String {
        match self.seeds.get(name) {
            Some(ref seed) => {
                TOTPBuilder::new()
                    .base32_key(seed)
                    .finalize()
                    .unwrap()   // crash if not base 32
                    .generate()
            },
            None => "".to_owned(),  // Could return an error
        }
    }

    pub fn get_names(&self) -> Vec<String> {
        let res = self.seeds.keys().map(|k| k.to_owned()).collect();
        res
    }

}