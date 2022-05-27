use std::fmt;

#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone)]
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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.col + 1)
    }
}

pub fn tokenize(source: String) -> Vec<Token> {
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
                '[' => tokens.push(Token::LoopStart(Position { line: i, col: j })),
                ']' => tokens.push(Token::LoopEnd(Position { line: i, col: j })),
                _ => (),
            }
        }
    }
    return tokens;
}

pub fn source_from_tokens(tokens: Vec<Token>) -> String {
    let mut chars = Vec::new();
    for token in tokens {
        match token {
            Token::Right => chars.push('>'),
            Token::Left => chars.push('<'),
            Token::Increment => chars.push('+'),
            Token::Decrement => chars.push('-'),
            Token::Write => chars.push('.'),
            Token::Read => chars.push(','),
            Token::LoopStart(_) => chars.push('['),
            Token::LoopEnd(_) => chars.push(']'),
        }
    }
    return chars.iter().collect();
}
