use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use libreauth::oath::TOTPBuilder;
use serde_json;

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

pub fn write_file(filepath: &str, filecontent: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .mode(0o600)
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath)?;
    file.write_all(filecontent.as_bytes())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Seed {
    name: String,
    seed: String,
}

impl Seed {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn seed(&self) -> &str {
        self.seed.as_str()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn set_seed(&mut self, seed: &str) {
        self.seed = seed.to_owned();
    }

    pub fn code(&self) -> String {
        TOTPBuilder::new()
            .base32_key(&self.seed)
            .finalize()
            .unwrap()   // crash if not base 32
            .generate()
    }
}

pub type SeedMap = HashMap<String, String>;

pub trait SeedMapIO {
    fn from_file() -> Result<SeedMap, IOError>;
    fn to_file(&self) -> Result<(), IOError>;
}

impl SeedMapIO for SeedMap {
    fn from_file() -> Result<SeedMap, IOError> {
        let filepath = config_file()?;
        let filepath = filepath.to_str().unwrap(); // crash if ??
        info!("Try loading seedmap from file {}", filepath);
        let cfg = read_file(filepath)?;
        debug!("File {} readed ({} chars.)", filepath, cfg.len());
        let seedmap = serde_json::from_str::<SeedMap>(cfg.as_str()).unwrap(); // crash if the format
        Ok(seedmap)
    }
    fn to_file(&self) -> Result<(), IOError> {
        let filepath = config_file()?;
        let filepath = filepath.to_str().unwrap(); // crash if ??

        info!("Writing seeds in file {}", filepath);
        let filecontent = serde_json::to_string_pretty(&self);
        let filecontent =
            filecontent.expect("Unable to save workspace, cannot serializing it to json");

        write_file(filepath, filecontent.as_str())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Seeds {
    seeds: Vec<Seed>,
}

impl Seeds {
    pub fn new() -> Self {
        Seeds { seeds: Vec::new() }
    }

    pub fn from_file() -> Result<Self, IOError> {
        let seedmap = SeedMap::from_file()?;
        Ok(Self::from_seedmap(&seedmap))
    }

    pub fn from_seedmap(seedmap: &SeedMap) -> Self {
        let seeds = seedmap
            .iter()
            .map(|(key, val)| Seed {
                name: key.to_owned(),
                seed: val.to_owned(),
            })
            .collect();
        info!("Seeds loaded from SeedMap");
        Seeds { seeds: seeds }
    }

    pub fn get_seeds(&self) -> &[Seed] {
        self.seeds.as_slice()
    }
}
