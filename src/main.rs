pub mod parser;
pub mod eval;

use std::{
    rc::Rc,
    io::{self, Write},
    cell::RefCell,
    collections::HashMap
};
use self::eval::{ Env, EnvType };

fn main() {
    // let tokens = parser::tokenize("(+. 1.1 (+. 2. (/. 6. 2.) 3.))");
    // println!("{:?}", tokens);
    //
    // match parser::parse(&mut tokens.into_iter().peekable()) {
    //     Ok(parsed) => {
    //         println!("{parsed:?}");
    //         let env = Env { parent: None, defs: HashMap::new() };
    //         match eval::eval(parsed, Rc::new(RefCell::new(env))) {
    //             Ok(obj) => println!("{obj:?}"),
    //             Err(e) => println!("{e:?}")
    //         }
    //     } 
    //     Err(e) => println!("{e:?}")
    // }
    repl();
}

fn evaluate(str: &str, env: EnvType) {
    let tokens = parser::tokenize(str);
    // println!("{:?}", tokens);
    match parser::parse(&mut tokens.into_iter().peekable()) {
        Ok(parsed) => {
            // println!("{parsed:?}");
            match eval::eval(parsed, env) {
                Ok(obj) => println!("{obj:?}"),
                Err(e) => println!("{e:?}")
            }
        } 
        Err(e) => println!("{e:?}")
    }
}

fn repl() {
    let env = Env { parent: None, defs: HashMap::new() };
    let env = Rc::new(RefCell::new(env));
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                input.pop();
                evaluate(&input, env.clone());
            },
            Err(_) => {
                print!("\n");
                continue;
            },
        };
        print!("\n");
    }
}
