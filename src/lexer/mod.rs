pub mod tokens;
pub mod size;
use std::{f32::consts::E, str::CharIndices};
use std::iter::Peekable;

use size::Size;
use tokens::{Token, TokenType};
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

    fn double_char_token(
        &mut self,
        start: usize,
        expected_second: char,
        double_token: TokenType,
        single_token: TokenType
    ) -> Token {
        // consume current char
        self.next_char();
    
        match self.peek_char() {
            Some((_, ch)) if ch == expected_second => {
                self.next_char(); // consume second char
                Token {
                    token_type: double_token,
                    size: Size { start, end: start + 2 },
                }
            }
            _ => Token {
                token_type: single_token,
                size: Size { start, end: start + 1 },
            },
        }
    }

    fn read_ident(&mut self, start: usize) -> &str {
        let mut end = start;
    
        while let Some((idx, ch)) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.next_char();
                end = idx + ch.len_utf8(); // set end to the byte *after* current char
            } else {
                break;
            }
        }
    
        &self.program[start..end]
    }
    
    fn read_int_literal(&mut self, start: usize) -> (usize, &str) {
        let mut end = start;
    
        while let Some((idx, ch)) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.next_char();
                end = idx + ch.len_utf8(); // ensures correct byte range
            } else {
                break;
            }
        }
    
        (end, &self.program[start..end])
    }
    

    fn stirng_char_token(&mut self,start:usize,start_offset:usize,token_type:TokenType)->Token{
        Token{
            token_type:token_type,
            size:Size{start:start,end:start+start_offset}
        }
    }
    pub fn next_token(&mut self)->Option<Token>{

        use tokens::TokenType::*;
        use tokens::Literal::*;
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
            '('=> self.single_char_token(curr_offset, LParen),
            ')'=> self.single_char_token(curr_offset, RParen),
            ':'=> self.single_char_token(curr_offset, Colon),
            ','=> self.single_char_token(curr_offset, Comma),
            '^'=> self.single_char_token(curr_offset,BitXor),
            '/'=> self.single_char_token(curr_offset, Division),
            '*'=> self.single_char_token(curr_offset, Asterisk),
            '-'=> self.single_char_token(curr_offset, Minus),
            '+'=> self.single_char_token(curr_offset, Plus),

            '.'=> self.double_char_token(curr_offset,'.',Range,Dot),
            '='=> self.double_char_token(curr_offset, '=', EqualEqual, Assign),
            '>'=> self.double_char_token(curr_offset, '=', GreaterThanEqual, GreaterThan),
            '<'=> self.double_char_token(curr_offset, '=', LessThanEqual, LessThan),
            '!'=> self.double_char_token(curr_offset, '=', NotEqual,Not),
            '|'=> self.double_char_token(curr_offset, '|', Or,BitOr),
            '&'=> self.double_char_token(curr_offset, '&', And, BitAnd),
            'a'..='z'| 'A'..='Z'|'_'=>{
                let ident= self.read_ident(curr_offset);
                
                match ident{
                    "rune"=>self.stirng_char_token(curr_offset, 4, Bind),
                    "summon"=>self.stirng_char_token(curr_offset, 6, Summon),
                    "self"=>self.stirng_char_token(curr_offset, 4, SelfType),
                    "draws"=>self.stirng_char_token(curr_offset, 5, Draws),
                    "spell"=>self.stirng_char_token(curr_offset, 5, Spell),
                    "chant"=>self.stirng_char_token(curr_offset, 5, Chant),
                    "shatter"=>self.stirng_char_token(curr_offset, 7, Shatter),
                    "phase"=>self.stirng_char_token(curr_offset, 5, Phase),
                    "bind"=>self.stirng_char_token(curr_offset,4, Bind),
                    "seal"=>self.stirng_char_token(curr_offset, 4, Seal),
                    "reveal"=>self.stirng_char_token(curr_offset, 6, Reveal),
                    "veil"=>self.stirng_char_token(curr_offset, 4, Veil),
                    "divine"=>self.stirng_char_token(curr_offset, 6, Divine),
                    "sigil"=>self.stirng_char_token(curr_offset, 5, Sigil),
                    "default"=>self.stirng_char_token(curr_offset, 7, Default),
                    "invoke"=>self.stirng_char_token(curr_offset, 6, Invoke),
                    "linger"=>self.stirng_char_token(curr_offset, 6, Linger),
                    "in"=>self.stirng_char_token(curr_offset, 2, In),
                    "call"=>self.stirng_char_token(curr_offset, 4, Call),
                    "as"=>self.stirng_char_token(curr_offset, 2, As),
                    _=>self.stirng_char_token(curr_offset,ident.len(), Literal(Str(ident.to_string())))
                }
            },
            '0'..='9'=> {
                let (end, literal) = self.read_int_literal(curr_offset);
                Token {
                    token_type: Literal(Int(literal.parse().unwrap())),
                    size: Size { start: curr_offset, end },
                }
            } ,
            _ => return None,
        };
    
        Some(token)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    struct LexerTestCase<'a>{
        name:&'a str,
        input:&'a str,
        expected_tokn: Option<Token>
    }

    fn run_test_case(case:LexerTestCase){
        let mut lexer= Lexer::new(case.input);
        let token= lexer.next_token();
        assert_eq!(
            token,
            case.expected_tokn,
            "Test case `{}` failed: Expected {:?}, got {:?}",
            case.name,
            case.expected_tokn,
            token
        )
    }
    #[test]
    fn table_driven_lexer_tests(){
        use tokens::TokenType::*;

        let test_cases= vec![
            LexerTestCase{
                name: "whitespace and single-line comment",
                input:"

                    # this is a comment
                ",
                expected_tokn:Some(
                    Token { 
                        token_type:Eof ,
                        size: Size{start:0,end:0} }
                )
            },
            LexerTestCase{
                name: "irregular comments",
                input:"
                    ### #
                    The first rule of fight club is 
                    that you do not talk about fight club
                    # # ###
                ",
                expected_tokn:Some(
                    Token { token_type: Eof, size: Size { start: 0, end: 0 } }
                )
            },
            LexerTestCase{
                name:"Eof only",
                input:" ",
                expected_tokn:Some(
                    Token { token_type: Eof, size: Size{start:0,end:0} }
                )
            }
        ];
        for case in test_cases{
            run_test_case(case);
        }
    }
}
