use crate::attributes::FileType;
use crate::{Item, Titta};
use std::io;

// *brakoll - d: overhaul tree generation to minimize duplicate code, p: 100, t: refactor, s: closed
impl Titta {
    pub fn s_view_as_tree(&self) -> io::Result<String> {
        let mut out = String::new();

        let root = if self.use_opt_dir {
            &self.opt_dir
        } else {
            &self.current_dir
        };

        let root_name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| root.display().to_string());

        out.push_str(&root_name);
        out.push('\n');

        let mut items = self.dir_items.clone();
        items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let max_depth = self.sf_tree_lvl.max(1) as usize;

        for (idx, item) in items.iter().enumerate() {
            let is_last = idx + 1 == items.len();
            self.write_tree_item(item, "", 1, max_depth, is_last, &mut out)?;
        }

        Ok(out)
    }
    fn write_tree_item(
        &self,
        item: &Item,
        prefix: &str,
        depth: usize,
        max_depth: usize,
        is_last: bool,
        out: &mut String,
    ) -> io::Result<()> {
        let branch = if is_last { "└── " } else { "├── " };

        out.push_str(prefix);
        out.push_str(branch);
        out.push_str(&format!("{}", item));
        out.push('\n');

        if depth >= max_depth {
            return Ok(());
        }

        if matches!(item.f_type, FileType::Directory | FileType::DirHidden) {
            let mut children = self.dir_items.clone();

            children.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            let next_prefix =
            format!("{prefix}{}", if is_last { "    " } else { "│   " });

            for (idx, child) in children.iter().enumerate() {
                let child_is_last = idx + 1 == children.len();

                self.write_tree_item(
                    child,
                    &next_prefix,
                    depth + 1,
                    max_depth,
                    child_is_last,
                    out,
                )?;
            }
        }

        Ok(())
    }
}