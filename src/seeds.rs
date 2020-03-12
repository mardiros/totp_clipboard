use std::collections::HashMap;
//use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, ErrorKind};
//use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use dirs;
use libreauth::oath::TOTPBuilder;
use serde_json;

pub type IOError = io::Error;

fn home_dir() -> io::Result<PathBuf> {
    match dirs::home_dir() {
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

pub struct Seed {
    name: String,
    seed: String,
}

impl Seed {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn code(&self) -> String {
        TOTPBuilder::new()
            .base32_key(&self.seed)
            .finalize()
            .unwrap()   // crash if not base 32
            .generate()
    }
}

type SeedsMap = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Seeds {
    seeds: SeedsMap,
}

impl Seeds {
    pub fn new() -> Self {
        let seeds = SeedsMap::new();
        let seeds = Seeds { seeds: seeds };
        seeds
    }

    pub fn from_file() -> Result<Self, IOError> {
        let filepath = config_file()?;
        let filepath = filepath.to_str().unwrap(); // crash if ??
        let cfg = read_file(filepath)?;
        let seeds = serde_json::from_str::<SeedsMap>(cfg.as_str()).unwrap(); // crash if the format
        let seeds = Seeds { seeds: seeds };
        Ok(seeds)
    }

    pub fn get_seeds(&self) -> Vec<Seed> {
        let res = self.seeds
            .iter()
            .map(|(key, val)| Seed {
                name: key.to_owned(),
                seed: val.to_owned(),
            })
            .collect();
        res
    }
}
