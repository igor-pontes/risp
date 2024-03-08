pub mod parser;
pub mod eval;

fn main() {
    // let mut objs = parser::parse("((+ 1))");
    // let mut objs = parser::parse("/ (* 10 10) (+ 1 1 1) (+. 1.1 1.1)");
    // let mut objs = parser::parse("+ 1 1");
    // let mut objs = parser::parse("+. 1. (+. 1. 1.)");
    let mut objs = parser::parse("+. 1. (+. 2. (1.) 3.)");
    println!("{:?}", objs);
    eval::eval(&mut objs);
}

