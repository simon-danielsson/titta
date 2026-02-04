// ===
//
// TRANSPARENCY!
//
// for the code structure of my tree generation
// I have taken some ideas from the eza crate
//
// repo: https://github.com/eza-community/eza
//
// ===

use crate::Titta;
use std::io;

#[allow(unused)]
pub enum TreePart {
    /// Rightmost column, *not* the last in the directory.
    Edge,
    /// Not the rightmost column, and the directory has not finished yet.
    Line,
    /// Rightmost column, and the last in the directory.
    Corner,
    /// Not the rightmost column, and the directory *has* finished.
    Blank,
}

#[allow(unused)]
impl TreePart {
    /// Turn this tree part into ASCII-licious box drawing characters!
    /// (Warning: not actually ASCII)
    pub fn ascii_art(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Edge    => "├── ",
            Self::Line    => "│   ",
            Self::Corner  => "└── ",
            Self::Blank   => "    ",
        };
    }
}

impl Titta {
    /// subcommand
    pub fn s_view_as_tree(&mut self) -> io::Result<()> {
        println!("tree with level: {}", self.sf_tree_lvl);
        println!("color: {}, dev: {}", self.f_with_color, self.f_use_devicons);
        println!("current dir: {:?}", self.current_dir);
        println!("chosen dir: {:?}", self.opt_dir);
        println!("use chosen dir: {}", self.use_opt_dir);
        Ok(())
    }
}
