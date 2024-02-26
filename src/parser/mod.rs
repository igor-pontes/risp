use Object::*;
use core::iter::Peekable;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Empty
}

#[derive(Debug)]
pub enum Object {
    Expr(Op,Vec<Object>),
    Number(usize),
}

pub fn parse(str: &str) {
    let mut program = vec![];
    let mut symbols = str.chars().enumerate().peekable();
    match str.chars().next() {
        Some('(') => program.push(parse_exp(&mut symbols, str, Op::Empty)),
        Some(_) => program.push(parse_op(&mut symbols, str)),
        None => (),
    }
    if let Some((index, chr)) = symbols.next() {
        panic!("Error parsing code. Character '{}' at {}.", chr, index);
    }
    println!("{:?}", program);
}

pub fn parse_op<I>(symbols: &mut Peekable<I>, str: &str) -> Object
where
    I: Iterator<Item = (usize, char)>
{
    let op = match symbols.next() {
        Some((_, '+')) => Op::Add,
        Some((_, '-')) => Op::Sub,
        Some((_, '/')) => Op::Div,
        Some((_, '*')) => Op::Mul,
        Some((_, ' ')) => return parse_op(symbols, str),
        _ => panic!("Error parsing operator.")
    };
    return parse_exp(symbols, str, op);
}

pub fn parse_exp<I>(symbols: &mut Peekable<I>, str: &str, op: Op) -> Object
where
    I: Iterator<Item = (usize, char)>
{
    let mut exp = vec![];
    
    while let Some((index, chr)) = symbols.next() {
        match chr {
            '(' => {
                if let Some((_, '(')) = symbols.peek() {
                    exp.push(parse_exp(symbols, str, Op::Empty));
                } else {
                    exp.push(parse_op(symbols, str));
                }
                match symbols.next() {
                    Some((_, ')')) => (),
                    _ => panic!("Error here."),
                }
            },
            _ if chr.is_ascii_digit() => {
                let start = index;
                let end = process_number(symbols, start);
                let number = &str[start..=end].parse::<usize>();
                match number {
                    Ok(n) => exp.push(Number(*n)),
                    _ => panic!("Error trying to parse number."),
                }

            },
            ' ' => (),
            c => panic!("Error parsing expression. {c}")
        }
        if let Some((_, ')')) = symbols.peek() { 
            break; 
        }
    }

    Expr(op, exp)
}

fn process_number<T>(symbols: &mut Peekable<T>, index: usize) -> usize 
where 
    T: Iterator<Item = (usize, char)>
{
    let mut end = index;
    while let Some((_, chr)) = symbols.peek() {
        if chr.is_ascii_digit() { 
            symbols.next();
            end += 1;
        } else { 
            break 
        }
    }
    end
}

