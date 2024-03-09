pub use Object::*;
use std::fmt::{ Formatter, Display, Result };
use core::iter::Peekable;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldType {
    Integer(isize),
    Float(f64),
    Unit
}

impl Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Integer(_) => write!(f, "Integer"),
            Self::Float(_) => write!(f, "Float"),
            Self::Unit => write!(f, "()"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Empty
}

pub type ObjectType<'a> = (Object<'a>, FieldType);

#[derive(Debug)]
pub enum Object<'a> {
    Expr(Op, Option<Vec<ObjectType<'a>>>),
    Number(&'a str),
}

pub fn parse(str: &str) -> Vec<ObjectType> {
    let mut program = vec![];
    let mut symbols = str.chars().enumerate().peekable();
    match str.chars().next() {
        // Handle white space
        Some('(') => program.push(parse_exp(&mut symbols, str, Op::Empty, FieldType::Unit)),
        Some(_) => program.push(parse_op(&mut symbols, str, None)),
        None => (),
    }
    if let Some((index, chr)) = symbols.next() {
        panic!("Parsing error. Character '{}' at {}.", chr, index);
    }
    program
}

pub fn parse_op<'a, I>(symbols: &mut Peekable<I>, str: &'a str, field_t: Option<FieldType>) -> ObjectType<'a>
where
    I: Iterator<Item = (usize, char)>
{
    // TODO: Use HashMap later.
    let (op, op_t) = match symbols.peek() {
        Some((_, '+')) => {
            symbols.next();
            if let Some((_, '.')) = symbols.peek() {
                symbols.next();
                (Op::Add, FieldType::Float(0.))
            } else {
                (Op::Add, FieldType::Integer(0))
            }
        },
        Some((index, 'a')) => {
            let index = index.clone();
            if &str[index..index+3] == "add" { 
                symbols.next();
                symbols.next();

                symbols.next();
                (Op::Add, FieldType::Integer(0))
            } else {
                panic!("Expected one of Operator or Expression, but got 'a' at {index}.");
            }
        },
        Some((_, '-')) => {
            symbols.next();
            if let Some((_, '.')) = symbols.peek() {
                symbols.next();
                (Op::Sub, FieldType::Float(0.))
            } else {
                (Op::Sub, FieldType::Integer(0))
            }
        },
        Some((index, 's')) => {
            let index = index.clone();
            if &str[index..index+3] == "sub" { 
                symbols.next();
                symbols.next();

                symbols.next();
                (Op::Sub, FieldType::Integer(0))
            } else {
                panic!("Expected one of Operator or Expression, but got 's' at {index}.");
            }
        },
        Some((_, '/')) => {
            symbols.next();
            if let Some((_, '.')) = symbols.peek() {
                symbols.next();
                (Op::Div, FieldType::Float(0.))
            } else {
                (Op::Div, FieldType::Integer(0))
            }
        },
        Some((index, 'd')) => {
            let index = index.clone();
            if &str[index..index+3] == "sub" { 
                symbols.next();
                symbols.next();

                symbols.next();
                (Op::Div, FieldType::Integer(0)) 
            } else {
                panic!("Expected one of Operator or Expression, but got 'd' at {index}.");
            }
        },
        Some((_, '*')) => {
            symbols.next();
            if let Some((_, '.')) = symbols.peek() {
                symbols.next();
                (Op::Mul, FieldType::Float(0.))
            } else {
                (Op::Mul, FieldType::Integer(0))
            }
        },
        Some((index, 'm')) => {
            let index = index.clone();
            if &str[index..index+3] == "sub" { 
                symbols.next();
                symbols.next();

                symbols.next();
                (Op::Mul, FieldType::Integer(0)) 
            } else {
                panic!("Expected one of Operator or Expression, but got 'm' at {index}.");
            }
        },
        Some((_, ' ')) => {
            symbols.next();
            return parse_op(symbols, str, field_t)
        },
        Some(_) => { 
            let temp = match field_t {
                Some(field_t) => parse_exp(symbols, str, Op::Empty, field_t),
                None => parse_exp(symbols, str, Op::Empty, FieldType::Unit)
            };
            return temp
        },
        None => panic!("No more characters to be processed!")
    };
    // symbols.next();
    
    match field_t {
        Some(field_t) => if op_t != field_t { panic!("Expected return type to be {field_t}, got {op_t}.") },
        None => ()
    };
    parse_exp(symbols, str, op, op_t)
}

pub fn parse_exp<'a, I>(symbols: &mut Peekable<I>, str: &'a str, op: Op, field_t: FieldType) -> ObjectType<'a>
where
    I: Iterator<Item = (usize, char)>
{
    let mut exp = vec![];
    
    if let Some((_, ')')) = symbols.peek() { 
        return (Expr(op, None), FieldType::Unit) 
    }

    while let Some((index, chr)) = symbols.next() {
        if let (Op::Empty, true, true) = (op, exp.len() != 0, chr != ' ') { panic!("Error: excessive number of expressions at {index}.")} 
        match chr {
            '(' => {
                if let Some((_, '(')) = symbols.peek() {
                    let s_exp = parse_exp(symbols, str, Op::Empty, field_t);
                    if s_exp.1 != field_t { panic!("Error: Sub-expression type does not match. Expected '{field_t}', got '{}' at {index}", s_exp.1) } 
                    exp.push(s_exp);
                } else {
                    let s_exp = parse_op(symbols, str, Some(field_t));
                    if s_exp.1 != field_t { panic!("Error: Sub-expression type does not match. Expected '{field_t}', got '{}' at {index}", s_exp.1) } 
                    exp.push(s_exp);
                }
                match symbols.next() {
                    Some((_, ')')) => (),
                    Some((i, chr)) => panic!("Expected ')' ({index}), found '{chr} at {i}'"),
                    _ => panic!("Expected ')' to close expression at {index}, got empty."),
                }
            },
            chr if chr.is_ascii_digit() => {
                let start = index;
                let Some((end, number_t)) = process_number(symbols, start) else { panic!("Error parsing: expected a number at {}.", start) };
                match (field_t, number_t) {
                    (FieldType::Integer(_), FieldType::Float(_)) => panic!("Error: expected '{field_t}', got '{number_t}' type at position {start}."),
                    (FieldType::Float(_), FieldType::Integer(_)) => panic!("Error: expected '{field_t}', got '{number_t}' type at position {start}."),
                    _ => (),
                }
                let number = (Number(&str[start..=end]), number_t);
                exp.push(number)
            },
            ' ' => (),
            c => panic!("Error parsing expression: '{c}' at {index}")
        }
        if let Some((_, ')')) = symbols.peek() { 
            break; 
        }
    }

    (Expr(op, Some(exp)), field_t)
}

fn process_number<T>(symbols: &mut Peekable<T>, index: usize) -> Option<(usize, FieldType)>
where 
    T: Iterator<Item = (usize, char)>
{
    let mut end = index;
    let mut field_t = FieldType::Integer(0);
    while let Some((_, chr)) = symbols.peek() {
        if chr.is_ascii_digit() || chr == &'.' { 
            if chr == &'.' { 
                if field_t != FieldType::Float(0.) {
                    field_t = FieldType::Float(0.); 
                } else {
                    return None;
                }
            }
            symbols.next();
            end += 1;
        } else { 
            break
        }
    }

    Some((end, field_t))
}

