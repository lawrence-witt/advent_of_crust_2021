

// Structs

#[derive(Debug)]
pub struct Token {
    value: char
}

impl Token {
    pub fn new(value: char) -> Token {
        return Token{value}
    }

    pub fn is(&self, token: Token) -> bool {
        return self.value == token.value
    }

    pub fn is_opening(&self) -> bool {
        match self.value {
            '(' | '[' | '{' | '<' => return true,
            _ => return false
        }
    }

    pub fn get_closing(&self) -> Option<Token> {
        match self.value {
            '(' => return Some(Token::new(')')),
            '[' => return Some(Token::new(']')),
            '{' => return Some(Token::new('}')),
            '<' => return Some(Token::new('>')),
            _ => return None
        }
    }

    pub fn score(&self) -> u32 {
        match self.value {
            ')' => return 3,
            ']' => return 57,
            '}' => return 1197,
            '>' => return 25137,
            _ => return 0
        }
    }
}