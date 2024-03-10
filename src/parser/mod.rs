use core::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(isize),
    Float(f64),
    Symbol(String),
    Lparen,
    Rparen
}

#[derive(Debug)]
pub enum Object {
    Integer(isize),
    Bool(bool),
    Float(f64),
    Unit,
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

#[derive(Debug)]
pub struct ParseError {
    err: String
}

pub fn parse<'a, I>(tokens: &mut Peekable<I>) -> Result<Object, ParseError>
where
    I: Iterator<Item = Token>
{
    let token = tokens.next();
    if Some(Token::Lparen) != token {
        return Err(ParseError {
            err: format!("Expected LParen, got {token:?}"),
        });
    };

    let mut objs = vec![];
    while let Some(token) = tokens.peek() {
        match token {
            Token::Integer(n) => objs.push(Object::Integer(*n)),
            Token::Float(n) => objs.push(Object::Float(*n)),
            Token::Symbol(str) => objs.push(Object::Symbol(str.clone())),
            Token::Lparen => {
                let obj = parse(tokens)?;
                objs.push(obj);
            },
            Token::Rparen => return Ok(Object::List(objs)),
        }
        tokens.next();
    };

    Err(ParseError {
        err: format!("Insufficient tokens.")
    })
}

pub fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut symbols = str.chars().enumerate().peekable();
    while let Some((index, symbol)) = symbols.next() {
        match symbol {
            // Handle white space
            '(' => tokens.push(Token::Lparen),
            ')' => tokens.push(Token::Rparen),
            ' ' => (),
            _ => {
                let start = index;
                let end = get_next_symbol(&mut symbols, start);
                let symbol = &str[start..=end];
                match (symbol.parse::<isize>(), symbol.parse::<f64>()) {
                    (Ok(n), _) => tokens.push(Token::Integer(n)),
                    (_, Ok(n)) => tokens.push(Token::Float(n)),
                    _ => tokens.push(Token::Symbol(symbol.to_string())),
                }
            },
        }
    }
    tokens
}

fn get_next_symbol<I>(symbols: &mut Peekable<I>, index: usize) -> usize
where 
    I: Iterator<Item = (usize, char)>
{
    let mut end = index;
    let mut is_float = false;
    while let Some((_, chr)) = symbols.peek() {
        if chr.is_alphanumeric() || chr == &'.' || chr == &'_' { 
            if chr == &'.' && !is_float { 
                is_float = !is_float;
            }
            symbols.next();
            end += 1;
        } else { 
            break
        }
    }
    end 
}

