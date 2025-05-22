pub mod tokens;
pub mod size;
use std::str::CharIndices;
use std::iter::Peekable;

use size::Size;
use tokens::Token;
// FIXME : Add Peekable to improve lexer performance as program.chars().nth(0) is O(n) and expensive
// FIXME : If we simply use program.chars().next()  it will consume the value rather we do program.chars().peek() it doesnt consume 
// FUCK : New discoveries you can't slice a UTF-8 string directly in rust it may panic as in rust slices string on there byte offset boundaries rather than there actual indices
// Thus you also need CharIndices as it procides the actual byte offset indexing  
#[derive(Debug)]
pub struct Lexer<'l>{
    // we keep program to have a lifetime denoting it doesn't dies in the entire life time of lexer
    // thus safe to use 
    program: &'l str, 
    chars: Peekable<CharIndices<'l>>
}

impl <'l> Lexer<'l> {
    // intialize lexer with cursor to 0 as pointer is at the start of the program
    pub fn new(program:&'l str)-> Self{
        Self{
            program:program,
            chars: program.char_indices().peekable()
        }
    }

    fn peek_char(&mut self)->Option<(usize, char)>{
        self.chars.peek().copied()
    }
    fn next_char(&mut self)->Option<(usize, char)>{
        self.chars.next()
    }
    pub fn skip_whitespace(&mut self){
        while let Some(c) =  self.peek_char() {
            if c.1.is_whitespace(){
                // continue 
                self.next_char();
            }else{
                break;
            }
        }        
    }
    pub fn next_token(&mut self)->Option<Token>{

        use tokens::TokenType::*;
        // skip the white spaces in the code
        self.skip_whitespace();

        let (curr_offset,curr_char)=match self.peek_char(){
            Some(c)=>c,
            None=>return Some(Token{
                token_type:Eof,
                size:Size{start:0,end:0}
            })
        };

        let token = match curr_char {
            '#' =>{
                // case 1 : it's single line comment
                // case 2 : there are actually only 2 ## and  its still single line comment
                // for checking if its single line comment we just need to move it to next and peek for char after it
                // if the char isn't # no worry just drop that line
                // and then check the next char as well after 
                self.next_char();
                // let (hash_offset, hash_char) = match self.peek_char(){
                //     Some(h)=>{
                //         if(h.1=='#'){
                //             // check for multi line comments
                //             self.next_char(); // as we know here we have 2 hash
                //             // now we need to check whether we have 3 hashs?
                //             let (third_hash_offset,third_hash_char)= match self.peek_char(){
                //                 Some(t)=>{
                //                     t
                //                 }
                //             } 
                //         }else{
                //             // advance to the next white space()
                //             while let Some(t) = self.peek_char() {
                //                 if t.1 != '\n'{
                //                     self.next_char();   
                //                 }
                //             }

                //             self.next_char(); // there will be atleast a single new line token 
                //         }
                //     },
                //     None=>{

                //     }

                // }


                

                
                // case 2 : it's multi line comment

                Token {
                    token_type: As,
                    size: Size { start: 1, end: 1 },
                }
            },
            _ => return None,
        };
    
        Some(token)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_whitespace() {
        let program="
        
        
            # The first rule of fight club is  
            ### That you do not talk about fight club
            The second rule of fight club is 
            That you DO NOT talk about fight club
            ###

            spell double(val:int):int{
                chant val*2
            }

            bind val:int=8
            double(8)
        ";
        let mut lexer=Lexer::new(program);
        lexer.skip_whitespace();

        if let Some((_, ch)) = lexer.peek_char() {
            assert_eq!(ch, '#', "Expected '#' after whitespace, got '{}'", ch);
        } else {
            assert!(false, "Expected some character after whitespace, but found None");
        }
    }

    #[test]
    fn get_eof_token(){
        let program=" ";
        let mut lexer=Lexer::new(program);

        let token =lexer.next_token();

        match token{
            Some(t)=>{
                assert_eq!(t,Token{token_type:tokens::TokenType::Eof,size:Size{start:0,end:0}} , "Exepcetd a token of type Eof but got '{:?}' ",t);
            },
            None=>{
                assert!(false,"Expected token of type Eof but got None");
            }

        }
    }
}
