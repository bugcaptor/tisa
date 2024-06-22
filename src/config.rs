// src/config.rs
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::sync::RwLock;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub directory: String,
    pub pattern: String,
    pub todo_directory: String,
}

impl Config {
    pub fn new(config_path: &str) -> io::Result<Self> {
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config {
        directory: String::new(),
        pattern: String::new(),
        todo_directory: String::new(),
    });
}

pub fn load_config(config_path: &str) -> io::Result<()> {
    let config = Config::new(config_path)?;
    let mut w = CONFIG.write().unwrap();
    *w = config;
    Ok(())
}
