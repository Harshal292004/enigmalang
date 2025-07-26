pub mod lexer;
pub mod  parser;
use lexer::Lexer;
use lexer::tokens::{Token,TokenType};
use parser::Parser;
use std::fs;

fn get_token_stream(lex:Lexer){
    
   loop{

    let mut token_type:TokenType;

    let token=  lex.next();

    match token.{
        TokenType::Eof => None,
        _ => Some(token),

    }
    if tok_type == TokenType::Eof{
        break;
    }
   }
    while tok_type!= TokenType::Eof {
        
    }
}
fn main() {
    println!("Hello, Mystic!");
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    let lex= Lexer::new(&contents);
    let parse =  Parser::new(token_stream);
}
