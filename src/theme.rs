use clap::ValueEnum;
use std::process::Command;

const KEY: &str = r"'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize'";
const PROPERTY: &str = "AppsUseLightTheme";

#[derive(Debug, Clone, ValueEnum)]
pub enum Theme {
    Light,
    Dark,
}

pub fn get_win_theme() -> Theme {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-Command",
            &format!("Get-ItemPropertyValue {KEY} -Name {PROPERTY}"),
        ])
        .output()
        .unwrap()
        .stdout;

    let is_light_theme = String::from_utf8(output)
        .unwrap()
        .trim()
        .parse::<u8>()
        .is_ok_and(|x| x != 0);
    if is_light_theme {
        Theme::Light
    } else {
        Theme::Dark
    }
}
