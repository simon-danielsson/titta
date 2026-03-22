use crate::Titta;
// *brakoll - d: update readme and help with new details after overhaul, p: 100, t: docs, s: closed

const HELP_BODY: &str = include_str!("./static/help.txt");

// app info
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERS: &str = env!("CARGO_PKG_VERSION");
const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTH: &str = env!("CARGO_PKG_AUTHORS");

impl Titta {
    /// subcommand: print help
    pub fn s_help(&mut self) {
        println!("{APP_NAME} v{APP_VERS}");
        println!("{APP_DESC}");
        println!("{APP_REPO}");
        println!("{APP_AUTH}");
        println!("---");
        println!("{HELP_BODY}");
    }
}