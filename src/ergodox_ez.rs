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
        let header: String =
            format!("[{}] = LAYOUT_ergodox_pretty(\n", self.layer.name).to_string();
        let first_line_format: String =
            String::from("\t{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\n");
        let second_line_format: String =
            String::from("\t{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\n");
        let third_line_format: String =
            String::from("\t\t{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\n");
        let fourth_line_format: String =
            String::from("\t{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},\n");
        let fifth_line_format: String =
            String::from("\t\t{}, {}, {}, {}, {}, {}, {}, {}, {}, {},\n");
        let thumb_format: String = String::from("\t\t\t{}, {}, {}, {}, {}, {}, {},\n");
        let finish: String = String::from(")\n");
        res.push_str(&header);
        res.push_str(&first_line_format);
        res.push_str(&second_line_format);
        res.push_str(&third_line_format);
        res.push_str(&fourth_line_format);
        res.push_str(&fifth_line_format);
        res.push_str(&thumb_format);
        res.push_str(&finish);
        dbg!(&self.layer.keys_to_string_vec());
        res = res.format(&self.layer.keys_to_string_vec());
        res
    }
}
