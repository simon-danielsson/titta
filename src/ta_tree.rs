use crate::Titta;
use crate::file_attr::lookup;

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

impl Titta {
    pub fn s_view_as_tree(&mut self) -> io::Result<()> {
        let root: PathBuf = if self.use_opt_dir {
            self.opt_dir.clone()
        } else {
            self.current_dir.clone()
        };

        // depth behavior
        let max_depth = self.sf_tree_lvl.max(0) as usize;

        if max_depth == 0 {
            return Ok(());
        }

        self.print_tree_dir(&root, "", 1, max_depth)?;
        Ok(())
    }

    fn print_tree_dir(
        &mut self,
        dir: &Path,
        prefix: &str,
        depth: usize,
        max_depth: usize,
    ) -> io::Result<()> {
        if depth > max_depth {
            return Ok(());
        }

        // collect and sort entries: directories first, then files
        let mut entries: Vec<fs::DirEntry> =
        fs::read_dir(dir)?.filter_map(Result::ok).collect();

        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name()
                    .to_string_lossy()
                    .cmp(&b.file_name().to_string_lossy()),
            }
        });

        // optionally filter hidden
        if !self.f_show_hidden {
            entries.retain(|e| {
                e.file_name()
                    .to_string_lossy()
                    .chars()
                    .next()
                    .map(|c| c != '.')
                    .unwrap_or(true)
            });
        }

        let len = entries.len();

        for (idx, entry) in entries.into_iter().enumerate() {
            let is_last = idx + 1 == len;
            let branch = if is_last { "╰── " } else { "├── " };

            let path = entry.path();
            let name_raw = entry.file_name().to_string_lossy().to_string();

            // symlink-aware metadata
            let md = fs::symlink_metadata(&path).ok();
            let is_symlink = md
                .as_ref()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);
            let is_dir = md.as_ref().map(|m| m.is_dir()).unwrap_or(path.is_dir());
            let is_file = md.as_ref().map(|m| m.is_file()).unwrap_or(path.is_file());

            // executable
            let is_exec = md
                .as_ref()
                .map(|m| is_file && (m.permissions().mode() & 0o111 != 0))
                .unwrap_or(false);

            // name formatting (optional exec marker)
            let mut name = name_raw.clone();
            if self.f_show_executables && is_exec {
                name.push('*');
            }

            let formatted = self.format_tree_item(&path, &name, is_dir);

            println!("{prefix}{branch}{formatted}");

            // recurse into directories (skip symlink dirs)
            if is_dir && !is_symlink && depth < max_depth {
                let next_prefix = if is_last {
                    format!("{prefix}    ")
                } else {
                    format!("{prefix}│   ")
                };
                self.print_tree_dir(&path, &next_prefix, depth + 1, max_depth)?;
            }
        }

        Ok(())
    }

    fn format_tree_item(&self, path: &Path, name: &str, is_dir: bool) -> String {
        // decide key for lookup()
        #[allow(unused)]
        let mut key = String::new();

        if is_dir {
            if name.starts_with('.') {
                key = "hidden_dir".to_string();
            } else {
                key = "dir".to_string();
            }
        } else {
            // extension, or
            key = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            if key.is_empty() {
                key = "".to_string();
            }
        }

        let (icon, color_code) = lookup(&key).unwrap_or_else(|| lookup("").unwrap());

        let icon_str = if self.f_use_devicons {
            format!("{} ", icon)
        } else {
            "".to_string()
        };

        let color_end = "\x1b[0m";
        let color = if self.f_with_color {
            color_code
        } else {
            color_end
        };

        format!("{color}{icon_str}{name}{color_end}")
    }
}
