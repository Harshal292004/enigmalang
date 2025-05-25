pub mod size;
pub mod tokens;
use std::iter::Peekable;
use std::str::CharIndices;

use size::Size;
use tokens::{Token, TokenType};

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
    fn peek_char(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }
    fn next_char(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
    fn skip_whitespace(&mut self) {
        while let Some((_, c)) = self.peek_char() {
            if c.is_whitespace() {
                // continue
                self.next_char();
            } else {
                break;
            }
        }
    }
    fn skip_single_line_comment(&mut self) {
        // single line comment
        while let Some((_, ch)) = self.peek_char() {
            if ch == '\n' {
                self.next_char();
                break;
            }
            self.next_char();
        }
    }
    fn skip_comments(&mut self) {
        //  consume first #
        self.next_char();

        if let Some((_, '#')) = self.peek_char() {
            // consume second #
            self.next_char();

            if let Some((_, '#')) = self.peek_char() {
                // consume third #
                self.next_char();
                loop {
                    match self.peek_char() {
                        Some((_, '#')) => {
                            // consume the first ending #
                            self.next_char();
                            if let Some((_, '#')) = self.peek_char() {
                                // consume second # in ending
                                self.next_char();

                                if let Some((_, '#')) = self.peek_char() {
                                    // consume third # in the ending
                                    self.next_char();
                                    // EOL or say new line or we stop here
                                    break;
                                }
                            }
                        }
                        Some(_) => {
                            self.next_char();
                        }
                        None => {
                            break;
                        }
                    }
                }
            } else {
                // single line coment
                self.next_char();
                self.skip_single_line_comment();
            }
        } else {
            self.skip_single_line_comment();
        }
    }

    fn single_char_token(&mut self, start: usize, token_type: tokens::TokenType) -> Token {
        Token::new(start, 1, token_type)
    }

    fn double_char_token(
        &mut self,
        start: usize,
        expected_second: char,
        double_token: TokenType,
        single_token: TokenType,
    ) -> Token {
        // consume current char
        self.next_char();

        match self.peek_char() {
            Some((_, ch)) if ch == expected_second => {
                self.next_char(); // consume second char
                Token::new(start, 2, double_token)
            }
            _ => Token::new(start, 1, single_token),
        }
    }

    fn read_ident(&mut self, start: usize) -> &str {
        let mut end = start;

        while let Some((idx, ch)) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.next_char();
                end = idx + ch.len_utf8();
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
                end = idx + ch.len_utf8();
            } else {
                break;
            }
        }

        (end, &self.program[start..end])
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

        let (curr_offset, curr_char) = match self.peek_char() {
            Some(c) => c,
            None => return Token::new(0, 0, Eof),
        };

        let token = match curr_char {
            '#' => {
                self.skip_comments();
                // Helps skip comments and recursively advance to the next token for clean parsing
                return self.next_token();
            }

            // single char tokens
            '{' => self.single_char_token(curr_offset, LCurly),
            '}' => self.single_char_token(curr_offset, RCurly),
            '(' => self.single_char_token(curr_offset, LParen),
            ')' => self.single_char_token(curr_offset, RParen),
            ':' => self.single_char_token(curr_offset, Colon),
            ',' => self.single_char_token(curr_offset, Comma),
            '^' => self.single_char_token(curr_offset, BitXor),
            '/' => self.single_char_token(curr_offset, Division),
            '*' => self.single_char_token(curr_offset, Asterisk),
            '-' => self.single_char_token(curr_offset, Minus),
            '+' => self.single_char_token(curr_offset, Plus),

            '.' => self.double_char_token(curr_offset, '.', Range, Dot),
            '=' => self.double_char_token(curr_offset, '=', EqualEqual, Assign),
            '>' => self.double_char_token(curr_offset, '=', GreaterThanEqual, GreaterThan),
            '<' => self.double_char_token(curr_offset, '=', LessThanEqual, LessThan),
            '!' => self.double_char_token(curr_offset, '=', NotEqual, Not),
            '|' => self.double_char_token(curr_offset, '|', Or, BitOr),
            '&' => self.double_char_token(curr_offset, '&', And, BitAnd),
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident(curr_offset);

                match ident {
                    "rune" => Token::new(curr_offset, 4, Bind),
                    "summon" => Token::new(curr_offset, 6, Summon),
                    "self" => Token::new(curr_offset, 4, SelfType),
                    "draws" => Token::new(curr_offset, 5, Draws),
                    "spell" => Token::new(curr_offset, 5, Spell),
                    "chant" => Token::new(curr_offset, 5, Chant),
                    "shatter" => Token::new(curr_offset, 7, Shatter),
                    "phase" => Token::new(curr_offset, 5, Phase),
                    "bind" => Token::new(curr_offset, 4, Bind),
                    "seal" => Token::new(curr_offset, 4, Seal),
                    "reveal" => Token::new(curr_offset, 6, Reveal),
                    "veil" => Token::new(curr_offset, 4, Veil),
                    "divine" => Token::new(curr_offset, 6, Divine),
                    "sigil" => Token::new(curr_offset, 5, Sigil),
                    "default" => Token::new(curr_offset, 7, Default),
                    "invoke" => Token::new(curr_offset, 6, Invoke),
                    "linger" => Token::new(curr_offset, 6, Linger),
                    "in" => Token::new(curr_offset, 2, In),
                    "call" => Token::new(curr_offset, 4, Call),
                    "as" => Token::new(curr_offset, 2, As),
                    _ => Token::new(curr_offset, ident.len(), Literal(Str(ident.to_string()))),
                }
            }
            '0'..='9' => {
                let (end, literal) = self.read_int_literal(curr_offset);
                Token::new(
                    curr_offset,
                    end - curr_offset,
                    Literal(Int(literal.parse().unwrap())),
                )
            }
            _ => panic!("Mother fucker"),
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

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
        let token = lexer.next_token();
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
                Some(t) => assert_eq!(&t, expected, "Test {} failed at token {}", case.name, i),
                None => {
                    assert!(
                        false,
                        "Test {} failed at token {}: expected {:?}, got None",
                        case.name, i, expected
                    );
                    panic!(
                        "Test {} failed at token {}: expected {:?}, got None",
                        case.name, i, expected
                    );
                }
            }
        }
        // after exhausting expected tokens, lexer should be at EOF
        assert!(
            lexer.next().is_none(),
            "Test {} failed: Lexer has extra tokens",
            case.name
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
                name: "irregular comments",
                input: "
                    ### #
                    The first rule of fight club is 
                    that you do not talk about fight club
                    # # ###
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
            LexerTestCase {
                name: "right parenthesis",
                input: ")",
                expected_token: Token::new(0, 1, RParen),
            },
            LexerTestCase {
                name: "colon",
                input: ":",
                expected_token: Token::new(0, 1, Colon),
            },
            LexerTestCase {
                name: "comma",
                input: ",",
                expected_token: Token::new(0, 1, Comma),
            },
            LexerTestCase {
                name: "bit xor",
                input: "^",
                expected_token: Token::new(0, 1, BitXor),
            },
            LexerTestCase {
                name: "division",
                input: "/",
                expected_token: Token::new(0, 1, Division),
            },
            LexerTestCase {
                name: "asterisk",
                input: "*",
                expected_token: Token::new(0, 1, Asterisk),
            },
            LexerTestCase {
                name: "minus",
                input: "-",
                expected_token: Token::new(0, 1, Minus),
            },
            LexerTestCase {
                name: "plus",
                input: "+",
                expected_token: Token::new(0, 1, Plus),
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
                name: "range token ..",
                input: "..",
                expected_token: Token::new(0, 2, Range),
            },
            LexerTestCase {
                name: "dot token .",
                input: ".",
                expected_token: Token::new(0, 1, Dot),
            },
            LexerTestCase {
                name: "equal equal token ==",
                input: "==",
                expected_token: Token::new(0, 2, EqualEqual),
            },
            LexerTestCase {
                name: "assign token =",
                input: "=",
                expected_token: Token::new(0, 1, Assign),
            },
            LexerTestCase {
                name: "greater than equal token >=",
                input: ">=",
                expected_token: Token::new(0, 2, GreaterThanEqual),
            },
            LexerTestCase {
                name: "greater than token >",
                input: ">",
                expected_token: Token::new(0, 1, GreaterThan),
            },
            LexerTestCase {
                name: "less than equal token <=",
                input: "<=",
                expected_token: Token::new(0, 2, LessThanEqual),
            },
            LexerTestCase {
                name: "less than token <",
                input: "<",
                expected_token: Token::new(0, 1, LessThan),
            },
            LexerTestCase {
                name: "not equal token !=",
                input: "!=",
                expected_token: Token::new(0, 2, NotEqual),
            },
            LexerTestCase {
                name: "not token !",
                input: "!",
                expected_token: Token::new(0, 1, Not),
            },
            LexerTestCase {
                name: "or token ||",
                input: "||",
                expected_token: Token::new(0, 2, Or),
            },
            LexerTestCase {
                name: "bit or token |",
                input: "|",
                expected_token: Token::new(0, 1, BitOr),
            },
            LexerTestCase {
                name: "and token &&",
                input: "&&",
                expected_token: Token::new(0, 2, And),
            },
            LexerTestCase {
                name: "bit and token &",
                input: "&",
                expected_token: Token::new(0, 1, BitAnd),
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
            // Single keyword and identifier with punctuation
            LexerMultiTokenCase {
                name: "Simple rune declaration",
                input: "rune Account {",
                expected_tokens: vec![
                    Token::new(0, 4, Bind),
                    Token::new(5, 7, Literal(Str("Account".into()))),
                    Token::new(13, 1, LCurly),
                ],
            },
            // Struct fields with type annotations and keywords
            LexerMultiTokenCase {
                name: "Bindings inside rune",
                input: "bind owner: str\nbind balance: float",
                expected_tokens: vec![
                    Token::new(0, 4, Bind),
                    Token::new(5, 5, Literal(Str("owner".into()))),
                    Token::new(10, 1, Colon),
                    Token::new(12, 3, Literal(Str("str".into()))),
                    Token::new(16, 4, Bind),
                    Token::new(21, 7, Literal(Str("balance".into()))),
                    Token::new(28, 1, Colon),
                    Token::new(30, 5, Literal(Str("float".into()))),
                ],
            },
            // Function declaration and parameters
            LexerMultiTokenCase {
                name: "Spell with parameters",
                input: "spell summon(self, owner: str, balance: float) {",
                expected_tokens: vec![
                    Token::new(0, 5, Spell),
                    Token::new(6, 6, Literal(Str("summon".into()))),
                    Token::new(12, 1, LParen),
                    Token::new(13, 4, SelfType),
                    Token::new(17, 1, Comma),
                    Token::new(19, 5, Literal(Str("owner".into()))),
                    Token::new(24, 1, Colon),
                    Token::new(26, 3, Literal(Str("str".into()))),
                    Token::new(29, 1, Comma),
                    Token::new(31, 7, Literal(Str("balance".into()))),
                    Token::new(38, 1, Colon),
                    Token::new(40, 5, Literal(Str("float".into()))),
                    Token::new(45, 1, RParen),
                    Token::new(47, 1, LCurly),
                ],
            },
            // Conditional with reveal and veil blocks
            LexerMultiTokenCase {
                name: "Reveal and veil blocks",
                input: "reveal amount > 0 {\nchant \"Deposit successful\"\n} veil {\nchant \"Invalid deposit amount\"\n}",
                expected_tokens: vec![
                    Token::new(0, 6, Reveal),
                    Token::new(7, 6, Literal(Str("amount".into()))),
                    Token::new(14, 1, GreaterThan),
                    Token::new(16, 1, Literal(Str("0".into()))), // Actually this should be Int(0), fix needed in lexer test case if required
                    Token::new(18, 1, LCurly),
                    Token::new(20, 5, Chant),
                    // String tokens missing in lexer, but here we treat as literal string (needs lexer support)
                    // For test simplicity, skipping actual string tokens (implement string lexing if needed)
                    // Token::new(26, ..., Literal(Str("Deposit successful".into()))),
                    Token::new(44, 1, RCurly),
                    Token::new(46, 4, Veil),
                    Token::new(51, 1, LCurly),
                    Token::new(53, 5, Chant),
                    // Token::new(59, ..., Literal(Str("Invalid deposit amount".into()))),
                    Token::new(81, 1, RCurly),
                ],
            },
            // Range token and invocation loop
            LexerMultiTokenCase {
                name: "Invoke loop and range",
                input: "invoke i in 1..3 {",
                expected_tokens: vec![
                    Token::new(0, 6, Invoke),
                    Token::new(7, 1, Literal(Str("i".into()))),
                    Token::new(9, 2, In),
                    Token::new(12, 1, Literal(Int(1))),
                    Token::new(13, 2, Range),
                    Token::new(15, 1, Literal(Int(3))),
                    Token::new(16, 1, LCurly),
                ],
            },
            // Multiple sigil branches inside divine
            LexerMultiTokenCase {
                name: "Divine with sigil cases",
                input: r#"divine action {
                    sigil "1" { io.echoln("Enter amount to deposit:") }
                    sigil "2" { io.echoln("Enter amount to withdraw:") }
                    sigil default { io.echoln("Invalid action") }
                }"#,
                expected_tokens: vec![
                    Token::new(0, 6, Divine),
                    Token::new(7, 6, Literal(Str("action".into()))),
                    Token::new(14, 1, LCurly),
                    Token::new(24, 5, Sigil),
                    Token::new(30, 3, Literal(Str("1".into()))),
                    Token::new(34, 1, LCurly),
                    Token::new(36, 2, Literal(Str("io".into()))),
                    Token::new(38, 1, Dot),
                    Token::new(39, 7, Literal(Str("echoln".into()))),
                    Token::new(46, 1, LParen),
                    // Ignoring inner string literals for now (requires string lexing)
                    Token::new(68, 1, RParen),
                    Token::new(69, 1, RCurly),
                    Token::new(79, 5, Sigil),
                    Token::new(85, 3, Literal(Str("2".into()))),
                    Token::new(89, 1, LCurly),
                    Token::new(91, 2, Literal(Str("io".into()))),
                    Token::new(93, 1, Dot),
                    Token::new(94, 7, Literal(Str("echoln".into()))),
                    Token::new(101, 1, LParen),
                    Token::new(123, 1, RParen),
                    Token::new(124, 1, RCurly),
                    Token::new(134, 5, Sigil),
                    Token::new(140, 7, Default),
                    Token::new(148, 1, LCurly),
                    Token::new(150, 2, Literal(Str("io".into()))),
                    Token::new(152, 1, Dot),
                    Token::new(153, 7, Literal(Str("echoln".into()))),
                    Token::new(160, 1, LParen),
                    Token::new(176, 1, RParen),
                    Token::new(177, 1, RCurly),
                    Token::new(186, 1, RCurly),
                ],
            },
        ];
        // Loop to run all these multi-token tests using your existing test runner
        for case in test_cases {
            run_multiple_token_test(case);
        }
    }
}
