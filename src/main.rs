mod parser;
mod token;

use crate::parser::Parser;
use rustyline::{error::ReadlineError, Editor};

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
                if line == "clear" {
                    result = None;
                } else {
                    let parser = Parser::new(&line, result);

                    let res = parser.tokenize().parse();
                    result = Some(res);

                    println!("{}", &line);
                }
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
