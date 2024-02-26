pub mod parser;

fn main() {
    parser::parse("+ 5 (* 2 2) ");
    parser::parse("(+ 5 (* 2 2))");
    parser::parse("(+ 5 ((* 2 2)))");
    parser::parse("+ 5 (* 2 2))");
}
