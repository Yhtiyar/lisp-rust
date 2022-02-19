use std::{fs::read, io::stdin};

//import lexer

mod interpretator;
mod lexer;
mod nodes;
mod parser;

fn main() {
    let mut interpretator = interpretator::Interpretator::new(None);
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let source = buffer.trim();
        let result: String = match interpretator.run(source.to_owned()) {
            Ok(v) => format!("{:?}", v),
            Err(e) => format!("{}", e),
        };
        println!("{}", result);
    }
}
