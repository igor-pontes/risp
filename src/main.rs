pub mod parser;
pub mod eval;

fn main() {
    // let mut objs = parser::parse("+. 1. (+. 2. (1. 1.) 3.)");
    // let mut objs = parser::parse("+. 1.1 (+. 2. (/. 6. 2.) 3.)");
    
    // let tokens = parser::tokenize("(+. 1.1 (+. 2. (/. 6. 2.) 3.))");
    let tokens = parser::tokenize("(+. 1.1 (+. 2. (/. 6. 2.) 3.))");
    println!("{:?}", tokens);
    let parsed = parser::parse(&mut tokens.into_iter().peekable());
    println!("{parsed:?}");
}

