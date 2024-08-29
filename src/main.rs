use clap::Parser;
use tsw::{cli::Args, config::parse_config, theme::get_win_theme, theme_manager::apply_theme};

fn main() {
    let config = parse_config().expect("Config not found!");
    let args = Args::parse();
    let theme_to_set = args.theme.unwrap_or_else(get_win_theme);
    apply_theme(theme_to_set.into(), config);
}
