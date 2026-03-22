use std::fmt;

use crate::{Item, format};

// *brakoll - d: improve filetype attributes, p: 100, t: refactor, s: closed
macro_rules! file_types {
    ($($variant:ident => $ext:expr),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum FileType {
        $($variant),*
        }

        impl FileType {
        pub fn as_str(&self) -> &'static str {
        match self {
        $(Self::$variant => $ext),*
        }
        }

        pub fn from_ext(ext: &str) -> Self {
        match ext {
        $($ext => Self::$variant,)*
        _ => Self::Unknown,
        }
        }
        }
    };
}

file_types! {
    // === general
    Directory => "dir",
    DirHidden => "dir_hidden",
    Unknown => "unknown",

    // === dev
    Rs => "rs",
    Js => "js",
    Py => "py",
    Sh => "sh",
    C => "c",
    Toml => "toml",
    Json => "json",
    Typ => "typ",
    Html => "html",

    // === images
    Lua => "lua",
    Png => "png",
    Jpg => "jpg",
    Gif => "gif",
    Rw2 => "rw2",
    Raw => "raw",

    // === video
    Mp4 => "mp4",
    Mkv => "mkv",
    Mov => "mov",
    Avi => "avi",
    Webm => "webm",

    // === audio
    Wav => "wav",
    Flac => "flac",
    Aif => "aif",
    Aiff => "aiff",
    Mp3 => "mp3",

    // === text & pdf
    Log => "log",
    Txt => "txt",
    Md => "md",
    Pdf => "pdf",

    // === spreadsheet
    Xlsx => "xlsx",
    Xls => "xls",
    Gsheet => "gsheet",
    Numbers => "numbers",
    Csv => "csv",
    Tsv => "tsv",
    Ods => "ods",
    Xml => "xml",

    // === archives
    Zip => "zip",
    Tar => "tar",
    Rar => "rar",
    Gz => "gz",
    Tgz => "tgz",

    // === exe
    Bin => "bin",
    Run => "run",
    AppImage => "appimage",
    App => "app",
    Pkg => "pkg",
    Dmg => "dmg",

    // === font
    Ttf => "ttf",
    Otf => "otf",
}

pub enum EasyColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Orange,
    White,
}
impl fmt::Display for EasyColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            EasyColor::Red => "#FF0000",
            EasyColor::Green => "#00FF00",
            EasyColor::Yellow => "#FFFF00",
            EasyColor::Blue => "#0000FF",
            EasyColor::Magenta => "#FF00FF",
            EasyColor::Cyan => "#00FFFF",
            EasyColor::Orange => "#FFA500",
            EasyColor::White => "#FFFFFF",
        };

        write!(f, "{}", color)
    }
}

macro_rules! clr {
    ($color:ident) => {
        EasyColor::$color.to_string()
    };
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (color, content): (String, String) = match self.f_type {
            // general
            FileType::Directory => {
                (clr!(Blue), format!("{i} {n}", i = "󰉖", n = self.name))
            }
            FileType::DirHidden => {
                (clr!(Blue), format!("{i} {n}", i = "󱞋", n = self.name))
            }
            // dev
            FileType::Rs => (clr!(Red), format!("{i} {n}", i = "", n = self.name)),
            FileType::Js => (clr!(Yellow), format!("{i} {n}", i = "", n = self.name)),
            FileType::Py => (clr!(Yellow), format!("{i} {n}", i = "", n = self.name)),
            FileType::Sh => (clr!(Green), format!("{i} {n}", i = "", n = self.name)),
            FileType::C => (clr!(Cyan), format!("{i} {n}", i = "", n = self.name)),
            FileType::Toml => (clr!(Red), format!("{i} {n}", i = "", n = self.name)),
            FileType::Json => (clr!(Red), format!("{i} {n}", i = "", n = self.name)),
            FileType::Html => {
                (clr!(Magenta), format!("{i} {n}", i = "", n = self.name))
            }
            FileType::Typ => (clr!(Cyan), format!("{i} {n}", i = "", n = self.name)),
            FileType::Lua => (clr!(Blue), format!("{i} {n}", i = "", n = self.name)),
            // images
            FileType::Png
            | FileType::Jpg
            | FileType::Gif
            | FileType::Rw2
            | FileType::Raw => (clr!(Magenta), format!("{i} {n}", i = "", n = self.name)),
            // video
            FileType::Mp4
            | FileType::Mkv
            | FileType::Mov
            | FileType::Avi
            | FileType::Webm => (clr!(Magenta), format!("{i} {n}", i = "", n = self.name)),
            // audio
            FileType::Wav
            | FileType::Flac
            | FileType::Aif
            | FileType::Aiff
            | FileType::Mp3 => (clr!(Magenta), format!("{i} {n}", i = "", n = self.name)),
            // text & pdf
            FileType::Txt | FileType::Log => {
                (clr!(White), format!("{i} {n}", i = "", n = self.name))
            }
            FileType::Md => (clr!(Magenta), format!("{i} {n}", i = "", n = self.name)),
            FileType::Pdf => (clr!(White), format!("{i} {n}", i = "", n = self.name)),
            // spreadsheet
            FileType::Xls
            | FileType::Xlsx
            | FileType::Gsheet
            | FileType::Numbers
            | FileType::Csv
            | FileType::Tsv
            | FileType::Ods
            | FileType::Xml => (clr!(Cyan), format!("{i} {n}", i = "󰱿", n = self.name)),
            // archives
            FileType::Zip
            | FileType::Tar
            | FileType::Rar
            | FileType::Gz
            | FileType::Tgz => (clr!(Orange), format!("{i} {n}", i = "", n = self.name)),
            // exe
            FileType::Bin
            | FileType::Run
            | FileType::AppImage
            | FileType::App
            | FileType::Pkg
            | FileType::Dmg => (clr!(Yellow), format!("{i} {n}", i = "󰏗", n = self.name)),
            // font
            FileType::Ttf | FileType::Otf => {
                (clr!(Yellow), format!("{i} {n}", i = "", n = self.name))
            }
            // unknown
            FileType::Unknown => {
                (clr!(White), format!("{i} {n}", i = "", n = self.name))
            }
        };
        write!(f, "{}", format::apply_color(color, content))
    }
}