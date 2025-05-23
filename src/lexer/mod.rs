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
    fn skip_whitespace(&mut self){
        while let Some(c) =  self.peek_char() {
            if c.1.is_whitespace(){
                // continue 
                self.next_char();
            }else{
                break;
            }
        }        
    }   
    fn skip_single_line_comment(&mut self){
         // single line comment
         while let Some((_,ch)) = self.peek_char(){
            if ch=='\n'{
             //TODO:  I need to check this kinda correct
             self.next_char();
             break;
            }
            self.next_char();
         }
    }
    fn skip_comments(&mut self){
         //  consume first #
         self.next_char();

         if let Some((_,'#'))= self.peek_char(){
            // consume second #
            self.next_char();


            if let Some((_,'#'))= self.peek_char(){
                // consume third #
                self.next_char(); 
                loop{
                    match self.peek_char() {
                        Some((_,'#'))=>{
                            // consume the first # 
                            self.next_char();
                            if let Some((_,'#'))=self.peek_char(){
                                // consume second # in ending
                                self.next_char();
                                
                                if let Some((_,'#'))= self.peek_char(){
                                    // consume third # in the ending 
                                    self.next_char();
                                    // EOL or say new line or we stop here 
                                    break;
                                }
                            }
                        },
                        Some(_)=>{
                            self.next_char();
                        }
                        None=>{
                            break;
                        }
                    }
                }

            }else{
                // single line coment
                self.next_char();
                self.skip_single_line_comment();
            }
            
         }
         else{
            self.skip_single_line_comment();
         }
    }

    fn single_char_token(&self,start:usize,token_type:tokens::TokenType)->Token{
        Token{token_type:token_type,size:Size{start:start,end:start+1}}
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
                self.skip_comments();
                // this is intresting as this will help to avoid all of the comments and actually move towards the next token and then return that using recurrsion
                return  self.next_token();
            },

            // single char tokens 
            '{'=> self.single_char_token(curr_offset, LCurly),
            '}'=> self.single_char_token(curr_offset, RCurly),
            '('=>  self.single_char_token(curr_offset, LParen),
            ')'=>  self.single_char_token(curr_offset, RParen),
            ':'=>  self.single_char_token(curr_offset, Colon),
            ','=>  self.single_char_token(curr_offset, Comma),
            '.'=>  self.single_char_token(curr_offset,Dot),
            '='=>{

                // consume = 
                self.next_char();
                // case 1 : only equals 
                let peek= self.peek_char();


                match peek {
                    Some((_,'='))=>{
                        self.next_char();
                        Token{token_type:EqualEqual,size:Size{start:curr_offset,end:2}}
                    },
                    _ => Token {
                        token_type: Assign,
                        size: Size {
                            start: curr_offset,
                            end: curr_offset + 1,
                        },
                    },
                }
            }
            // Operators
            '0'=>  self.single_char_token(curr_offset, LCurly),
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

    #[test]
    fn verify_single_line_comment() {
        let program = "# sjdfsiljd";
        let mut lexer = Lexer::new(program);
    
        let token = lexer.next_token();
        assert_eq!(
            token,
            Some(Token {
                token_type: tokens::TokenType::Eof,
                size: Size { start: 0, end: 0 }
            }),
            "Failed to handle basic single-line comment"
        );
    }

    #[test]
    fn verify_proper_multiline_comment() {
        let program = "
            # intro
            ###
            slkefslekfme
            ###
        ";
        let mut lexer = Lexer::new(program);

        let token = lexer.next_token();
        assert_eq!(
            token,
            Some(Token {
                token_type: tokens::TokenType::Eof,
                size: Size { start: 0, end: 0 }
            }),
            "Failed to handle  multi-line comment block"
        );
    }



    #[test]
    fn verify_padded_multiline_comment() {
        let program = "
            #####
            dsgsegpsegoks
            #####
        ";
        let mut lexer = Lexer::new(program);

        let token = lexer.next_token();
        assert_eq!(
            token,
            Some(Token {
                token_type: tokens::TokenType::Eof,
                size: Size { start: 0, end: 0 }
            }),
            "Failed to skip padded multi-line comment with extra hashes"
        );
    }

    #[test]
    fn verify_multiline_with_noise_hashes() {
        let program = "
            ### #
            dsgsegpsegoks
            # # ###
        ";
        let mut lexer = Lexer::new(program);

        let token = lexer.next_token();
        assert_eq!(
            token,
            Some(Token {
                token_type: tokens::TokenType::Eof,
                size: Size { start: 0, end: 0 }
            }),
            "Failed to process multiline comment containing irregular hash patterns"
        );
    }



}
