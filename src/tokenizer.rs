use log::debug;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart(Position),
    LoopEnd(Position),
}

impl Token {
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
    return tokens;
}

pub fn source_from_tokens(tokens: Vec<Token>) -> String {
    return tokens.iter().map(|token| token.to_char()).collect();
}
