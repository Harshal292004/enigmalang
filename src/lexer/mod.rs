pub mod position;
pub mod tokens;

use std::str::Chars;

// program is the string of program
// cursor is the current location of the cursor in program 
pub struct Lexer{
    program: String,
    cursor: usize
}

impl Lexer {

    // intialize lexer with cursor to 0 as pointer is at the start of the program
    fn new(program:String)-> Self{
        Self{
            program:program,
            cursor:0
        }
    }
    // we want to check whats the next cahracter without moving the cursor
    fn peak(&self,position:usize){
        let peaked_char = self.program.chars().nth(self.cursor+position).unwrap();
        
    }

    // move cursor to next character
    fn next(){

    }

    // skip whitespaces in program
    fn skip_whitespace(){

    }

    // we get next token
    fn next_token(){

    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input(){
        let program= "bind val:int= 3;\n";
        let lexer= Lexer::new(program.to_string());

        lexer.peak(3);
    }
}
