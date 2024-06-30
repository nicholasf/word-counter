mod shapes;

use std::env;
use crate::shapes::word::{ Empty, Word };

fn main() {
    let filename = env::args().nth(1).expect("No filename given.");
    let word = env::args().nth(2).expect("No word given.");
    println!("filename: {}", filename);
    println!("word: {}", word);

    // temp to make the compiler happy
    let _ne = Empty{};
    let _w = Word { word: "Hello".to_string() , id: 0 };
}
