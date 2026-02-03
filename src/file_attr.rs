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
// | Grey    | `90` |
// | Reset   | `0`  |

/// key, icon<char>, color<&str>
pub const FILETYPE_ATTR: &[(&str, char, &str)] = &[
    // general
    ("dir", '󰉖', "\x1b[34m"),
    ("hidden_dir", '󱞋', "\x1b[34m"),
    ("", '', "\x1b[0m"),
    ("log", '', "\x1b[0m"),
    // dev
    ("rs", '', "\x1b[31m"),
    ("py", '', "\x1b[33m"),
    ("sh", '', "\x1b[32m"),
    ("js", '', "\x1b[33m"),
    ("c", '', "\x1b[36m"),
    ("toml", '', "\x1b[31m"),
    ("json", '', "\x1b[31m"),
    ("typ", '', "\x1b[36m"),
    // images
    ("png", '', "\x1b[32m"),
    ("jpg", '', "\x1b[32m"),
    ("gif", '', "\x1b[32m"),
    ("rw2", '', "\x1b[32m"),
    // video
    ("mp4", '', "\x1b[35m"),
    ("mkv", '', "\x1b[35m"),
    ("mov", '', "\x1b[35m"),
    ("avi", '', "\x1b[35m"),
    ("webm", '', "\x1b[35m"),
    // audio
    ("wav", '', "\x1b[35m"),
    ("flac", '', "\x1b[35m"),
    ("aif", '', "\x1b[35m"),
    ("aiff", '', "\x1b[35m"),
    ("mp3", '', "\x1b[35m"),
    // text & pdf
    ("txt", '', "\x1b[90"),
    ("md", '', "\x1b[35m"),
    ("pdf", '', "\x1b[31m"),
    // archives
    ("zip", '', "\x1b[31m"),
    ("tar", '', "\x1b[31m"),
    ("rar", '', "\x1b[31m"),
    ("tar.gz", '', "\x1b[31m"),
    ("tgz", '', "\x1b[31m"),
    ("7z", '', "\x1b[31m"),
    // executables
    ("bin", '󰏗', "\x1b[33m"),
    ("run", '', "\x1b[33m"),
    ("AppImage", '', "\x1b[33m"),
    ("app", '', "\x1b[33m"),
    ("pkg", '󰏗', "\x1b[33m"),
    ("dmg", '󰏗', "\x1b[33m"),
    // font
    ("ttf", '', "\x1b[0m"),
    ("otf", '', "\x1b[0m"),
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
