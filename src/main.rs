mod shapes;

use std::env;
use crate::shapes::Empty;

fn main() {
    let filename = env::args().nth(1).expect("No filename given.");
    let word = env::args().nth(2).expect("No word given.");
    println!("filename: {}", filename);
    println!("word: {}", word);
    let _ne = Empty{};
}
