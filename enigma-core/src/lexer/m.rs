pub mod size;
pub mod tokens;
use std::iter::Peekable;
use std::str::CharIndices;
use tokens::{Literal, Token, TokenType};

pub struct Lexer<'l> {
    program: &'l str,
    chars: Peekable<CharIndices<'l>>,
}

impl<'l> Lexer<'l> {
    pub fn new(program: &'l str) -> Self {
        Self {
            program,
            chars: program.char_indices().peekable(),
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
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

    fn read_identifier(&mut self, start: usize) -> &str {
        let mut end = start;
        while let Some((idx, ch)) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.next();
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
                    self.next();
                    end = idx + 1;
                }
                '.' => {
                    if is_float || has_exponent {
                        valid = false;
                    }
                    is_float = true;
                    self.next();
                    end = idx + 1;
                }
                'e' | 'E' => {
                    if has_exponent {
                        valid = false;
                    }
                    has_exponent = true;
                    self.next();
                    end = idx + 1;

                    if let Some((_, sign)) = self.peek() {
                        if sign == '+' || sign == '-' {
                            self.next();
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
            Token::new(start, end - start, TokenType::Literal(Literal::Float(value)))
        } else {
            let value = literal_str.parse().unwrap();
            Token::new(start, end - start, TokenType::Literal(Literal::Int(value)))
        }
    }

    pub fn next_token(&mut self) -> Token {
        use TokenType::*;
        self.skip_whitespace();

        let (start, ch) = match self.peek() {
            Some((pos, c)) => (pos, c),
            None => return Token::new(0, 0, Eof),
        };

        match ch {
            '#' => {
                self.skip_comment();
                return self.next_token();
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
            '_' => self.consume_single(UnderScore),
            '"' => self.read_string_literal(start),
            '\'' => self.read_char_literal(start),
            ':' => self.consume_double(':', DoubleColon, Colon, '=', Assign, Colon),
            '.' => self.consume_double('.', DotDot, Dot, '=', Dot, Dot),
            '=' => self.consume_double('=', EqualEqual, Assign, '=', Assign, Assign),
            '!' => self.consume_double('=', ExclaimEqual, Exclaim, '=', Exclaim, Exclaim),
            '<' => self.consume_double('=', LessThanEqual, LessThan, '<', LessThan, LessThan),
            '>' => self.consume_double('=', GreaterThanEqual, GreaterThan, '>', GreaterThan, GreaterThan),
            '&' => self.consume_double('&', AmpersandAmpersand, Ampersand, '=', Ampersand, Ampersand),
            '|' => self.consume_double('|', PipePipe, Pipe, '=', Pipe, Pipe),
            '$' => self.consume_double('=', Destructure, Dollar, '$', Dollar, Dollar),
            '+' => self.consume_operator(PlusPlus, PlusEqual, Plus),
            '-' => self.consume_operator(MinusMinus, MinusEqual, Minus),
            '*' => self.consume_operator(Asterisk, AsteriskEqual, Asterisk),
            '/' => self.consume_operator(Division, SlashEqual, Division),
            '0'..='9' => self.read_number_literal(start),
            'a'..='z' | 'A'..='Z' => self.handle_identifier(start),
            _ => panic!("Unexpected character: '{}' at position {}", ch, start),
        }
    }

    fn consume_single(&mut self, token_type: TokenType) -> Token {
        let (start, _) = self.next().unwrap();
        Token::new(start, 1, token_type)
    }

    fn consume_double(
        &mut self,
        second_char: char,
        double_type: TokenType,
        single_type: TokenType,
        third_char: char,
        triple_type: TokenType,
        default_type: TokenType,
    ) -> Token {
        let (start, _) = self.next().unwrap();
        match self.peek() {
            Some((_, c)) if c == second_char => {
                self.next();
                Token::new(start, 2, double_type)
            }
            Some((_, c)) if c == third_char => {
                self.next();
                Token::new(start, 2, triple_type)
            }
            _ => Token::new(start, 1, default_type),
        }
    }

    fn consume_operator(
        &mut self,
        double_type: TokenType,
        equal_type: TokenType,
        single_type: TokenType,
    ) -> Token {
        let (start, _) = self.next().unwrap();
        match self.peek() {
            Some((_, c)) if c == '=' => {
                self.next();
                Token::new(start, 2, equal_type)
            }
            Some((_, c)) if c == double_type.to_char().unwrap() => {
                self.next();
                Token::new(start, 2, double_type)
            }
            _ => Token::new(start, 1, single_type),
        }
    }

    fn handle_identifier(&mut self, start: usize) -> Token {
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
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        match token.token_type {
            TokenType::Eof => None,
            _ => Some(token),
        }
    }
}

// TokenType implementation needed for consume_operator
impl TokenType {
    fn to_char(&self) -> Option<char> {
        match self {
            TokenType::PlusPlus => Some('+'),
            TokenType::MinusMinus => Some('-'),
            _ => None,
        }
    }
}