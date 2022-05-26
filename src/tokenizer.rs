#[derive(Clone)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart,
    LoopEnd,
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for command in source.chars() {
        match command {
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            '.' => tokens.push(Token::Write),
            ',' => tokens.push(Token::Read),
            '[' => tokens.push(Token::LoopStart),
            ']' => tokens.push(Token::LoopEnd),
            _ => (),
        }
    }
    return tokens;
}
