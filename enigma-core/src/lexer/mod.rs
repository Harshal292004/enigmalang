pub mod size;
pub mod tokens;
use core::panic;
use std::iter::Peekable;
use std::str::CharIndices;

use tokens::{Token, TokenType,Literal};

// Use Peekable to avoid O(n) cost of chars().nth(0) and to peek without consuming.
// Rust strings are UTF-8; slicing with arbitrary indices can panic.
// Use char_indices() to get valid byte offsets for slicing.
#[derive(Debug)]
pub struct Lexer<'l> {
    // program has a lifetime tied to the lexer, ensuring it lives as long as the lexer safe to reference throughout.
    program: &'l str,
    chars: Peekable<CharIndices<'l>>,
}

impl<'l> Lexer<'l> {
    pub fn new(program: &'l str) -> Self {
        Self {
            program: program,
            chars: program.char_indices().peekable(),
        }
    }
    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }
    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
    fn skip_whitespace(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }
    fn skip_comment(&mut self) {
        while let Some((_, ch)) = self.peek() {
            if ch == '\n' {
                self.next();
                break;
            }
            self.next();
        }
    }
    fn read_string_literal(&mut self, start: usize) -> Token {
        let mut end = start;
        let mut value = String::new();
        self.next(); // Consume opening quote

        while let Some((idx, ch)) = self.next() {
            match ch {
                '\\' => {
                    if let Some((_, esc)) = self.next() {
                        end = idx + 1;
                        match esc {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            '"' => value.push('"'),
                            '\\' => value.push('\\'),
                            _ => {
                                value.push('\\');
                                value.push(esc);
                            }
                        }
                    } else {
                        break; // Unterminated escape sequence
                    }
                }
                '"' => {
                    end = idx + 1; // Include closing quote
                    break;
                }
                _ => {
                    value.push(ch);
                    end = idx + ch.len_utf8();
                }
            }
        }

        Token::new(
            start,
            end - start,
            TokenType::Literal(Literal::Str(value)),
        )
    }
    
    fn read_char_literal(&mut self, start: usize) -> Token {
        self.next(); // Consume opening quote
        let (mut value, mut end) = match self.next() {
            Some((idx, '\\')) => {
                if let Some((_, esc)) = self.next() {
                    let ch = match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\'' => '\'',
                        '\\' => '\\',
                        _ => esc,
                    };
                    (ch, idx + 1 + esc.len_utf8())
                } else {
                    panic!("Unterminated char literal");
                }
            }
            Some((idx, ch)) => (ch, idx + ch.len_utf8()),
            None => panic!("Unterminated char literal"),
        };

        if let Some((idx, '\'')) = self.next() {
            end = idx + 1;
        } else {
            panic!("Expected closing quote");
        }

        Token::new(
            start,
            end - start,
            TokenType::Literal(Literal::Char(value)),
        )
    }
    fn read_identifier(&mut self, start:usize)-> &str{
        let mut end = start;
        while let Some((idx,ch))=self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.next();
                end= idx+ch.len_utf8();
            }else{
                break;
            }
        }

        &self.program[start..end]
    }
    fn double_char_token(
        &mut self,
        start: usize,
        expected_second: char,
        double_token: TokenType,
        single_token: TokenType,
    ) -> Token {
        // consume current char
        self.next();

        match self.peek() {
            Some((_, ch)) if ch == expected_second => {
                self.next(); // consume second char
                Token::new(start, 2, double_token)
            }
            _ => Token::new(start, 1, single_token),
        }
    }

    fn  build_operator(
        &mut self,
        start:usize,
        curr_char:char,
        double_token:TokenType,
        equal_token:TokenType,
        single_token:TokenType
    )-> Token{
        self.next();
        match self.peek(){
            Some((_,c)) if c==curr_char =>{
                self.next();
                Token::new(start,2,double_token)
            },
            Some((_,'='))=>{
                self.next();
                Token::new(start,2,equal_token)
            },
            None =>{
                self.next();
                Token::new(start,2,single_token)    
            }         
        }
    }

    fn read_string_literal(&mut self, start: usize) -> &str {
        let mut end = start;
        // consume "
        self.next();

        while let Some((idx, ch)) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch== '\\' || ch.is_whitespace() {
                self.next();
                end = idx + ch.len_utf8();
            } else {
                // consume "
                self.next();
                break;
            }
        }
        &self.program[start..end]
    }

    fn read_identifier(&mut self , start:usize) -> &str{
        let mut end = start;

        while let Some((idx, ch)) = self.peek() {
            if ch.is_alphanumeric() || ch == '_'  {
                self.next();
                end = idx + ch.len_utf8();
            } else {
                break;
            }
        }
        &self.program[start..end]
    }
    fn read_int_literal(&mut self, start: usize) -> (usize,bool, &str) {
        let mut end = start;
        let mut is_float:bool = false; 
        while let Some((idx, ch)) = self.peek() {
            if ch.is_ascii_digit() || ch == '.'{
                if ch == '.' {
                    is_float=true;
                }
                self.next();
                end = idx + ch.len_utf8();
            } else {
                break;
            }
        }

        (end, is_float,&self.program[start..end])
    }

    fn stirng_char_token(
        &mut self,
        start: usize,
        start_offset: usize,
        token_type: TokenType,
    ) -> Token {
        Token::new(start, start_offset, token_type)
    }
    pub fn next_token(&mut self) -> Token {
        use tokens::Literal::*;
        use tokens::TokenType::*;
        // skip the white spaces in the code
        self.skip_whitespace();

        let (curr_offset, curr_char) = match self.peek() {
            Some(c) => c,
            None => return Token::new(0, 0, Eof),
        };

        let token = match curr_char {
            '#' => {
                self.skip_comment();
                // Helps skip comments and recursively advance to the next token for clean parsing
                return self.next_token();
            }
            // single char tokens
            '@' => self.single_char_token(curr_offset, Func),
            '{' => self.single_char_token(curr_offset, LCurly),
            '}' => self.single_char_token(curr_offset, RCurly),
            '(' => self.single_char_token(curr_offset, LParen),
            ')' => self.single_char_token(curr_offset, RParen),
            '[' => self.single_char_token(curr_offset, LSquare),
            ']' => self.single_char_token(curr_offset, RSquare),
            ',' => self.single_char_token(curr_offset, Comma),
            '^' => self.single_char_token(curr_offset, Carrot),
            '/' => self.single_char_token(curr_offset, Division),
            '*' => self.double_char_token(curr_offset,'=',AsteriskEqual,Asterisk),
            '-' => self.build_operator(curr_offset, curr_char, MinusMinus, MinusEqual, Minus),
            '+' => self.build_operator(curr_offset, curr_char, PlusPlus, PlusEqual, Plus),
            ':' => self.build_operator(curr_offset, curr_char,DoubleColon, Assign, Colon),
            '.' => self.double_char_token(curr_offset, '.', DotDot, Dot),
            '=' => self.double_char_token(curr_offset, '=', EqualEqual, Assign),
            '>' => self.double_char_token(curr_offset, '=', GreaterThanEqual, GreaterThan),
            '<' => self.double_char_token(curr_offset, '=', LessThanEqual, LessThan),
            '!' => self.double_char_token(curr_offset, '=', ExclaimEqual,Exclaim),
            '|' => self.double_char_token(curr_offset, '|', PipePipe, Pipe),
            '&' => self.double_char_token(curr_offset, '&', AmpersandAmpersand, Ampersand),
            '$' => self.double_char_token(curr_offset, '=', Destructure, Dollar),
            '"'=> {
                let ident = self.read_string_literal(curr_offset);

                match ident {
                    _ => Token::new(curr_offset, ident.len(), Literal(Str(ident.to_string()))),
                }
            }
            '\''=>{
                // consume '
                self.next();
                
                match self.peek(){
                    Some((_,'\\'))=>{
                        // consume \
                        self.next();
                        // consume char
                        let i=self.next().unwrap();
                        // consume '
                        self.next();
                        Token::new(curr_offset, 4, Literal(Char(('{i.1}'))))
                        
                    }
                    _ => {
                        // consume char
                        let i=self.next().unwrap();
                        // consume '
                        self.next();
                     
                        Token::new(curr_offset, 3,Literal(Char('{}'))),
                    }
                }     
            }
            'a'..='z' | 'A'..='Z' | '_' => {

                match self.peek() {
                    Some((_,ch)) if ch.is_whitespace()=>{
                        Token::new(curr_offset, 1, UnderScore)
                    },
                    _=>{
                        let ident = self.read_identifier(curr_offset);

                        match ident {
                            "get" => Token::new(curr_offset, 4, Get),
                            "module" => Token::new(curr_offset, 6, Module),
                            "as" => Token::new(curr_offset, 4, As),
                            "mut" => Token::new(curr_offset, 5, Mut),
                            "return" => Token::new(curr_offset, 5, Return),
                            "if" => Token::new(curr_offset, 5, If),
                            "else" => Token::new(curr_offset, 7, Else),
                            "for" => Token::new(curr_offset, 5, For),
                            "in" => Token::new(curr_offset, 4, In),
                            "loop" => Token::new(curr_offset, 4, Loop),
                            "while" => Token::new(curr_offset, 6, While),
                            "match" => Token::new(curr_offset, 4, Match),
                            "case" => Token::new(curr_offset, 6, Case),
                            "pub" => Token::new(curr_offset, 5, Pub),
                            "implement" => Token::new(curr_offset, 7, Impl),
                            "record" => Token::new(curr_offset, 6, Record),
                            "union" => Token::new(curr_offset, 6, Union),
                            "ref" => Token::new(curr_offset, 2, In),
                            "deref" => Token::new(curr_offset, 4, Deref),
                            "raw_ref" => Token::new(curr_offset, 2, RawRef),
                            "unsafee" => Token::new(curr_offset, 2, Unsafe),
                            "protoc" => Token::new(curr_offset, 2, Protoc),
                            "asm" => Token::new(curr_offset, 2,Asm),
                            "continue" => Token::new(curr_offset, 2, Continue),
                            "break"=>Token::new(curr_offset, 2, Break),
                            "true"=> Token::new(curr_offset,4,Literal(Bool(true))),
                            "false"=> Token::new(curr_offset,4,Literal(Bool(false))), 
                            _ => panic!("No identifier found")                       }

                    
                    } 
                }
            },
            '0'..='9' => {
                let (end,is_float, literal) = self.read_int_literal(curr_offset);
                if is_float{
                    Token::new(
                        curr_offset,
                        end - curr_offset,
                        Literal(Float(literal.parse().unwrap())),
                    )
                }else{
                    Token::new(
                        curr_offset,
                        end - curr_offset,
                        Literal(Int(literal.parse().unwrap())),
                    )    
                }
            }
            _ => panic!(""),
        };

        token
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.token_type == TokenType::Eof {
            None
        } else {
            Some(token)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use super::*;

//     struct LexerTestCase<'a> {
//         name: &'a str,
//         input: &'a str,
//         expected_token: Token,
//     }

//     struct LexerMultiTokenCase<'m> {
//         name: &'m str,
//         input: &'m str,
//         expected_tokens: Vec<Token>,
//     }

//     fn run_test_case(case: LexerTestCase) {
//         let mut lexer = Lexer::new(case.input);
//         let token = lexer.next_token();
//         assert_eq!(
//             token, case.expected_token,
//             "Test case {} failed: Expected {:?}, got {:?}",
//             case.name, case.expected_token, token
//         )
//     }

//     fn run_multiple_token_test(case: LexerMultiTokenCase) {
//         let mut lexer = Lexer::new(case.input);
//         for (i, expected) in case.expected_tokens.iter().enumerate() {
//             let token = lexer.next();
//             match token {
//                 Some(t) => assert_eq!(&t, expected, "Test {} failed at token {}", case.name, i),
//                 None => {
//                     assert!(
//                         false,
//                         "Test {} failed at token {}: expected {:?}, got None",
//                         case.name, i, expected
//                     );
//                     panic!(
//                         "Test {} failed at token {}: expected {:?}, got None",
//                         case.name, i, expected
//                     );
//                 }
//             }
//         }
//         // after exhausting expected tokens, lexer should be at EOF
//         assert!(
//             lexer.next().is_none(),
//             "Test {} failed: Lexer has extra tokens",
//             case.name
//         );
//     }

//     #[test]
//     fn test_whitespace_and_comments() {
//         use tokens::TokenType::*;

//         let test_cases = vec![
//             LexerTestCase {
//                 name: "whitespace and single-line comment",
//                 input: "

//                     # this is a comment
//                 ",
//                 expected_token: Token::new(0, 0, Eof),
//             },
//             LexerTestCase {
//                 name: "irregular comments",
//                 input: "
//                     ### #
//                     The first rule of fight club is 
//                     that you do not talk about fight club
//                     # # ###
//                 ",
//                 expected_token: Token::new(0, 0, Eof),
//             },
//             LexerTestCase {
//                 name: "Eof only",
//                 input: " ",
//                 expected_token: Token::new(0, 0, Eof),
//             },
//         ];
//         for case in test_cases {
//             run_test_case(case);
//         }
//     }

//     #[test]
//     fn test_single_character_token() {
//         use tokens::TokenType::*;

//         let test_cases = vec![
//             LexerTestCase {
//                 name: "left curly brace",
//                 input: "{",
//                 expected_token: Token::new(0, 1, LCurly),
//             },
//             LexerTestCase {
//                 name: "right curly brace",
//                 input: "}",
//                 expected_token: Token::new(0, 1, RCurly),
//             },
//             LexerTestCase {
//                 name: "left parenthesis",
//                 input: "(",
//                 expected_token: Token::new(0, 1, LParen),
//             },
//             LexerTestCase {
//                 name: "right parenthesis",
//                 input: ")",
//                 expected_token: Token::new(0, 1, RParen),
//             },
//             LexerTestCase {
//                 name: "colon",
//                 input: ":",
//                 expected_token: Token::new(0, 1, Colon),
//             },
//             LexerTestCase {
//                 name: "comma",
//                 input: ",",
//                 expected_token: Token::new(0, 1, Comma),
//             },
//             LexerTestCase {
//                 name: "bit xor",
//                 input: "^",
//                 expected_token: Token::new(0, 1, BitXor),
//             },
//             LexerTestCase {
//                 name: "division",
//                 input: "/",
//                 expected_token: Token::new(0, 1, Division),
//             },
//             LexerTestCase {
//                 name: "asterisk",
//                 input: "*",
//                 expected_token: Token::new(0, 1, Asterisk),
//             },
//             LexerTestCase {
//                 name: "minus",
//                 input: "-",
//                 expected_token: Token::new(0, 1, Minus),
//             },
//             LexerTestCase {
//                 name: "plus",
//                 input: "+",
//                 expected_token: Token::new(0, 1, Plus),
//             },
//         ];

//         for case in test_cases {
//             run_test_case(case);
//         }
//     }

//     #[test]
//     fn test_multiple_character_token() {
//         use tokens::TokenType::*;
//         let test_cases = vec![
//             LexerTestCase {
//                 name: "range token ..",
//                 input: "..",
//                 expected_token: Token::new(0, 2, Range),
//             },
//             LexerTestCase {
//                 name: "dot token .",
//                 input: ".",
//                 expected_token: Token::new(0, 1, Dot),
//             },
//             LexerTestCase {
//                 name: "equal equal token ==",
//                 input: "==",
//                 expected_token: Token::new(0, 2, EqualEqual),
//             },
//             LexerTestCase {
//                 name: "assign token =",
//                 input: "=",
//                 expected_token: Token::new(0, 1, Assign),
//             },
//             LexerTestCase {
//                 name: "greater than equal token >=",
//                 input: ">=",
//                 expected_token: Token::new(0, 2, GreaterThanEqual),
//             },
//             LexerTestCase {
//                 name: "greater than token >",
//                 input: ">",
//                 expected_token: Token::new(0, 1, GreaterThan),
//             },
//             LexerTestCase {
//                 name: "less than equal token <=",
//                 input: "<=",
//                 expected_token: Token::new(0, 2, LessThanEqual),
//             },
//             LexerTestCase {
//                 name: "less than token <",
//                 input: "<",
//                 expected_token: Token::new(0, 1, LessThan),
//             },
//             LexerTestCase {
//                 name: "not equal token !=",
//                 input: "!=",
//                 expected_token: Token::new(0, 2, NotEqual),
//             },
//             LexerTestCase {
//                 name: "not token !",
//                 input: "!",
//                 expected_token: Token::new(0, 1, Not),
//             },
//             LexerTestCase {
//                 name: "or token ||",
//                 input: "||",
//                 expected_token: Token::new(0, 2, Or),
//             },
//             LexerTestCase {
//                 name: "bit or token |",
//                 input: "|",
//                 expected_token: Token::new(0, 1, BitOr),
//             },
//             LexerTestCase {
//                 name: "and token &&",
//                 input: "&&",
//                 expected_token: Token::new(0, 2, And),
//             },
//             LexerTestCase {
//                 name: "bit and token &",
//                 input: "&",
//                 expected_token: Token::new(0, 1, BitAnd),
//             },
//         ];
//         for case in test_cases {
//             run_test_case(case);
//         }
//     }
//     #[test]
//     fn test_stream_of_tokens() {
//         use tokens::Literal::*;
//         use tokens::TokenType::*;
//         let test_cases = vec![
//             // Single keyword and identifier with punctuation
//             LexerMultiTokenCase {
//                 name: "Simple rune declaration",
//                 input: "rune Account {",
//                 expected_tokens: vec![
//                     Token::new(0, 4, Bind),
//                     Token::new(5, 7, Literal(Str("Account".into()))),
//                     Token::new(13, 1, LCurly),
//                 ],
//             },
//             // Struct fields with type annotations and keywords
//             LexerMultiTokenCase {
//                 name: "Bindings inside rune",
//                 input: "bind owner: str\nbind balance: float",
//                 expected_tokens: vec![
//                     Token::new(0, 4, Bind),
//                     Token::new(5, 5, Literal(Str("owner".into()))),
//                     Token::new(10, 1, Colon),
//                     Token::new(12, 3, Literal(Str("str".into()))),
//                     Token::new(16, 4, Bind),
//                     Token::new(21, 7, Literal(Str("balance".into()))),
//                     Token::new(28, 1, Colon),
//                     Token::new(30, 5, Literal(Str("float".into()))),
//                 ],
//             },
//             // Function declaration and parameters
//             LexerMultiTokenCase {
//                 name: "Spell with parameters",
//                 input: "spell summon(self, owner: str, balance: float) {",
//                 expected_tokens: vec![
//                     Token::new(0, 5, Spell),
//                     Token::new(6, 6, Literal(Str("summon".into()))),
//                     Token::new(12, 1, LParen),
//                     Token::new(13, 4, SelfType),
//                     Token::new(17, 1, Comma),
//                     Token::new(19, 5, Literal(Str("owner".into()))),
//                     Token::new(24, 1, Colon),
//                     Token::new(26, 3, Literal(Str("str".into()))),
//                     Token::new(29, 1, Comma),
//                     Token::new(31, 7, Literal(Str("balance".into()))),
//                     Token::new(38, 1, Colon),
//                     Token::new(40, 5, Literal(Str("float".into()))),
//                     Token::new(45, 1, RParen),
//                     Token::new(47, 1, LCurly),
//                 ],
//             },
//             // Conditional with reveal and veil blocks
//             LexerMultiTokenCase {
//                 name: "Reveal and veil blocks",
//                 input: "reveal amount > 0 {\nchant \"Deposit successful\"\n} veil {\nchant \"Invalid deposit amount\"\n}",
//                 expected_tokens: vec![
//                     Token::new(0, 6, Reveal),
//                     Token::new(7, 6, Literal(Str("amount".into()))),
//                     Token::new(14, 1, GreaterThan),
//                     Token::new(16, 1, Literal(Str("0".into()))), // Actually this should be Int(0), fix needed in lexer test case if required
//                     Token::new(18, 1, LCurly),
//                     Token::new(20, 5, Chant),
//                     // String tokens missing in lexer, but here we treat as literal string (needs lexer support)
//                     // For test simplicity, skipping actual string tokens (implement string lexing if needed)
//                     // Token::new(26, ..., Literal(Str("Deposit successful".into()))),
//                     Token::new(44, 1, RCurly),
//                     Token::new(46, 4, Veil),
//                     Token::new(51, 1, LCurly),
//                     Token::new(53, 5, Chant),
//                     // Token::new(59, ..., Literal(Str("Invalid deposit amount".into()))),
//                     Token::new(81, 1, RCurly),
//                 ],
//             },
//             // Range token and invocation loop
//             LexerMultiTokenCase {
//                 name: "Invoke loop and range",
//                 input: "invoke i in 1..3 {",
//                 expected_tokens: vec![
//                     Token::new(0, 6, Invoke),
//                     Token::new(7, 1, Literal(Str("i".into()))),
//                     Token::new(9, 2, In),
//                     Token::new(12, 1, Literal(Int(1))),
//                     Token::new(13, 2, Range),
//                     Token::new(15, 1, Literal(Int(3))),
//                     Token::new(16, 1, LCurly),
//                 ],
//             },
//             // Multiple sigil branches inside divine
//             LexerMultiTokenCase {
//                 name: "Divine with sigil cases",
//                 input: r#"divine action {
//                     sigil "1" { io.echoln("Enter amount to deposit:") }
//                     sigil "2" { io.echoln("Enter amount to withdraw:") }
//                     sigil default { io.echoln("Invalid action") }
//                 }"#,
//                 expected_tokens: vec![
//                     Token::new(0, 6, Divine),
//                     Token::new(7, 6, Literal(Str("action".into()))),
//                     Token::new(14, 1, LCurly),
//                     Token::new(24, 5, Sigil),
//                     Token::new(30, 3, Literal(Str("1".into()))),
//                     Token::new(34, 1, LCurly),
//                     Token::new(36, 2, Literal(Str("io".into()))),
//                     Token::new(38, 1, Dot),
//                     Token::new(39, 7, Literal(Str("echoln".into()))),
//                     Token::new(46, 1, LParen),
//                     // Ignoring inner string literals for now (requires string lexing)
//                     Token::new(68, 1, RParen),
//                     Token::new(69, 1, RCurly),
//                     Token::new(79, 5, Sigil),
//                     Token::new(85, 3, Literal(Str("2".into()))),
//                     Token::new(89, 1, LCurly),
//                     Token::new(91, 2, Literal(Str("io".into()))),
//                     Token::new(93, 1, Dot),
//                     Token::new(94, 7, Literal(Str("echoln".into()))),
//                     Token::new(101, 1, LParen),
//                     Token::new(123, 1, RParen),
//                     Token::new(124, 1, RCurly),
//                     Token::new(134, 5, Sigil),
//                     Token::new(140, 7, Default),
//                     Token::new(148, 1, LCurly),
//                     Token::new(150, 2, Literal(Str("io".into()))),
//                     Token::new(152, 1, Dot),
//                     Token::new(153, 7, Literal(Str("echoln".into()))),
//                     Token::new(160, 1, LParen),
//                     Token::new(176, 1, RParen),
//                     Token::new(177, 1, RCurly),
//                     Token::new(186, 1, RCurly),
//                 ],
//             },
//         ];
//         // Loop to run all these multi-token tests using your existing test runner
//         for case in test_cases {
//             run_multiple_token_test(case);
//         }
//     }
// }
