use std::{
    fs::{self, hard_link},
    io::Error,
    os::unix::fs::symlink,
};

use expanduser::expanduser;
use serde::Deserialize;

use crate::{
    config::{AppConfig, Config},
    theme::Theme,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LinkMethod {
    Soft,
    Hard,
}

pub fn apply_theme(theme: Theme, config: Config) {
    for (app_name, app_config) in config.apps {
        let file = get_filename_based_on_theme(&app_config, &theme);
        let status = link(file, &app_config.target, &app_config.method);
        if let Err(_) = status {
            println!("Config for {} not found", app_name);
        }
    }
}

fn link(source: &str, target: &str, method: &LinkMethod) -> Result<(), Error> {
    let source_path = expanduser(source)?;
    let target_path = expanduser(target)?;

    if !(target_path.is_dir()) {
        fs::remove_file(&target_path)?;
    }

    match method {
        LinkMethod::Hard => hard_link(source_path, target_path),
        LinkMethod::Soft => symlink(source_path, target_path),
    }
}

fn get_filename_based_on_theme<'a>(config: &'a AppConfig, theme: &Theme) -> &'a str {
    match theme {
        Theme::Light => &config.light,
        Theme::Dark => &config.dark,
    }
}
