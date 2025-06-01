pub mod size;
pub mod tokens;
use std::iter::Peekable;
use std::str::CharIndices;

use tokens::{Literal, Token, TokenType};

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
    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
    fn skip_whitespace(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn skip_comment(&mut self) {
        while let Some((_, ch)) = self.peek() {
            if ch == '\n' {
                self.advance();
                break;
            }
            self.advance();
        }
    }
    fn read_string_literal(&mut self, start: usize) -> Token {
        let mut end = start;
        let mut value = String::new();
        self.advance(); // Consume opening quote

        while let Some((idx, ch)) = self.advance() {
            match ch {
                '\\' => {
                    if let Some((_, esc)) = self.advance() {
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
                    end = idx + 1;
                    break;
                }
                _ => {
                    value.push(ch);
                    end = idx + ch.len_utf8();
                }
            }
        }

        Token::new(start, end - start, TokenType::Literal(Literal::Str(value)))
    }

    fn read_char_literal(&mut self, start: usize) -> Token {
        self.advance(); // Consume opening quote
        let (value, mut end) = match self.advance() {
            Some((idx, '\\')) => {
                if let Some((_, esc)) = self.advance() {
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

        if let Some((idx, '\'')) = self.advance() {
            end = idx + 1;
        } else {
            panic!("Expected closing quote");
        }

        Token::new(start, end - start, TokenType::Literal(Literal::Char(value)))
    }
    fn read_identifier(&mut self, start: usize) -> &str {
        let mut end = start;
        while let Some((idx, ch)) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
                end = idx + ch.len_utf8();
            } else {
                break;
            }
        }
        &self.program[start..end]
    }
    fn read_number_literal(&mut self, start: usize) -> Token {
        let mut end = start;
        let mut is_float = false;
        let mut has_exponent = false;
        let mut valid = true;

        while let Some((idx, ch)) = self.peek() {
            match ch {
                '0'..='9' => {
                    self.advance();
                    end = idx + 1;
                }
                '.' => {
                    if is_float || has_exponent {
                        break; // For range ..
                    }

                    let mut temp_chars = self.chars.clone();
                    temp_chars.next();
                    if let Some((_, next_ch)) = temp_chars.peek() {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            self.advance();
                            end = idx + 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                'e' | 'E' => {
                    if has_exponent {
                        valid = false;
                    }
                    has_exponent = true;
                    self.advance();
                    end = idx + 1;

                    if let Some((_, sign)) = self.peek() {
                        if sign == '+' || sign == '-' {
                            self.advance();
                            end += 1;
                        }
                    }
                }
                _ => break,
            }
        }

        let literal_str = &self.program[start..end];
        if !valid {
            panic!("Invalid number format: {}", literal_str);
        }

        if is_float || has_exponent {
            let value = literal_str.parse().unwrap();
            Token::new(
                start,
                end - start,
                TokenType::Literal(Literal::Float(value)),
            )
        } else {
            let value = literal_str.parse().unwrap();
            Token::new(start, end - start, TokenType::Literal(Literal::Int(value)))
        }
    }

    fn consume_single(&mut self, token_type: TokenType) -> Token {
        let (start, _) = self.advance().unwrap();
        Token::new(start, 1, token_type)
    }

    fn consume_double(
        &mut self,
        second_char: char,
        double_type: TokenType,
        default_type: TokenType,
    ) -> Token {
        let (start, _) = self.advance().unwrap();
        match self.peek() {
            Some((_, c)) if c == second_char => {
                self.advance();
                Token::new(start, 2, double_type)
            }
            _ => Token::new(start, 1, default_type),
        }
    }

    fn consume_double_with_panic(&mut self, second_char: char, default_type: TokenType) -> Token {
        let (start, _) = self.advance().unwrap();
        match self.peek() {
            Some((_, c)) if c == second_char => {
                self.advance();
                Token::new(start, 2, default_type)
            }
            _ => panic!("Undetermined token"),
        }
    }

    fn consume_triple(
        &mut self,
        second_char: char,
        double_type: TokenType,
        third_char: char,
        triple_type: TokenType,
        default_type: TokenType,
    ) -> Token {
        let (start, _) = self.advance().unwrap();
        match self.peek() {
            Some((_, c)) if c == second_char => {
                self.advance();
                Token::new(start, 2, double_type)
            }
            Some((_, c)) if c == third_char => {
                self.advance();
                Token::new(start, 2, triple_type)
            }
            _ => Token::new(start, 1, default_type),
        }
    }

    fn consume_quad(
        &mut self,
        second_char: char,
        double_type: TokenType,
        third_char: char,
        triple_type: TokenType,
        fourth_char: char,
        fourth_type: TokenType,
        default_type: TokenType,
    ) -> Token {
        let (start, _) = self.advance().unwrap();
        match self.peek() {
            Some((_, c)) if c == second_char => {
                self.advance();
                Token::new(start, 2, double_type)
            }
            Some((_, c)) if c == third_char => {
                self.advance();
                Token::new(start, 2, triple_type)
            }
            Some((_, c)) if c == fourth_char => {
                self.advance();
                Token::new(start, 2, fourth_type)
            }
            _ => Token::new(start, 1, default_type),
        }
    }

    fn handle_identifier(&mut self, start: usize) -> Token {
        use tokens::{Literal, TokenType::*};
        let ident = self.read_identifier(start);
        let token_type = match ident {
            "get" => Get,
            "module" => Module,
            "as" => As,
            "mut" => Mut,
            "return" => Return,
            "if" => If,
            "else" => Else,
            "for" => For,
            "in" => In,
            "loop" => Loop,
            "while" => While,
            "match" => Match,
            "case" => Case,
            "pub" => Pub,
            "implement" => Impl,
            "record" => Record,
            "union" => Union,
            "ref" => Ref,
            "deref" => Deref,
            "raw_ref" => RawRef,
            "unsafe" => Unsafe,
            "protoc" => Protoc,
            "asm" => Asm,
            "continue" => Continue,
            "break" => Break,
            "true" => TokenType::Literal(Literal::Bool(true)),
            "false" => TokenType::Literal(Literal::Bool(false)),
            _ => Identifier,
        };
        Token::new(start, ident.len(), token_type)
    }

    pub fn advance_token(&mut self) -> Token {
        use TokenType::*;
        // skip the white spaces in the code
        self.skip_whitespace();

        let (start, ch) = match self.peek() {
            Some((pos, c)) => (pos, c),
            None => return Token::new(0, 0, Eof),
        };

        match ch {
            '#' => {
                self.skip_comment();
                return self.advance_token();
            }
            '@' => self.consume_single(Func),
            '{' => self.consume_single(LCurly),
            '}' => self.consume_single(RCurly),
            '(' => self.consume_single(LParen),
            ')' => self.consume_single(RParen),
            '[' => self.consume_single(LSquare),
            ']' => self.consume_single(RSquare),
            ',' => self.consume_single(Comma),
            '^' => self.consume_single(Carrot),
            '%' => self.consume_single(Percent),
            '?' => self.consume_single(Question),
            ';' => self.consume_single(ReturnSemi),
            '"' => self.read_string_literal(start),
            '\'' => self.read_char_literal(start),
            ':' => self.consume_triple(':', DoubleColon, '=', Assign, Colon),
            '+' => self.consume_triple('+', PlusPlus, '=', PlusEqual, Plus),
            '-' => self.consume_quad('-', MinusMinus, '=', MinusEqual, '>', Arrow, Minus),
            '*' => self.consume_double('=', AsteriskEqual, Asterisk),
            '&' => self.consume_double('&', AmpersandAmpersand, Ampersand),
            '|' => self.consume_double('|', PipePipe, Pipe),
            '/' => self.consume_double('=', SlashEqual, Slash),
            '!' => self.consume_double('=', ExclaimEqual, Exclaim),
            '.' => self.consume_double('.', DotDot, Dot),
            '<' => self.consume_double('=', LessThanEqual, LessThan),
            '>' => self.consume_double('=', GreaterThanEqual, GreaterThan),
            '$' => self.consume_double_with_panic('=', Destructure),
            '=' => self.consume_double_with_panic('=', EqualEqual),
            '0'..='9' => self.read_number_literal(start),
            // No identifier starts with _ for good sake
            '_' => self.consume_single(UnderScore),
            'a'..='z' | 'A'..='Z' => self.handle_identifier(start),
            _ => panic!("Unexpected character: '{}' at position {}", ch, start),
        }
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.advance_token();
        match token.token_type {
            TokenType::Eof => None,
            _ => Some(token),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    struct LexerTestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected_token: Token,
    }

    struct LexerMultiTokenCase<'m> {
        name: &'m str,
        input: &'m str,
        expected_tokens: Vec<Token>,
    }

    fn run_test_case(case: LexerTestCase) {
        let mut lexer = Lexer::new(case.input);
        let token = lexer.advance_token();
        assert_eq!(
            token, case.expected_token,
            "Test case {} failed: Expected {:?}, got {:?}",
            case.name, case.expected_token, token
        )
    }

    fn run_multiple_token_test(case: LexerMultiTokenCase) {
        let mut lexer = Lexer::new(case.input);
        for (i, expected) in case.expected_tokens.iter().enumerate() {
            let token = lexer.next();
            match token {
                Some(t) => assert_eq!(
                    &t, expected,
                    "Test {} failed at token {}: expected {:?}, got {:?}",
                    case.name, i, expected, t
                ),
                None => {
                    panic!(
                        "Test {} failed at token {}: expected {:?}, got None",
                        case.name, i, expected
                    );
                }
            }
        }
        let next_token = lexer.next();
        assert!(
            next_token.is_none(),
            "Test {} failed: Lexer has extra tokens: {:?}",
            case.name,
            next_token
        );
    }

    #[test]
    fn test_whitespace_and_comments() {
        use tokens::TokenType::*;

        let test_cases = vec![
            LexerTestCase {
                name: "whitespace and single-line comment",
                input: "

                    # this is a comment
                ",
                expected_token: Token::new(0, 0, Eof),
            },
            LexerTestCase {
                name: "Eof only",
                input: " ",
                expected_token: Token::new(0, 0, Eof),
            },
        ];
        for case in test_cases {
            run_test_case(case);
        }
    }

    #[test]
    fn test_single_character_token() {
        use tokens::TokenType::*;

        let test_cases = vec![
            LexerTestCase {
                name: "left curly brace",
                input: "{",
                expected_token: Token::new(0, 1, LCurly),
            },
            LexerTestCase {
                name: "right curly brace",
                input: "}",
                expected_token: Token::new(0, 1, RCurly),
            },
            LexerTestCase {
                name: "left parenthesis",
                input: "(",
                expected_token: Token::new(0, 1, LParen),
            },
        ];

        for case in test_cases {
            run_test_case(case);
        }
    }

    #[test]
    fn test_multiple_character_token() {
        use tokens::TokenType::*;
        let test_cases = vec![
            LexerTestCase {
                name: "Double dot",
                input: "..",
                expected_token: Token::new(0, 2, DotDot),
            },
            LexerTestCase {
                name: "dot token .",
                input: ".",
                expected_token: Token::new(0, 1, Dot),
            },
            LexerTestCase {
                name: "dollar equal",
                input: "$=",
                expected_token: Token::new(0, 2, Destructure),
            },
        ];
        for case in test_cases {
            run_test_case(case);
        }
    }

    #[test]
    fn test_stream_of_tokens() {
        use tokens::Literal::*;
        use tokens::TokenType::*;

        let test_cases = vec![
            // Simple function declaration
            LexerMultiTokenCase {
                name: "Simple function declaration",
                input: "@sum(int a, int b)",
                expected_tokens: vec![
                    Token::new(0, 1, Func),        // @
                    Token::new(1, 3, Identifier),  // sum
                    Token::new(4, 1, LParen),      // (
                    Token::new(5, 3, Identifier),  // int
                    Token::new(9, 1, Identifier),  // a
                    Token::new(10, 1, Comma),      // ,
                    Token::new(12, 3, Identifier), // int
                    Token::new(16, 1, Identifier), // b
                    Token::new(17, 1, RParen),     // )
                ],
            },
            // Variable declaration with assignment
            LexerMultiTokenCase {
                name: "Variable declaration",
                input: "mut int x := 42",
                expected_tokens: vec![
                    Token::new(0, 3, Mut),               // mut
                    Token::new(4, 3, Identifier),        // int
                    Token::new(8, 1, Identifier),        // x
                    Token::new(10, 2, Assign),           // :=
                    Token::new(13, 2, Literal(Int(42))), // 42
                ],
            },
            // String literal
            LexerMultiTokenCase {
                name: "String literal",
                input: "\"hello world\"",
                expected_tokens: vec![Token::new(0, 13, Literal(Str("hello world".into())))],
            },
            // If-else statement
            LexerMultiTokenCase {
                name: "If-else statement",
                input: "if x == 5 { return true } else { return false }",
                expected_tokens: vec![
                    Token::new(0, 2, If),                    // if
                    Token::new(3, 1, Identifier),            // x
                    Token::new(5, 2, EqualEqual),            // ==
                    Token::new(8, 1, Literal(Int(5))),       // 5
                    Token::new(10, 1, LCurly),               // {
                    Token::new(12, 6, Return),               // return
                    Token::new(19, 4, Literal(Bool(true))),  // true
                    Token::new(24, 1, RCurly),               // }
                    Token::new(26, 4, Else),                 // else
                    Token::new(31, 1, LCurly),               // {
                    Token::new(33, 6, Return),               // return
                    Token::new(40, 5, Literal(Bool(false))), // false
                    Token::new(46, 1, RCurly),               // }
                ],
            },
            // Record definition
            LexerMultiTokenCase {
                name: "Record definition",
                input: "record human { name: string, age: int }",
                expected_tokens: vec![
                    Token::new(0, 6, Record),      // record
                    Token::new(7, 5, Identifier),  // human
                    Token::new(13, 1, LCurly),     // {
                    Token::new(15, 4, Identifier), // name
                    Token::new(19, 1, Colon),      // :
                    Token::new(21, 6, Identifier), // string
                    Token::new(27, 1, Comma),      // ,
                    Token::new(29, 3, Identifier), // age
                    Token::new(32, 1, Colon),      // :
                    Token::new(34, 3, Identifier), // int
                    Token::new(38, 1, RCurly),     // }
                ],
            },
            // For loop
            LexerMultiTokenCase {
                name: "For loop",
                input: "for i in 1..10 { }",
                expected_tokens: vec![
                    Token::new(0, 3, For),               // for
                    Token::new(4, 1, Identifier),        // i
                    Token::new(6, 2, In),                // in
                    Token::new(9, 1, Literal(Int(1))),   // 1
                    Token::new(10, 2, DotDot),           // ..
                    Token::new(12, 2, Literal(Int(10))), // 10
                    Token::new(15, 1, LCurly),           // {
                    Token::new(17, 1, RCurly),           // }
                ],
            },
            LexerMultiTokenCase {
                name: "One line",
                input: "@sum(int a, int b)::int -> a + b;",
                expected_tokens: vec![
                    Token::new(0, 1, Func),         // @
                    Token::new(1, 3, Identifier),   // sum
                    Token::new(4, 1, LParen),       // (
                    Token::new(5, 3, Identifier),   // int
                    Token::new(9, 1, Identifier),   // a
                    Token::new(10, 1, Comma),       // ,
                    Token::new(12, 3, Identifier),  // int
                    Token::new(16, 1, Identifier),  // b
                    Token::new(17, 1, RParen),      // )
                    Token::new(18, 2, DoubleColon), // ::
                    Token::new(20, 3, Identifier),  // int
                    Token::new(24, 2, Arrow),       // ->
                    Token::new(27, 1, Identifier),  // a
                    Token::new(29, 1, Plus),        // +
                    Token::new(31, 1, Identifier),  // b
                    Token::new(32, 1, ReturnSemi),  // ;
                ],
            },
        ];

        for case in test_cases {
            run_multiple_token_test(case);
        }
    }

    #[test]
    fn test_operators() {
        use tokens::TokenType::*;

        let test_cases = vec![
            LexerMultiTokenCase {
                name: "Arithmetic operators",
                input: "a + b - c * d / e",
                expected_tokens: vec![
                    Token::new(0, 1, Identifier),  // a
                    Token::new(2, 1, Plus),        // +
                    Token::new(4, 1, Identifier),  // b
                    Token::new(6, 1, Minus),       // -
                    Token::new(8, 1, Identifier),  // c
                    Token::new(10, 1, Asterisk),   // *
                    Token::new(12, 1, Identifier), // d
                    Token::new(14, 1, Slash),      // /
                    Token::new(16, 1, Identifier), // e
                ],
            },
            LexerMultiTokenCase {
                name: "Comparison operators",
                input: "a <= b >= c != d",
                expected_tokens: vec![
                    Token::new(0, 1, Identifier),       // a
                    Token::new(2, 2, LessThanEqual),    // <=
                    Token::new(5, 1, Identifier),       // b
                    Token::new(7, 2, GreaterThanEqual), // >=
                    Token::new(10, 1, Identifier),      // c
                    Token::new(12, 2, ExclaimEqual),    // !=
                    Token::new(15, 1, Identifier),      // d
                ],
            },
        ];

        for case in test_cases {
            run_multiple_token_test(case);
        }
    }

    #[test]
    fn test_literals() {
        use tokens::Literal::*;
        use tokens::TokenType::*;

        let test_cases = vec![LexerMultiTokenCase {
            name: "Various literals",
            input: "42 3.14 \"hello\" 'c' true false",
            expected_tokens: vec![
                Token::new(0, 2, Literal(Int(42))),             // 42
                Token::new(3, 4, Literal(Float(3.14))),         // 3.14
                Token::new(8, 7, Literal(Str("hello".into()))), // "hello"
                Token::new(16, 3, Literal(Char('c'))),          // 'c'
                Token::new(20, 4, Literal(Bool(true))),         // true
                Token::new(25, 5, Literal(Bool(false))),        // false
            ],
        }];

        for case in test_cases {
            run_multiple_token_test(case);
        }
    }
}
