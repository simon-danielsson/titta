use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::{io, path::PathBuf};

mod constants;
mod file_attr;
mod help;
mod ta_tree;
use crate::file_attr::*;

#[allow(unused)]
#[derive(Clone)]
struct Item {
    f_type: String,
    is_symlink: bool,
    is_dir: bool,
    is_hidden: bool,
    is_exec: bool,
    icon: char,
    color_code: String,
    name: String,
    abs_path: PathBuf,
    format: String,
}

fn main() -> io::Result<()> {
    let mut titta: Titta = Titta::new();
    titta.parse_args()?;

    if titta.s_help {
        titta.s_help();
        return Ok(());
    }

    titta.get_contents()?;
    titta.format_items();

    // aux cmd: ta tree
    if titta.s_view_as_tree {
        titta.s_view_as_tree()?;
        return Ok(());
    }

    // main cmd: ta
    titta.print_contents();

    Ok(())
}

/// f = flag, s = subc, sf = subc flag
struct Titta {
    current_dir: PathBuf,
    opt_dir: PathBuf,
    use_opt_dir: bool,
    dir_items: Vec<Item>,
    f_use_devicons: bool,
    f_with_color: bool,
    f_show_hidden: bool,
    f_show_executables: bool,
    s_view_as_tree: bool,
    sf_tree_lvl: i32,
    s_help: bool,
}

impl Titta {
    fn new() -> Self {
        Self {
            // dir
            opt_dir: PathBuf::new(),
            use_opt_dir: false,
            current_dir: std::env::current_dir().expect(
                "ERROR: The current working directory could not be identified",
            ),
            dir_items: Vec::new(),
            // flags
            f_use_devicons: false,
            f_with_color: false,
            f_show_hidden: false,
            f_show_executables: false,
            s_view_as_tree: false,
            sf_tree_lvl: 1,
            s_help: false,
        }
    }

    fn format_items(&mut self) {
        for item in self.dir_items.iter_mut() {
            let mut icon: String = "".to_string();
            if self.f_use_devicons {
                icon = format!("{} ", &item.icon);
            }

            let color_end: String = "\x1b[0m".to_string();
            let mut color = color_end.clone();
            if self.f_with_color {
                color = item.color_code.clone();
            }

            item.format = format!("{color}{icon}{name}{color_end}", name = item.name);
        }
    }

    fn print_contents(&mut self) {
        let cols = 3;

        let col_w: usize = (self
            .dir_items
            .iter()
            .map(|item| item.name.chars().count())
            .max()
            .unwrap_or(0)) + 2;

        for row in self.dir_items.chunks(cols) {
            for item in row {
                let len = (format!(
                    "{icon}{name}",
                    icon = item.icon,
                    name = item.name
                ))
                    .chars()
                    .count();

                let output = {
                    let spaces = " ".repeat(col_w.saturating_sub(len - 2));
                    format!("{item}{spaces}", item = item.format)
                };

                print!("{output}");
            }
            println!();
        }
    }

    fn is_executable(&mut self, metadata: &Metadata) -> bool {
        let permissions = metadata.permissions();
        return metadata.is_file() && permissions.mode() & 0o111 != 0;
    }

    fn get_contents(&mut self) -> io::Result<()> {
        // get dir
        let dir: PathBuf;
        if self.use_opt_dir {
            dir = self.opt_dir.clone();
        } else {
            dir = self.current_dir.clone();
        }

        // fill item vec
        let paths = fs::read_dir(dir)?;
        for path in paths {
            let mut opath = path;
            let f_type: String;

            if opath.as_mut().unwrap().path().is_dir() {
                f_type = "dir".to_string();
            } else {
                f_type = opath
                    .as_mut()
                    .unwrap()
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_string();
            }

            let mut name = opath.as_mut().unwrap().file_name().display().to_string();

            let mut is_exec: bool = false;
            let mut is_symlink: bool = false;
            let mut is_dir: bool = false;

            if let Ok(metadata) = opath.as_mut().unwrap().metadata() {
                is_exec = self.is_executable(&metadata);
                if is_exec {
                    name = format!("{name}*");
                }

                is_symlink = metadata.is_symlink();
                is_dir = metadata.is_dir();
                // metadata.modified()
                // metadata.accessed()
                // metadata.size()
            }

            // icons & color codes
            let mut color_code: &str = lookup("").unwrap().1;
            let mut icon: char = lookup("").unwrap().0;
            if !(f_type == "") {
                if let Some(ic) = lookup(&f_type) {
                    icon = ic.0;
                    color_code = ic.1;
                };
            } else {
                if let Some(ic) = lookup("unknown") {
                    icon = ic.0;
                    color_code = ic.1;
                };
            }

            // hidden files
            let mut is_hidden = false;

            if is_dir && name.chars().nth(0) == Some('.') {
                is_hidden = true;
                if let Some(ic) = lookup("hidden_dir") {
                    icon = ic.0;
                    color_code = ic.1;
                };
            }

            if !is_dir && name.chars().nth(0) == Some('.') {
                is_hidden = true;
            }

            // skip hidden files if show_hidden flag is not set
            if self.f_show_hidden == false {
                if is_hidden == true {
                    continue;
                }
            }

            // format placeholder
            let format: String = "".to_string();

            // push
            self.dir_items.push(Item {
                f_type: f_type.clone(),
                icon,
                color_code: color_code.to_string(),
                is_dir,
                is_hidden,
                is_symlink,
                is_exec,
                name,
                abs_path: opath.as_mut().unwrap().path(),
                format,
            });
        }

        Ok(())
    }

    fn parse_args(&mut self) -> std::io::Result<()> {
        let mut it = std::env::args().skip(1); // skip program name

        while let Some(arg) = it.next() {
            match arg.as_str() {
                "tree" => {
                    self.s_view_as_tree = true;

                    // use next if it exists and parses as i32, else default to 1
                    self.sf_tree_lvl = it
                        .next()
                        .as_deref()
                        .unwrap_or("1")
                        .parse::<i32>()
                        .unwrap_or(1);
                }
                "help" => self.s_help = true,
                "-i" => self.f_use_devicons = true,
                "-w" => self.f_with_color = true,
                "-a" => self.f_show_hidden = true,
                "-e" => self.f_show_executables = true,
                other => {
                    self.opt_dir = PathBuf::from(other);
                    if self.opt_dir.exists() {
                        self.use_opt_dir = true;
                    } else {
                        eprintln!("Directory doesn't exist!");
                    }
                    break; // stop after first positional
                }
            }
        }

        Ok(())
    }
}
