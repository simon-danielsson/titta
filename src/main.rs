use std::fs::{self};
use std::panic;
use std::{io, path::PathBuf};

// | Color   | Code |
// | ------- | ---- |
// | Black   | `30` |
// | Red     | `31` |
// | Green   | `32` |
// | Yellow  | `33` |
// | Blue    | `34` |
// | Magenta | `35` |
// | Cyan    | `36` |
// | White   | `37` |
// | Reset   | `0`  |

/// key, icon<char>, color<&str>
const FILETYPE_ATTR: &[(&str, char, &str)] = &[
    // general
    ("dir", '󰉖', "\x1b[34m"),
    ("hidden_dir", '󱞋', "\x1b[34m"),
    ("unknown", '', "\x1b[37m"),
    // dev
    ("rs", '', "\x1b[31m"),
    ("py", '', "\x1b[33m"),
    ("sh", '', "\x1b[32m"),
    ("js", '', "\x1b[33m"),
    ("c", '', "\x1b[36m"),
    ("toml", '', "\x1b[31m"),
    ("typ", '', "\x1b[36m"),
    // images
    ("png", '', "\x1b[32m"),
    ("jpg", '', "\x1b[32m"),
    ("gif", '', "\x1b[32m"),
    ("rw2", '', "\x1b[32m"),
    // audio
    ("wav", '', "\x1b[35m"),
    ("flac", '', "\x1b[35m"),
    ("aif", '', "\x1b[35m"),
    ("aiff", '', "\x1b[35m"),
    ("mp3", '', "\x1b[35m"),
    // text & pdf
    ("txt", '', "\x1b[37m"),
    ("md", '', "\x1b[35m"),
    ("pdf", '', "\x1b[31m"),
    // font
    ("ttf", '', "\x1b[37m"),
    ("otf", '', "\x1b[37m"),
];

/// helper: lookup()
const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// key search FILETYPE_ATTR
const fn lookup(key: &str) -> Option<(char, &str)> {
    let mut i = 0;
    while i < FILETYPE_ATTR.len() {
        let (k, v, c) = FILETYPE_ATTR[i];
        if str_eq(k, key) {
            return Some((v, c));
        }
        i += 1;
    }
    None
}

#[allow(unused)]
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
        for item in &self.dir_items {
            let mut ico: String = "".to_string();
            if self.f_use_devicons {
                ico = format!("{} ", &item.icon);
            }
            let mut col: String = "\x1b[0m".to_string();
            let col_end: String = "\x1b[0m".to_string();
            if self.f_with_color {
                col = format!("{}", &item.color_code);
            }

            println!("{col}{ico}{nam}{col_end}", nam = item.name);
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

            let mut color_code: &str = lookup("unknown").unwrap().1;
            let mut icon: char = lookup("unknown").unwrap().0;
            if !(f_type == "") {
                if let Some(ic) = lookup(&f_type) {
                    icon = ic.0;
                    color_code = ic.1;
                };
            }
            if is_dir && name.chars().nth(0) == Some('.') {
                if let Some(ic) = lookup("hidden_dir") {
                    icon = ic.0;
                    color_code = ic.1;
                };
            }

            // hidden files
            if name.chars().nth(0) == Some('.') {
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
