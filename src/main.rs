pub mod lexer;

use std::fs::File;
use std::io::{BufReader, Read};

use lexer::Lexer;
use lexer::Token;

fn main() {
    let file_result = File::open("codigo.txt");
  
    let file = match file_result {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let mut reader = BufReader::new(file);

    let mut input_string = String::new();
    if let Err(err) = reader.read_to_string(&mut input_string) {
        eprintln!("Error reading file: {}", err);
        return;
    }

    let mut lexer = Lexer::new(input_string.chars());

    loop {
        let token = lexer.get_token();
        if token == Token::EOF {
            break;
        }
        println!("{:?}", token);
    }
}
