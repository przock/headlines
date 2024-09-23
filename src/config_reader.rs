use std::{fs, io, path::Path, error::Error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub tui: Tui,
    pub api: Api,
}

#[derive(Deserialize)]
pub struct Tui {
    pub ratio: Option<u16>,
}

#[derive(Deserialize)]
pub struct Api {
    pub sources: Option<String>,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let config_str: String = read_string()?;
    let config: Config = toml::from_str(config_str.as_str())?;
    Ok(config)
}

pub fn read_string() -> Result<String, Box<dyn Error>> {
    let config_home = get_config_home().unwrap();
    let config_path = format!("{}/headlines/headlines.toml", config_home);

    if !Path::new(&config_path).exists() {
        create_config()?;
    }

    let message: String = fs::read_to_string(format!("{}/headlines/headlines.toml", config_home))?;

    Ok(message)
}

fn get_config_home() -> Result<String, io::Error> {
    let home = std::env::var("HOME").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(format!("{}/.config", home))
}

fn create_config() -> Result<(), Box<dyn Error>> {
    let config_home = get_config_home()?;

    fs::create_dir_all(format!("{}/headlines", config_home))?;
    fs::File::create(format!("{}/headlines/headlines.toml", config_home))?;
    
    Ok(())
}
