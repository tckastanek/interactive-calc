use rustyline::{error::ReadlineError, Editor};

struct Parser<'a> {
    input: &'a str,
    tokens: Option<Vec<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            tokens: None,
        }
    }

    pub fn tokenize(mut self) -> Self {
        let mut tokens: Vec<Token> = Vec::new();

        for raw_token in self.input.split(" ") {
            let token = Token::from_str(raw_token);

            tokens.push(token);
        }

        self.tokens = Some(tokens);

        self
    }

    pub fn parse(self) -> f32 {
        if let Some(tokens) = self.tokens {
            let mut lhs = None;
            let mut op = None;

            for token in tokens {
                match token {
                    Token::Float(float) => match (lhs, &op) {
                        (None, _) => lhs = Some(float),
                        (Some(lhs_val), Some(op_token)) => match op_token {
                            Token::Plus => {
                                lhs = Some(lhs_val + float);
                                op = None;
                            }
                            Token::Minus => {
                                lhs = Some(lhs_val - float);
                                op = None;
                            }
                            _ => unimplemented!(),
                        },
                        (_, _) => unimplemented!(),
                    },
                    Token::Plus => match (lhs, &op) {
                        (Some(_), None) => op = Some(Token::Plus),
                        (_, _) => unimplemented!(),
                    },
                    Token::Minus => match (lhs, &op) {
                        (Some(_), None) => op = Some(Token::Minus),
                        (_, _) => unimplemented!(),
                    },
                }
            }

            // eval
            lhs.unwrap()
        } else {
            unimplemented!();
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Float(f32),
    Plus,
    Minus,
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
                _ => unimplemented!(),
            }
        }
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut result = None;

    loop {
        let readline = match result {
            Some(result) => {
                let prompt = format!(">> {} ", result);
                rl.readline(&prompt)
            }
            None => rl.readline(">> "),
        };
        // let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let parser = Parser::new(&line);

                let res = parser.tokenize().parse();
                result = Some(res);

                println!("{}", &line);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
}
