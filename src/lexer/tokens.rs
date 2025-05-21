use super::position::Postion;

#[derive(Debug,PartialEq,Clone)]
pub enum Literal{
    // Int variant of Literal contains i64 type
    Int(i64),
    Float(f64),
    Str(String)
}

// Debug for {:?}
// PartialEq for comparison 
// Eq for stricter comparison
// Clone for `.clone()`
#[derive(Debug,PartialEq, Clone)]
pub enum TokenType{
    // Keywords
    // OOP
    Rune, // rune
    Summon, // summon
    SelfType, // self
    Draws, // draws

    // Func
    Spell, // spell
    Chant, // chant
    Shatter, // shatter
    Phase, // phase

    // Declarations
    Bind, // bind
    Seal, // seal

    //  Conditionals
    Reveal, // reveal
    Veil, // veil
    Divine, // divine
    Sigil, // sigil
    Default, // default    

    // Loops
    Invoke, // invoke 
    Linger, // linger
    Range, // ..
    In, // in 
    
    // Pkg Management
    Call, // call
    As, //as 

    // Syntax
    LCurly,        // {
    RCurly,        // }
    LParen,        // (
    RParen,        // )
    Colon,         // :
    Comma,         // ,
    Dot,           // .
    Assign,        // =
    

    // Operators
    And,           // &&
    Or,            // ||
    Not,           // !
    Plus,          // +
    Minus,         // -
    Asterisk,      // *
    Slash,         // /
    BitAnd,        // &
    BitOr,         // |
    BitXor,        // ^
    EqualEqual,    // ==
    NotEqual,      // !=
    LessThan,      // <
    GreaterThan,   // >
    LessThanEqual, // <=
    GreaterThanEqual, // >=

    // Identifiers and literals
    Identifier,    // variable names, type names
    // Literal variant of TokenType holds Literal type
    Literal(Literal),

    Eof            // end of file
}


#[derive(Debug,PartialEq,Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub postion: Postion
}