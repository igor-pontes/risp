use core::iter::Peekable;

// If we use &str we won't be able to use an Iterator while parsing the string.
// (problems with mutability)
#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(isize),
    Float(f64),
    Symbol(String),
    Lparen,
    Rparen
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(isize),
    Bool(bool),
    Float(f64),
    Void,
    Symbol(String),
    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
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
            Token::Symbol(str) => {
                let obj = match str.as_str() {
                    "true" => Object::Bool(true),
                    "false" => Object::Bool(false),
                    _ => Object::Symbol(str.clone())
                };
                objs.push(obj)
            },
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
    while let Some((_, chr)) = symbols.peek() {
        if chr.is_alphanumeric() || chr == &'.' || chr == &'_' { 
            symbols.next();
            end += 1;
        } else { 
            break
        }
    }
    end 
}

