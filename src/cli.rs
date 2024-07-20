use super::theme::Theme;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Theme to switch to
    #[arg(value_enum)]
    pub theme: Option<Theme>,
}
