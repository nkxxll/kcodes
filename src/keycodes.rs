use std::str::FromStr;

#[derive(Default)]
pub struct Dimension {
    pub height: usize,
    pub width: usize,
}

#[derive(Default)]
pub struct Layer {
    pub state: String,
    pub dim: Dimension,
    pub count_keys: usize,
}

pub enum Keycode {
    Kc1,
    Kc2,
    Kc3,
    Kc4,
    Kc5,
    Kc6,
    Kc7,
    Kc8,
    Kc9,
    Kc0,
    KcQ,
    KcW,
    KcE,
    KcR,
    KcT,
    KcY,
    KcU,
    KcI,
    KcO,
    KcP,
    KcA,
    KcS,
    KcD,
    KcF,
    KcG,
    KcH,
    KcJ,
    KcK,
    KcL,
    KcZ,
    KcX,
    KcC,
    KcV,
    KcB,
    KcN,
    KcM,
    KcUnds,
    KcColn,
    KcQstn,
    KcExlm,
    KcAt,
    KcHash,
    KcDlr,
    KcPerc,
    KcCirc,
    KcAmpr,
    KcAstr,
    KcLprn,
    KcRprn,
    KcLcbr,
    KcRcbr,
    KcMins,
    KcPlus,
    KcEql,
    KcLbrc,
    KcRbrc,
    KcBsls,
    KcScln,
    KcQuot,
    KcComm,
    KcDot,
    KcSlsh,
    KcLt,
    KcGt,
    KcDquo,
    KcPipe,
    KcTrns,
}

pub(crate) struct StringToKeycodeError;

impl Keycode {
    pub(crate) fn as_str(self) -> String {
        match self {
            Keycode::Kc1 => String::from("KC_1"),
            Keycode::Kc2 => String::from("KC_2"),
            Keycode::Kc3 => String::from("KC_3"),
            Keycode::Kc4 => String::from("KC_4"),
            Keycode::Kc5 => String::from("KC_5"),
            Keycode::Kc6 => String::from("KC_6"),
            Keycode::Kc7 => String::from("KC_7"),
            Keycode::Kc8 => String::from("KC_8"),
            Keycode::Kc9 => String::from("KC_9"),
            Keycode::Kc0 => String::from("KC_0"),
            Keycode::KcA => String::from("KC_A"),
            Keycode::KcB => String::from("KC_B"),
            Keycode::KcC => String::from("KC_C"),
            Keycode::KcD => String::from("KC_D"),
            Keycode::KcE => String::from("KC_E"),
            Keycode::KcF => String::from("KC_F"),
            Keycode::KcG => String::from("KC_G"),
            Keycode::KcH => String::from("KC_H"),
            Keycode::KcI => String::from("KC_I"),
            Keycode::KcJ => String::from("KC_J"),
            Keycode::KcK => String::from("KC_K"),
            Keycode::KcL => String::from("KC_L"),
            Keycode::KcM => String::from("KC_M"),
            Keycode::KcN => String::from("KC_N"),
            Keycode::KcO => String::from("KC_O"),
            Keycode::KcP => String::from("KC_P"),
            Keycode::KcQ => String::from("KC_Q"),
            Keycode::KcR => String::from("KC_R"),
            Keycode::KcS => String::from("KC_S"),
            Keycode::KcT => String::from("KC_T"),
            Keycode::KcU => String::from("KC_U"),
            Keycode::KcV => String::from("KC_V"),
            Keycode::KcW => String::from("KC_W"),
            Keycode::KcX => String::from("KC_X"),
            Keycode::KcY => String::from("KC_Y"),
            Keycode::KcZ => String::from("KC_Z"),
            Keycode::KcDot => String::from("KC_DOT"),
            Keycode::KcComm => String::from("KC_COMM"),
            Keycode::KcScln => String::from("KC_SCLN"),
            Keycode::KcColn => String::from("KC_COLN"),
            Keycode::KcQstn => String::from("KC_QSTN"),
            Keycode::KcExlm => String::from("KC_EXLM"),
            Keycode::KcMins => String::from("KC_MINS"),
            Keycode::KcPlus => String::from("KC_PLUS"),
            Keycode::KcEql => String::from("KC_EQL"),
            Keycode::KcLbrc => String::from("KC_LBRC"),
            Keycode::KcRbrc => String::from("KC_RBRC"),
            Keycode::KcLcbr => String::from("KC_LCBR"),
            Keycode::KcRcbr => String::from("KC_RCBR"),
            Keycode::KcLprn => String::from("KC_LPRN"),
            Keycode::KcRprn => String::from("KC_RPRN"),
            Keycode::KcAmpr => String::from("KC_AMPR"),
            Keycode::KcPipe => String::from("KC_PIPE"),
            Keycode::KcCirc => String::from("KC_CIRC"),
            Keycode::KcPerc => String::from("KC_PERC"),
            Keycode::KcAt => String::from("KC_AT"),
            Keycode::KcHash => String::from("KC_HASH"),
            Keycode::KcDlr => String::from("KC_DLR"),
            Keycode::KcAstr => String::from("KC_ASTR"),
            Keycode::KcSlsh => String::from("KC_SLSH"),
            Keycode::KcBsls => String::from("KC_BSLS"),
            Keycode::KcQuot => String::from("KC_QUOT"),
            Keycode::KcUnds => String::from("KC_UNDS"),
            Keycode::KcLt => String::from("KC_LT"),
            Keycode::KcGt => String::from("KC_GT"),
            Keycode::KcDquo => String::from("KC_DQUO"),
            Keycode::KcTrns => String::from("KC_TRNS"),
        }
    }
}

