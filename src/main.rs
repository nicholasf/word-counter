use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let filename = env::args().nth(1).expect("No filename given.");
    let word = env::args().nth(2).expect("No word given.");
    println!("filename: {}", filename);
    println!("word: {}", word);
}
