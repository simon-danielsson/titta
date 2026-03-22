use raster::Color;
use regex::Regex;
use std::str;

// *brakoll - d: make use of color flag so the user can choose if color is applied, p: 100, t: fix, s: closed
pub const RESET: &str = "\x1b[m";
pub const FG: &str = "\x1b[38;2;";

pub fn apply_color(fg_hex: String, txt: String) -> String {
    let rgb_fg: Color;

    let ansi_fg: String;

    // Validate hex
    let is_hex_fg: bool = val_hex(&fg_hex);

    if is_hex_fg {
        _ = fg_hex.to_lowercase();
        rgb_fg = Color::hex(fg_hex.as_str()).unwrap();
        ansi_fg = FG.to_string()
            + &rgb_fg.r.to_string()
            + ";" + &rgb_fg.g.to_string()
            + ";" + &rgb_fg.b.to_string()
            + "m";
    } else {
        ansi_fg = "".to_string();
    }

    format!("{}{}{}", &ansi_fg.to_string(), &txt, &(RESET))
}

fn val_hex(target: &str) -> bool {
    let mut is_hex: bool = false;

    if target == "" {
        is_hex = false;
        return is_hex;
    }
    if target.len() != 7 {
        is_hex = false;
        return is_hex;
    }

    let splited_target = target.chars();

    // regex
    let re = Regex::new(r"[0-9a-f]+").unwrap();

    // counter for loop
    let mut i: u8 = 0;

    for c in splited_target {
        let c_string = c.to_string().to_lowercase();

        if i == 0 {
            if &c_string == "#" {
                is_hex = true;
            } else {
                is_hex = false;
                return is_hex;
            }
        } else {
            if re.is_match(&c_string) {
                is_hex = true;
            } else {
                is_hex = false;
                return is_hex;
            }
        }

        i += 1;
    }

    is_hex
}