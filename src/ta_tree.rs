use crate::Titta;
use std::io;

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
