pub mod tokens;
pub mod size;
use std::str::CharIndices;

use size::Size;
use tokens::{*,TokenType::*};
use std::iter::Peekable;
// FIXME : Add Peekable to improve lexer performance as program.chars().nth(0) is O(n) and expensive
// FIXME : If we simply use program.chars().next()  it will consume the value rather we do program.chars().peek() it doesnt consume 
// FUCK : New discoveries you can't slice a UTF-8 string directly in rust it may panic as in rust slices string on there byte offset boundaries rather than there actual indices
// Thus you also need CharIndices as it procides the actual byte offset indexing  
// program is the string of program
// cursor is the current location of the cursor in program 
pub struct Lexer<'l>{
    chars: Peekable<CharIndices<'l>>
}

// impl Lexer {
//     // intialize lexer with cursor to 0 as pointer is at the start of the program
//     fn new(program:String)-> Self{
//         Self{
//             program:program,
//             cursor:0
//         }
//     }
//     // we want to check whats the next cahracter without moving the cursor
//     fn peek_char(&self,position:usize)->Option<char>{
//         // position is for getting the nth char from current step 
//         let peaked_char = self.program.chars().nth(self.cursor+position);
//         peaked_char
//     }

//     // move cursor to next character
//     fn next_char(&mut self)->Option<char>{
//         self.cursor=self.cursor+1; 
//         let next_char= self.program.chars().nth(self.cursor);
//         next_char
//     }

//     fn read_indentifier_or_keyword()->Token{
//         Token { token_type: And, size: Size{start:0,end:4} }
//     }

//     fn read_number()->Token{
//         Token { token_type: And, size: Size{start:0,end:4} }

//     }

//     fn read_string()->Token{
//         Token { token_type: And, size: Size{start:0,end:4} }

//     }

//     // skip whitespaces in program
//     fn skip_whitespace(&mut self){
//         while let Some(c)= self.peek_char(0) {
//             if c.is_whitespace() {
//                 self.next_char();
//             }else{
//                 break;
//             }
//         }
//     }

//     // we get next token
//     fn next_token(&mut self){

//         // skip whitespaces
//         self.skip_whitespace();

//     }

// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input(){
        {
        let program= "bind val:int= 3;\n";
        let lexer= Lexer{chars:program.char_indices().peekable()};
        }
    
    }
}
