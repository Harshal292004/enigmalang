use super::size::Size;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    // Int variant of Literal contains i64 type
    Int(usize),
    Float(f64),
    Str(String),
}

// Debug for {:?}
// PartialEq for comparison
// Eq for stricter comparison
// Clone for `.clone()`
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    // OOP
    Rune,     // rune
    Summon,   // summon
    SelfType, // self
    Draws,    // draws

    // Func
    Spell,   // spell
    Chant,   // chant
    Shatter, // shatter
    Phase,   // phase

    // Declarations
    Bind, // bind
    Seal, // seal

    //  Conditionals
    Reveal,  // reveal
    Veil,    // veil
    Divine,  // divine
    Sigil,   // sigil
    Default, // default

    // Loops
    Invoke, // invoke
    Linger, // linger
    Range,  // ..
    In,     // in

    // Pkg Management
    Call, // call
    As,   //as

    // Syntax
    LCurly, // {
    RCurly, // }
    LParen, // (
    RParen, // )
    Colon,  // :
    Comma,  // ,
    Dot,    // .
    Assign, // =

    // Operators
    And,              // &&
    Or,               // ||
    Not,              // !
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Division,         // /
    BitAnd,           // &
    BitOr,            // |
    BitXor,           // ^
    EqualEqual,       // ==
    NotEqual,         // !=
    LessThan,         // <
    GreaterThan,      // >
    LessThanEqual,    // <=
    GreaterThanEqual, // >=

    // Identifiers and literals
    Identifier, // variable names, type names
    // Literal variant of TokenType holds Literal type
    Literal(Literal),

    Eof, // end of file
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub size: Size,
}

impl Token {
    pub fn new(start: usize, end: usize, token_type: TokenType) -> Self {
        Self {
            token_type: token_type,
            size: Size {
                start: start,
                end: end,
            },
        }
    }
}
