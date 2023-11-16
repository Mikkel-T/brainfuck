use log::debug;
use std::fmt;

/// Position in the source file of a character
#[derive(Clone, Debug)]
pub struct Position {
    /// The line of the character
    pub line: usize,
    /// The column of the character
    pub col: usize,
}

/// Token that is generated from the source and can be parsed into an instruction by the parser
#[derive(Clone, Debug)]
pub enum Token {
    /// Move pointer right
    Right,
    /// Move pointer left
    Left,
    /// Increment current cell
    Increment,
    /// Decrement current cell
    Decrement,
    /// Output value of current cell
    Write,
    /// Replace value of current cell with input
    Read,
    /// Start of loop
    LoopStart(Position),
    /// End of loop
    LoopEnd(Position),
}

impl Token {
    /// Convert a token to the corresponding source char
    pub fn to_char(&self) -> char {
        match self {
            Token::Right => '>',
            Token::Left => '<',
            Token::Increment => '+',
            Token::Decrement => '-',
            Token::Write => '.',
            Token::Read => ',',
            Token::LoopStart(_) => '[',
            Token::LoopEnd(_) => ']',
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// Convert a string to a vec of tokens
pub fn tokenize(source: String) -> Vec<Token> {
    debug!("Tokenizing source");
    let mut tokens = Vec::new();
    for (i, line) in source.lines().enumerate() {
        for (j, command) in line.chars().enumerate() {
            match command {
                '>' => tokens.push(Token::Right),
                '<' => tokens.push(Token::Left),
                '+' => tokens.push(Token::Increment),
                '-' => tokens.push(Token::Decrement),
                '.' => tokens.push(Token::Write),
                ',' => tokens.push(Token::Read),
                '[' => tokens.push(Token::LoopStart(Position {
                    line: i + 1,
                    col: j + 1,
                })),
                ']' => tokens.push(Token::LoopEnd(Position {
                    line: i + 1,
                    col: j + 1,
                })),
                _ => (),
            }
        }
    }
    debug!("Done tokenizing");
    tokens
}

/// Convert a vec of tokens to a string
pub fn source_from_tokens(tokens: Vec<Token>) -> String {
    return tokens.iter().map(|token| token.to_char()).collect();
}