impl FromStr for Keycode {
    type Err = StringToKeycodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s {
            "1" => Ok(Keycode::Kc1),
            "2" => Ok(Keycode::Kc2),
            "3" => Ok(Keycode::Kc3),
            "4" => Ok(Keycode::Kc4),
            "5" => Ok(Keycode::Kc5),
            "6" => Ok(Keycode::Kc6),
            "7" => Ok(Keycode::Kc7),
            "8" => Ok(Keycode::Kc8),
            "9" => Ok(Keycode::Kc9),
            "0" => Ok(Keycode::Kc0),
            "a" | "A" => Ok(Keycode::KcA),
            "b" | "B" => Ok(Keycode::KcB),
            "c" | "C" => Ok(Keycode::KcC),
            "d" | "D" => Ok(Keycode::KcD),
            "e" | "E" => Ok(Keycode::KcE),
            "f" | "F" => Ok(Keycode::KcF),
            "g" | "G" => Ok(Keycode::KcG),
            "h" | "H" => Ok(Keycode::KcH),
            "i" | "I" => Ok(Keycode::KcI),
            "j" | "J" => Ok(Keycode::KcJ),
            "k" | "K" => Ok(Keycode::KcK),
            "l" | "L" => Ok(Keycode::KcL),
            "m" | "M" => Ok(Keycode::KcM),
            "n" | "N" => Ok(Keycode::KcN),
            "o" | "O" => Ok(Keycode::KcO),
            "p" | "P" => Ok(Keycode::KcP),
            "q" | "Q" => Ok(Keycode::KcQ),
            "r" | "R" => Ok(Keycode::KcR),
            "s" | "S" => Ok(Keycode::KcS),
            "t" | "T" => Ok(Keycode::KcT),
            "u" | "U" => Ok(Keycode::KcU),
            "v" | "V" => Ok(Keycode::KcV),
            "w" | "W" => Ok(Keycode::KcW),
            "x" | "X" => Ok(Keycode::KcX),
            "y" | "Y" => Ok(Keycode::KcY),
            "z" | "Z" => Ok(Keycode::KcZ),
            "." => Ok(Keycode::KcDot),
            "," => Ok(Keycode::KcComm),
            ";" => Ok(Keycode::KcScln),
            ":" => Ok(Keycode::KcColn),
            "?" => Ok(Keycode::KcQstn),
            "!" => Ok(Keycode::KcExlm),
            "-" => Ok(Keycode::KcMins),
            "+" => Ok(Keycode::KcPlus),
            "=" => Ok(Keycode::KcEql),
            "[" => Ok(Keycode::KcLbrc),
            "]" => Ok(Keycode::KcRbrc),
            "{" => Ok(Keycode::KcLcbr),
            "}" => Ok(Keycode::KcRcbr),
            "(" => Ok(Keycode::KcLprn),
            ")" => Ok(Keycode::KcRprn),
            "&" => Ok(Keycode::KcAmpr),
            "|" => Ok(Keycode::KcPipe),
            "^" => Ok(Keycode::KcCirc),
            "%" => Ok(Keycode::KcPerc),
            "@" => Ok(Keycode::KcAt),
            "#" => Ok(Keycode::KcHash),
            "$" => Ok(Keycode::KcDlr),
            "*" => Ok(Keycode::KcAstr),
            "/" => Ok(Keycode::KcSlsh),
            "\\" => Ok(Keycode::KcBsls),
            "\"" => Ok(Keycode::KcQuot),
            "__" => Ok(Keycode::KcTrns),
            _ => Err(StringToKeycodeError),
        }
    }
}

impl Layer {
    /// create the first line with the name of the layer of keycodes
    pub(crate) fn new(name: String, height: usize, width: usize) -> Self {
        Self {
            state: format!("[{}] = LAYOUT_ergodox_pretty(\n", name).to_string(),
            dim: Dimension { height, width },
            count_keys: 0,
        }
    }
    pub(crate) fn add_key(&mut self, code: Keycode) {
        // todo: this has to have a more elegant solution because there are not always 12 keys in a row
        self.state.push_str(&code.as_str());
        if self.count_keys % self.dim.width == 0 {
            self.state.push_str("\n");
        }
        if self.count_keys % self.dim.width == 1 {
            self.state.push_str("\t");
        }
        self.count_keys += 1;
    }

    pub(crate) fn add_default_thumb(&mut self) {
        self.state.push_str("KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT,\n\t\t\t\tKC_TRANSPARENT, KC_TRANSPARENT,\n\t\tKC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT, KC_TRANSPARENT\n");
    }

    pub(crate) fn close_layer(&mut self) {
        self.state.push_str("\n},\n");
    }

    pub(crate) fn get_layer(&self) -> String {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn little_parsing_test() {
        todo!();
    }
}
