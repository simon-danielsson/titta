use std::fs::{self};
use std::panic;
use std::{io, path::PathBuf};

mod file_attr;
use crate::file_attr::*;

#[allow(unused)]
#[derive(Clone)]
struct Item {
    f_type: String,
    is_symlink: bool,
    is_dir: bool,
    is_hidden: bool,
    icon: char,
    color_code: String,
    name: String,
    abs_path: PathBuf,
}

fn main() -> io::Result<()> {
    let mut titta: Titta = Titta::new();
    titta.parse_args()?;

    titta.get_contents()?;
    titta.print_contents();

    Ok(())
}

struct Titta {
    arg_counter: usize,
    current_dir: PathBuf,
    opt_dir: PathBuf,
    use_opt_dir: bool,
    dir_items: Vec<Item>,
    f_use_devicons: bool,
    f_with_color: bool,
    f_show_hidden: bool,
}

impl Titta {
    fn new() -> Self {
        Self {
            arg_counter: 1,
            // dir
            opt_dir: PathBuf::new(),
            use_opt_dir: false,
            current_dir: std::env::current_dir().expect(
                "ERROR: The current working directory could not be identified",
            ),
            dir_items: Vec::new(),
            // Flags
            f_use_devicons: false,
            f_with_color: false,
            f_show_hidden: false,
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
                let mut icon: String = "".to_string();
                if self.f_use_devicons {
                    icon = format!("{} ", &item.icon);
                }

                let mut color: String = "\x1b[0m".to_string();
                let color_end: String = "\x1b[0m".to_string();
                if self.f_with_color {
                    color = format!("{}", &item.color_code);
                }

                let print =
                format!("{color}{icon}{name}{color_end}", name = item.name);
                let len =
                (format!("{icon}{name}", name = item.name)).chars().count();

                let output = {
                    let spaces = " ".repeat(col_w.saturating_sub(len - 2));
                    format!("{print}{spaces}")
                };

                print!("{output}");
            }
            println!();
        }
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
            let is_symlink = opath.as_mut().unwrap().path().is_symlink();
            let is_dir = opath.as_mut().unwrap().path().is_dir();
            let name = opath.as_mut().unwrap().file_name().display().to_string();
            let mut is_hidden = false;

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

            if is_dir && name.chars().nth(0) == Some('.') {
                is_hidden = true;
                if let Some(ic) = lookup("hidden_dir") {
                    icon = ic.0;
                    color_code = ic.1;
                };
            }

            // hidden files
            if !is_dir && name.chars().nth(0) == Some('.') {
                is_hidden = true;
            }

            // skip hidden files if show_hidden flag is not set
            if self.f_show_hidden == false {
                if is_hidden == true {
                    continue;
                }
            }

            // push
            self.dir_items.push(Item {
                f_type: f_type.clone(),
                icon,
                color_code: color_code.to_string(),
                is_dir,
                is_hidden,
                is_symlink,
                name,
                abs_path: opath.as_mut().unwrap().path(),
            });
        }

        Ok(())
    }

    /// helper: parse_args()
    fn next_arg(&mut self) -> io::Result<String> {
        let mut arg: String = String::new();
        if let Some(arg_p) = std::env::args().nth(self.arg_counter) {
            arg = arg_p
        }
        self.arg_counter += 1;
        Ok(arg)
    }

    fn parse_args(&mut self) -> io::Result<()> {
        loop {
            let arg = self.next_arg()?;

            match arg.as_str() {
                "-i" => {
                    self.f_use_devicons = true;
                    continue;
                }
                "-w" => {
                    self.f_with_color = true;
                    continue;
                }
                "-a" => {
                    self.f_show_hidden = true;
                    continue;
                }
                _ => break,
            };
        }

        if let Some(path) = std::env::args().nth(self.arg_counter - 1) {
            self.opt_dir = PathBuf::from(path);
            if self.opt_dir.exists() {
                self.use_opt_dir = true;
            } else {
                panic!("ERROR: Directory doesn't exist")
            }
        }

        Ok(())
    }
}
