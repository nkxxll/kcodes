use std::fs;
use std::str::FromStr;

use clap::Parser;

use crate::keycodes::Keycode;
use crate::keycodes::Layer;

mod ergodox_ez;
mod keycodes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Short string for the layer name
    #[arg(short, long, default_value = "BASE")]
    name: String,
    #[arg(
        help = "this is the key code file you have to provide to be translated to a c array with key codes"
    )]
    file: String,
    #[arg(short, long, default_value = "14")]
    columns: usize,
    #[arg(short, long, default_value = "5")]
    rows: usize,
}

fn main() {
    // get the cli args
    let args = Args::parse();
    let input = fs::read_to_string(&args.file).expect("No file with this name!");
    let key_lines: Vec<&str> = input.lines().collect();
    assert_eq!(key_lines.len(), args.rows);
    let keys = map(key_lines);
    assert_eq!(keys.len(), args.rows * args.columns);
    print!(
        r#"[{}] = LAYOUT_ergodox_pretty(
    VRSN,    KC_F1,   KC_F2,   KC_F3,   KC_F4,   KC_F5,   KC_TRNS,     KC_TRNS, KC_F6,   KC_F7,   KC_F8,   KC_F9,   KC_F10,  KC_F11,
    KC_TRANSPARENT, {}, {},   {}, {}, {},    KC_TRANSPARENT,KC_TRANSPARENT,          {}, {},    {},    {},    {},       KC_F12,
    KC_TRANSPARENT, {}, {}, {}, {}, {},                                            {}, {},    {},    {},    {},         KC_TRANSPARENT,
    KC_TRANSPARENT, {},  {}, {}, {}, {},    KC_TRANSPARENT, KC_TRANSPARENT,         {}, {},    {},    {},    {},      KC_TRANSPARENT,
    KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,
                                                        KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,
                                                                        KC_TRANSPARENT, KC_TRANSPARENT,
                                        KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT
),
    "#,
        args.name,
        keys[0],
        keys[1],
        keys[2],
        keys[3],
        keys[4],
        keys[5],
        keys[6],
        keys[7],
        keys[8],
        keys[9],
        keys[10],
        keys[11],
        keys[12],
        keys[13],
        keys[14],
        keys[15],
        keys[16],
        keys[17],
        keys[18],
        keys[19],
        keys[20],
        keys[21],
        keys[22],
        keys[23],
        keys[24],
        keys[25],
        keys[26],
        keys[27],
        keys[28],
        keys[29]
    );
}

fn map(lines: Vec<&str>) -> Vec<&str> {
    let mut res: Vec<&str> = vec![];
    for line in lines {
        let split = line.split_whitespace();
        for key in split {
            match key {
                "1" => res.push("KC_1"),
                "2" => res.push("KC_2"),
                "3" => res.push("KC_3"),
                "4" => res.push("KC_4"),
                "5" => res.push("KC_5"),
                "6" => res.push("KC_6"),
                "7" => res.push("KC_7"),
                "8" => res.push("KC_8"),
                "9" => res.push("KC_9"),
                "0" => res.push("KC_0"),
                "q" => res.push("KC_Q"),
                "w" => res.push("KC_W"),
                "e" => res.push("KC_E"),
                "r" => res.push("KC_R"),
                "t" => res.push("KC_T"),
                "y" => res.push("KC_Y"),
                "u" => res.push("KC_U"),
                "i" => res.push("KC_I"),
                "o" => res.push("KC_O"),
                "p" => res.push("KC_P"),
                "a" => res.push("KC_A"),
                "s" => res.push("KC_S"),
                "d" => res.push("KC_D"),
                "f" => res.push("KC_F"),
                "g" => res.push("KC_G"),
                "h" => res.push("KC_H"),
                "j" => res.push("KC_J"),
                "k" => res.push("KC_K"),
                "l" => res.push("KC_L"),
                "z" => res.push("KC_Z"),
                "x" => res.push("KC_X"),
                "c" => res.push("KC_C"),
                "v" => res.push("KC_V"),
                "b" => res.push("KC_B"),
                "n" => res.push("KC_N"),
                "m" => res.push("KC_M"),
                "_" => res.push("KC_UNDS"),
                "!" => res.push("KC_EXLM"),
                "@" => res.push("KC_AT"),
                "#" => res.push("KC_HASH"),
                "$" => res.push("KC_DLR"),
                "%" => res.push("KC_PERC"),
                "^" => res.push("KC_CIRC"),
                "&" => res.push("KC_AMPR"),
                "*" => res.push("KC_ASTR"),
                "(" => res.push("KC_LPRN"),
                ")" => res.push("KC_RPRN"),
                "{" => res.push("KC_LCBR"),
                "}" => res.push("KC_RCBR"),
                "-" => res.push("KC_MINS"),
                "+" => res.push("KC_PLUS"),
                "=" => res.push("KC_EQL"),
                "[" => res.push("KC_LBRC"),
                "]" => res.push("KC_RBRC"),
                "\\" => res.push("KC_BSLS"),
                ";" => res.push("KC_SCLN"),
                "'" => res.push("KC_QUOT"),
                "," => res.push("KC_COMM"),
                "." => res.push("KC_DOT"),
                "/" => res.push("KC_SLSH"),
                "<" => res.push("KC_LT"),
                ">" => res.push("KC_GT"),
                "\"" => res.push("KC_DQUO"),
                "|" => res.push("KC_PIPE"),
                _ => res.push("KC_TRNS"),
            }
        }
    }
    res
}
