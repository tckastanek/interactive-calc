#[derive(Debug)]
pub enum Token {
    Float(f32),
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl Token {
    pub fn from_str(input: &str) -> Self {
        // let float = input.parse::<f32>().unwrap();
        if let Ok(float) = input.parse::<f32>() {
            Token::Float(float)
        } else {
            match input {
                "+" => Token::Plus,
                "-" => Token::Minus,
                "/" => Token::Divide,
                "*" => Token::Multiply,
                _ => unimplemented!(),
            }
        }
    }
}
