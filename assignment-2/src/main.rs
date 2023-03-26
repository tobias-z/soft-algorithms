#![allow(dead_code)]

pub mod binary_tree;
pub mod two_three_tree;
pub mod ceasar_decoder;

#[derive(Debug)]
enum X {
    Y(i32),
    Z
}

fn main() {
    println!("Crypt1 --------------");
    ceasar_decoder::decode("Crypt1.txt");
    println!("Crypt1 --------------");

    println!("Crypt2 --------------");
    ceasar_decoder::decode("Crypt2.txt");
    println!("Crypt2 --------------");

    println!("Crypt3 --------------");
    ceasar_decoder::decode("Crypt3.txt");
    println!("Crypt3 --------------");
}
