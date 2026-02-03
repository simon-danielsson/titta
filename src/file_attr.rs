pub enum AnsiCol {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Orange,
    Reset,
}

impl AnsiCol {
    pub const fn as_str(&self) -> &'static str {
        match self {
            AnsiCol::Red => "\x1b[31m",
            AnsiCol::Green => "\x1b[32m",
            AnsiCol::Yellow => "\x1b[33m",
            AnsiCol::Blue => "\x1b[34m",
            AnsiCol::Magenta => "\x1b[35m",
            AnsiCol::Cyan => "\x1b[36m",
            AnsiCol::Orange => "\x1b[38m",
            AnsiCol::Reset => "\x1b[0m",
        }
    }
}

macro_rules! ansi {
    ($color:ident) => {
        AnsiCol::$color.as_str()
    };
}

/// key, icon<char>, color<&str>
pub const FILETYPE_ATTR: &[(&str, char, &str)] = &[
    // general
    ("dir", '󰉖', ansi!(Blue)),
    ("hidden_dir", '󱞋', ansi!(Blue)),
    ("", '', ansi!(Reset)),
    ("log", '', ansi!(Reset)),
    // dev
    ("rs", '', ansi!(Red)),
    ("py", '', ansi!(Yellow)),
    ("sh", '', ansi!(Green)),
    ("js", '', ansi!(Yellow)),
    ("c", '', ansi!(Cyan)),
    ("toml", '', ansi!(Red)),
    ("json", '', ansi!(Red)),
    ("typ", '', ansi!(Cyan)),
    // images
    ("png", '', ansi!(Magenta)),
    ("jpg", '', ansi!(Magenta)),
    ("gif", '', ansi!(Magenta)),
    ("rw2", '', ansi!(Magenta)),
    // video
    ("mp4", '', ansi!(Green)),
    ("mkv", '', ansi!(Green)),
    ("mov", '', ansi!(Green)),
    ("avi", '', ansi!(Green)),
    ("webm", '', ansi!(Green)),
    // audio
    ("wav", '', ansi!(Reset)),
    ("flac", '', ansi!(Reset)),
    ("aif", '', ansi!(Reset)),
    ("aiff", '', ansi!(Reset)),
    ("mp3", '', ansi!(Reset)),
    // text & pdf
    ("txt", '', ansi!(Reset)),
    ("md", '', ansi!(Magenta)),
    ("pdf", '', ansi!(Cyan)),
    // archives
    ("zip", '', ansi!(Orange)),
    ("tar", '', ansi!(Orange)),
    ("rar", '', ansi!(Orange)),
    ("tar.gz", '', ansi!(Orange)),
    ("tgz", '', ansi!(Orange)),
    ("7z", '', ansi!(Orange)),
    // executables
    ("bin", '󰏗', ansi!(Yellow)),
    ("run", '', ansi!(Yellow)),
    ("AppImage", '', ansi!(Yellow)),
    ("app", '', ansi!(Yellow)),
    ("pkg", '󰏗', ansi!(Yellow)),
    ("dmg", '󰏗', ansi!(Yellow)),
    // font
    ("ttf", '', ansi!(Reset)),
    ("otf", '', ansi!(Reset)),
];

/// helper: lookup()
pub const fn str_eq(a: &str, b: &str) -> bool {
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
pub const fn lookup(key: &str) -> Option<(char, &str)> {
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
