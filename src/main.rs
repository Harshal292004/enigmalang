use std::{fs::File, io::Read, os::unix::fs::FileExt};

pub mod lexer;

fn main() {
    println!("Hello, Mystic!");
    panic!("xjgds")
}

enum Tokens {}

struct Lexer {}

impl Lexer {
    pub fn read_file(&self, file_path: &String) {}

    pub fn get_next_token(&self, file: &File) {}
}
