use std::fs;
use std::{
    borrow::Borrow, fs::hard_link, io::Error, os::unix::fs::symlink, path::Path, sync::Arc, thread,
};

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
    let mut handles = vec![];
    let theme = Arc::new(theme);
    for (app_name, app_config) in config.apps {
        let app_config = Arc::new(app_config);
        let app_name = Arc::new(app_name);
        let theme = Arc::clone(&theme);
        let handle = thread::spawn(move || {
            let c = app_config.borrow();
            let t = theme.borrow();
            let file = get_filename_based_on_theme(c, &t);
            if !Path::new(file).is_file() {
                println!("Config for {} not found", app_name);
                return;
            }
            let status = link(file, &c.target, &c.method);
            if let Err(e) = status {
                println!(
                    "Error occurred while linking config for {}: {:?}",
                    app_name, e
                );
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn link(source: &str, target: &str, method: &LinkMethod) -> Result<(), Error> {
    if Path::new(&target).is_file() | Path::new(&target).is_symlink() {
        fs::remove_file(&target)?;
    } else {
        eprintln!("Target file exists, and not a file or symlink {}", &target);
        return Ok(());
    }
    match method {
        LinkMethod::Hard => hard_link(&source, &target),
        LinkMethod::Soft => symlink(&source, &target),
    }
}

fn get_filename_based_on_theme<'a>(config: &'a AppConfig, theme: &Theme) -> &'a str {
    match theme {
        Theme::Light => &config.light,
        Theme::Dark => &config.dark,
    }
}
