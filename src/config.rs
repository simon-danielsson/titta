use std::{env, fs, io, path::PathBuf};

// *brakoll - d: make colors customizable using config file, p: 100, t: feat, s: closed

const DEF_CONF: &str = include_str!("./static/default_config");

pub fn get() -> io::Result<ConfigVars> {
    let home = env::home_dir().unwrap();
    let config_folder_path = home.join(".config/titta/");
    let config_file_path = home.join(".config/titta/config");

    if !config_file_path.exists() {
        gen_config(&config_folder_path)?;
    }

    return Ok(parse_config(config_file_path)?);
}

fn gen_config(dest: &PathBuf) -> io::Result<()> {
    fs::create_dir(dest)?;
    let file = dest.join("config");
    fs::write(file, DEF_CONF)?;
    Ok(())
}

fn parse_config(file: PathBuf) -> io::Result<ConfigVars> {
    let contents = fs::read_to_string(file)?;

    let mut conf = ConfigVars::new();

    let iter = contents.lines().into_iter();
    for lines in iter {
        let l = lines.trim();

        match l {
            // skip comments and empty lines
            l if l.starts_with("#") => {
                continue;
            }
            l if l.is_empty() => {
                continue;
            }

            // vars
            l if l.starts_with("red") => {
                conf.red = get_value(lines);
            }
            l if l.starts_with("green") => {
                conf.green = get_value(lines);
            }
            l if l.starts_with("yellow") => {
                conf.yellow = get_value(lines);
            }
            l if l.starts_with("blue") => {
                conf.blue = get_value(lines);
            }
            l if l.starts_with("magenta") => {
                conf.magenta = get_value(lines);
            }
            l if l.starts_with("cyan") => {
                conf.cyan = get_value(lines);
            }
            l if l.starts_with("orange") => {
                conf.orange = get_value(lines);
            }
            l if l.starts_with("white") => {
                conf.white = get_value(lines);
            }
            _ => {}
        }
    }

    // placeholder
    Ok(conf)
}

fn get_value(l: &str) -> String {
    let (_, v) = l.split_once('=').unwrap();
    v.trim().to_string()
}

pub struct ConfigVars {
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub orange: String,
    pub white: String,
}

impl ConfigVars {
    /// set defaults
    fn new() -> Self {
        Self {
            red: "#9ec1a3".to_string(),
            green: "#aa4465".to_string(),
            yellow: "#aab3c0".to_string(),
            blue: "#aab3c0".to_string(),
            magenta: "#9ec1a3".to_string(),
            cyan: "#aa4465".to_string(),
            orange: "#9ec1a3".to_string(),
            white: "#aa4465".to_string(),
        }
    }
}