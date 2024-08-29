use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use serde::Deserialize;

use crate::config::Config;
use crate::theme::Theme;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LinkMethod {
    Soft,
    Hard,
}

pub fn apply_theme(theme: Arc<Theme>, config: Config) {
    config
        .apps
        .into_iter()
        .map(|(app_name, app_config)| {
            let app_name = Arc::new(app_name);
            let app_config = Arc::new(app_config);
            let theme = Arc::clone(&theme);

            thread::spawn(move || {
                let file = match theme.deref() {
                    Theme::Light => &app_config.light,
                    Theme::Dark => &app_config.dark,
                };
                if !Path::new(file).is_file() {
                    eprintln!("Config for {} not found", app_name);
                    return;
                }

                if let Err(e) = link(file, &app_config.target, &app_config.method) {
                    eprintln!(
                        "Error occurred while linking config for {}: {:?}",
                        app_name, e
                    );
                }
            })
        })
        .for_each(|handle| handle.join().unwrap());
}

fn link(source: &str, target: &str, method: &LinkMethod) -> Result<(), std::io::Error> {
    if Path::new(target).exists() {
        fs::remove_file(target)?;
    }

    match method {
        LinkMethod::Hard => fs::hard_link(source, target),
        LinkMethod::Soft => std::os::unix::fs::symlink(source, target),
    }
}
