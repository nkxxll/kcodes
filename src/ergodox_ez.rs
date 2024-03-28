use crate::keycodes::Layer;
use dyn_fmt::AsStrFormatExt;

pub struct Ergodox {
    pub layer: Layer,
}

impl Ergodox {
    pub fn new(l: Layer) -> Self {
        Ergodox { layer: l }
    }
    pub fn create_layer(&self) -> String {
        let mut res: String = "".to_string();
        let header: String = format!("[{}] = LAYOUT_ergodox_pretty(", self.layer.name).to_string();
        // note: first second and fourth line have 14 keys
        // thrid has 12 keys
        // fifth has 10 keys
        let first_line_format: String =
            String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        let second_line_format: String =
            String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        let third_line_format: String =
            String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        let fourth_line_format: String =
            String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        let fifth_line_format: String = String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        let thumb_format: String = String::from("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},");
        res.push_str(&header);
        res.push_str(&first_line_format);
        res.push_str(&second_line_format);
        res.push_str(&third_line_format);
        res.push_str(&fourth_line_format);
        res.push_str(&first_line_format);
        res.push_str(&fifth_line_format);
        res.push_str(&thumb_format);
        res.format(&self.layer.keys_to_string_vec());
        res
    }
}
