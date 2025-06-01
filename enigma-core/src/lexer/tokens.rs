use super::size::Size;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(usize),
    Float(f64),
    Str(String),
    Bool(bool),
    Char(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Get,      // get
    Module,   // module
    As,       // as
    Mut,      // mut
    Return,   // return
    If,       // if
    Else,     // else
    For,      // for
    In,       // in
    Loop,     // loop
    While,    // while
    Match,    // match
    Case,     // case
    Pub,      // pub
    Impl,     // implement
    Record,   // record
    Union,    // union
    Ref,      // ref
    Deref,    // deref
    RawRef,   // raw_ref
    Unsafe,   // unsafe
    Protoc,   // protoc
    Asm,      // asm
    Continue, // continue
    Break,    // break

    // Symbols & Operators
    Assign,      // :=
    Colon,       // :
    DoubleColon, // ::
    Arrow,       // ->
    Comma,       // ,
    Dot,         // .
    DotDot,      // ..
    Percent,     // %
    LParen,      // (
    RParen,      // )
    LCurly,      // {
    RCurly,      // }
    LSquare,     // [
    RSquare,     // ]
    UnderScore,  // _
    Destructure, // $=
    Question,    // ?

    // Operators
    Plus,               // +
    Minus,              // -
    PlusEqual,          // +=
    MinusEqual,         // -=
    Asterisk,           // *
    AsteriskEqual,      // *=
    Slash,              // /
    SlashEqual,         // /=
    Ampersand,          // &
    AmpersandAmpersand, // &&
    Pipe,               // |
    PipePipe,           // ||
    Carrot,             // ^
    EqualEqual,         // ==
    Exclaim,            // !
    ExclaimEqual,       // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanEqual,      // <=
    GreaterThanEqual,   // >=
    PlusPlus,           // ++
    MinusMinus,         // --
    Dollar,             // $=
    // Special
    Func,       // @
    ReturnSemi, // shorthand return `val;`

    // Values
    Identifier,
    Literal(Literal),
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub size: Size,
}

impl Token {
    pub fn new(start: usize, end: usize, token_type: TokenType) -> Self {
        Self {
            token_type,
            size: Size { start, end },
        }
    }
}
