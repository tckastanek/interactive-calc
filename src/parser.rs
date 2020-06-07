use crate::token::Token;
pub struct Parser<'a> {
    input: &'a str,
    tokens: Option<Vec<Token>>,
    prev_result: Option<f32>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, prev_result: Option<f32>) -> Self {
        Self {
            input,
            tokens: None,
            prev_result,
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
            let mut lhs = match self.prev_result {
                Some(float) => Some(float),
                None => None,
            };
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
                            Token::Divide => {
                                lhs = Some(lhs_val / float);
                                op = None;
                            }
                            _ => unimplemented!(),
                        },
                        (Some(_), None) => lhs = Some(float),
                    },
                    Token::Plus => match (lhs, &op) {
                        (Some(_), None) => op = Some(Token::Plus),
                        (_, _) => unimplemented!(),
                    },
                    Token::Minus => match (lhs, &op) {
                        (Some(_), None) => op = Some(Token::Minus),
                        (_, _) => unimplemented!(),
                    },
                    Token::Divide => match (lhs, &op) {
                        (Some(_), None) => op = Some(Token::Divide),
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
