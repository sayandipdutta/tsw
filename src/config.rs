use super::theme_manager::LinkMethod;
use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf, str::FromStr};

const CONF_FILE: &str = "tsw.toml";

use std::fs;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub light: String,
    pub dark: String,
    pub target: String,
    pub method: LinkMethod,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub apps: HashMap<String, AppConfig>,
}

fn find_config() -> Option<PathBuf> {
    let mut paths: Vec<String> = vec![];
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        paths.push(path)
    };
    let home = env::var("HOME").unwrap();
    let config = &format!("{home}/.config");
    paths.push(config.to_string());
    paths.push(home);

    paths
        .into_iter()
        .map(|item| PathBuf::from_str(&item).unwrap().join(CONF_FILE))
        .find(|p| p.is_file())
}

pub fn parse_config() -> Result<Config, Box<dyn std::error::Error>> {
    match find_config() {
        Some(p) => {
            let toml_str = fs::read_to_string(p)?;
            let config: Config = toml::from_str(&toml_str)?;
            Ok(config)
        }
        None => panic!("OOPS!!"),
    }
}
