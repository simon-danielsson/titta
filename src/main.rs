use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::{io, path::PathBuf};

mod attributes;
mod config;
mod format;
mod help;
mod ta_tree;
use crate::attributes::FileType;

#[allow(dead_code)]
#[derive(Clone)]
struct Item {
    f_type: FileType,
    is_symlink: bool,
    is_hidden: bool,
    name: String,
    abs_path: PathBuf,
}

// *brakoll - d: new version after overhaul, p: 0, t: feature, s: closed
fn main() -> io::Result<()> {
    let mut titta: Titta = Titta::new();

    titta.parse_args()?;

    if titta.s_help {
        titta.s_help();
        return Ok(());
    }

    titta.get_contents()?;

    // *brakoll - d: add check for if show_hidden flag is active and filter hidden directories and dotfiles, p: 100, t: refactor, s: closed
    if !titta.f_show_hidden {
        titta.dir_items.retain(|item| !item.is_hidden);
    }

    // tree
    if titta.s_view_as_tree {
        print!("{}", titta.s_view_as_tree()?);
        return Ok(());
    }

    // main cmd: ta
    titta.print_contents();

    Ok(())
}

// *brakoll - d: make dir field of Titta struct less explicit, p: 100, t: refactor, s: closed
/// f = flag, s = subc, sf = subc flag
struct Titta {
    dir: PathBuf,
    dir_items: Vec<Item>,
    f_show_hidden: bool,
    s_view_as_tree: bool,
    sf_tree_lvl: i32,
    s_help: bool,
}

impl Titta {
    fn new() -> Self {
        Self {
            // dir
            dir: std::env::current_dir().expect("ERROR: CWD could not be identified"),
            dir_items: Vec::new(),

            // flags
            f_show_hidden: false,
            s_view_as_tree: false,
            sf_tree_lvl: 1,
            s_help: false,
        }
    }

    // *brakoll - d: make columns in print_contents function more dynamic to terminal size, p: 100, t: fix, s: closed
    fn print_contents(&mut self) {
        use terminal_size::{Width, terminal_size};

        if self.dir_items.is_empty() {
            return;
        }

        // width of the longest visible item
        let col_w = self
            .dir_items
            .iter()
            .map(|item| item.name.chars().count())
            .max()
            .unwrap_or(0) + 2;

        // get width, fall back 80
        let term_w = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80);

        // max 4 col and min 1
        let cols = (term_w / col_w).clamp(1, 4);

        // print
        for row in self.dir_items.chunks(cols) {
            for item in row {
                let visible_len = { item.name.chars().count() };

                let spaces = " ".repeat(col_w.saturating_sub(visible_len));
                print!("{}{}", item, spaces);
            }
            println!();
        }
    }

    fn is_executable(&mut self, metadata: &Metadata) -> bool {
        let permissions = metadata.permissions();
        return metadata.is_file() && permissions.mode() & 0o111 != 0;
    }

    fn get_contents(&mut self) -> io::Result<()> {
        let paths = fs::read_dir(&self.dir)?;
        for path in paths {
            let mut opath = path;
            let mut f_type: FileType;

            if opath.as_mut().unwrap().path().is_dir() {
                f_type = FileType::Directory;
            } else {
                f_type = opath
                    .as_mut()
                    .unwrap()
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(FileType::from_ext)
                    .unwrap_or(FileType::Unknown);
            }

            let mut name = opath.as_mut().unwrap().file_name().display().to_string();

            // *brakoll - d: remove need for is_exec field, p: 20, t: refactor, s: closed
            let mut is_symlink: bool = false;

            if let Ok(metadata) = opath.as_mut().unwrap().metadata() {
                if self.is_executable(&metadata) && f_type == FileType::Sh {
                    name = format!("{name}*");
                }

                is_symlink = metadata.is_symlink();
            }

            let mut is_hidden = false;

            if f_type == FileType::Directory && name.chars().nth(0) == Some('.') {
                f_type = FileType::DirHidden;
                is_hidden = true;
            }

            if f_type != FileType::Directory && name.chars().nth(0) == Some('.') {
                is_hidden = true;
            }

            // push
            self.dir_items.push(Item {
                f_type,
                is_symlink,
                is_hidden,
                name,
                abs_path: opath.as_mut().unwrap().path(),
            });
        }

        Ok(())
    }

    // *brakoll - d: remove devicon flag, p: 10, t: fix, s: closed
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
                "help" => {
                    self.s_help = true;
                    return Ok(());
                }
                "-a" => self.f_show_hidden = true,
                other => {
                    let path = PathBuf::from(other);
                    if path.exists() {
                        self.dir = path;
                    } else {
                        {}
                    }
                    break;
                }
            }
        }

        Ok(())
    }
}
