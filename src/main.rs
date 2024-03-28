use std::fs;

use clap::Parser;
use ergodox_ez::Ergodox;

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
    #[arg(
        short,
        long = "type",
        help = "this is the key code file you have to provide to be translated to a c array with key codes",
        default_value = "ergodox"
    )]
    type_: String,
}

fn main() {
    // get the cli args
    let args = Args::parse();
    let input = fs::read_to_string(&args.file).expect("No file with this name!");
    let mut l = Layer::new(args.name);
    l.parse_string_to_keycodes(input);
    let e = Ergodox::new(l);
    print!("{}", e.create_layer())
}
