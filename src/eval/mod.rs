use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use crate::parser::Object;

#[derive(Debug)]
pub struct EvalError {
    err: String
}

pub type EnvType = Rc<RefCell<Env>>;

pub struct Env {
    pub parent: Option<EnvType>,
    pub defs: HashMap<String, Object> 
}

pub fn eval(obj: Object, env: EnvType) -> Result<Object, EvalError> {
    match obj {
        Object::Void => Ok(obj.clone()),
        Object::Lambda(_strs, _obj) => Ok(Object::Void),
        Object::Integer(_) => Ok(obj.clone()),
        Object::Float(_) => Ok(obj.clone()),
        Object::Bool(_) => Ok(obj.clone()),
        Object::List(v) => eval_list(v, env),
        Object::Symbol(str) => eval_symbol(&str, env),
    }
}

// Look up the object associated with a specific symbol within given context
fn eval_symbol(symbol: &str, env: EnvType) -> Result<Object, EvalError> {
    match env.borrow().defs.get(symbol) {
        Some(obj) => Ok(obj.clone()),
        None => {
            match &env.borrow().parent {
                Some(env) => eval_symbol(symbol, env.clone()),
                None => Err(EvalError { 
                    err: String::from(format!("Symbol '{symbol}' not found within context.")) 
                })
            }
        }
    }
}

fn eval_list(objs: Vec<Object>, env: EnvType) -> Result<Object, EvalError> {
    let Object::Symbol(symbol) = &objs[0] else {
        let obj = eval(objs[0].clone(), env.clone())?;
        match obj {
            Object::Lambda(_, _) => { 
                let n_env = Env {
                    parent: Some(env.clone()),
                    defs: HashMap::from([("lambda".to_string(), obj.clone())])
                };
                return eval_function_call("lambda", &objs[1..], Rc::new(RefCell::new(n_env)))
            },
            Object::Bool(_) => return Ok(obj.clone()),
            _ => return Err(EvalError { 
                err: String::from(format!("Expected Symbol, got something else.")) 
            })
        }
    };
    match symbol.as_str() {
        "+" | "-" | "*" | "/" | "+." | "-." | "*." | "/." | "=" | ">=" | "<=" | "<" | ">" => eval_binary_op(&mut objs.into_iter(), env),
        "if" => {
            let mut objs = objs.into_iter();
            objs.next();
            eval_if(&mut objs.into_iter(), env)
        },
        "define" => {
            let mut objs = objs.into_iter();
            objs.next();
            eval_define(&mut objs.into_iter(), env)
        },
        "lambda" => {
            let mut objs = objs.into_iter();
            objs.next();
            eval_lambda(&mut objs.into_iter())
        },
        "list" => {
            let mut new_list = vec![];
            for obj in objs {
                let result = eval(obj, env.clone())?;
                match result {
                    Object::Void => {},
                    _ => new_list.push(result),
                };
            }
            Ok(Object::List(new_list))
        },
        _ => eval_function_call(&symbol, &objs[1..], env)
    }
}

fn eval_if<T>(objs: &mut T, env: EnvType) -> Result<Object, EvalError>
where 
    T: Iterator<Item = Object>
{
    let Some(bool_exp) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected expression, got nothing.")) 
        })
    };
    println!("{:?}", bool_exp);
    let Ok(Object::Bool(result)) = eval(bool_exp, env.clone()) else {
        return Err(EvalError { 
            err: String::from(format!("Expected boolean expression, got something else.")) 
        })
    };
    if !result {
        return Ok(Object::Void);
    }
    let obj = objs.next();
    match obj {
        Some(obj) => eval(obj, env.clone()),
        None => Ok(Object::Void)
    }
}

fn eval_lambda<T>(objs: &mut T) -> Result<Object, EvalError>
where 
    T: Iterator<Item = Object>
{
    let mut params = vec![];
    let Some(Object::List(symbols)) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected list of parameters, got something else.")) 
        })
    };
    for symbol in symbols {
        match symbol {
            Object::Symbol(str) => params.push(str),
            _ => return Err(EvalError { 
                err: String::from(format!("Expected symbol, got something else.")) 
            })
        }
    }
    let Some(Object::List(body)) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected function body, got something else.")) 
        })
    };
    Ok(Object::Lambda(params, body))
}

fn eval_define<T>(objs: &mut T, env: EnvType) -> Result<Object, EvalError>
where 
    T: Iterator<Item = Object>
{
    let Some(Object::Symbol(str)) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected Symbol, got something else.")) 
        })
    };

    let Some(obj) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected an expression, got something else.")) 
        })
    };

    env.borrow_mut().defs.insert(str, eval(obj, env.clone())?);

    Ok(Object::Void)
}

// Using iterator so we can avoid runtime array bound check.
// (See: https://nnethercote.github.io/perf-book/bounds-checks.html)
fn eval_binary_op<'a, T>(objs: &mut T, env: EnvType) -> Result<Object, EvalError>
where 
    T: Iterator<Item = Object>
{
    let Some(Object::Symbol(str)) = objs.next() else {
        return Err(EvalError { 
            err: String::from(format!("Expected Symbol, got something else.")) 
        })
    };

    let f = match str.as_str() {
        "+" | "+." => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Integer(a)), Object::Integer(b)) => Ok(Object::Integer(a + b)),
                (None, Object::Integer(b)) => Ok(Object::Integer(b)),
                (Some(Object::Float(a)), Object::Float(b)) => Ok(Object::Float(a + b)),
                (None, Object::Float(b)) => Ok(Object::Float(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        "-" | "-." => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Integer(a)), Object::Integer(b)) => Ok(Object::Integer(a - b)),
                (Some(Object::Float(a)), Object::Float(b)) => Ok(Object::Float(a - b)),
                (None, Object::Integer(b)) => Ok(Object::Integer(b)),
                (None, Object::Float(b)) => Ok(Object::Float(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        "*" | "*." => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Integer(a)), Object::Integer(b)) => Ok(Object::Integer(a * b)),
                (Some(Object::Float(a)), Object::Float(b)) => Ok(Object::Float(a * b)),
                (None, Object::Integer(b)) => Ok(Object::Integer(b)),
                (None, Object::Float(b)) => Ok(Object::Float(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        "/" | "/." => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Integer(a)), Object::Integer(b)) => Ok(Object::Integer(a / b)),
                (Some(Object::Float(a)), Object::Float(b)) => Ok(Object::Float(a / b)),
                (None, Object::Integer(b)) => Ok(Object::Integer(b)),
                (None, Object::Float(b)) => Ok(Object::Float(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        "=" => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Bool(a)), Object::Bool(b)) => Ok(Object::Bool(a == b)),
                (None, Object::Bool(b)) => Ok(Object::Bool(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        ">=" => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Bool(a)), Object::Bool(b)) => Ok(Object::Bool(a >= b)),
                (None, Object::Bool(b)) => Ok(Object::Bool(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        "<=" => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Bool(a)), Object::Bool(b)) => Ok(Object::Bool(a <= b)),
                (None, Object::Bool(b)) => Ok(Object::Bool(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        ">" => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Bool(a)), Object::Bool(b)) => Ok(Object::Bool(a > b)),
                (None, Object::Bool(b)) => Ok(Object::Bool(b)),
                _ => Err(EvalError { err: String::from(format!("Incompatible type.")) })
            }
        },
        _ => |a: Option<Object>, b: Object| {
            match (a, b) {
                (Some(Object::Bool(a)), Object::Bool(b)) => Ok(Object::Bool(a < b)),
                (None, Object::Bool(b)) => Ok(Object::Bool(b)),
                (Some(Object::Bool(_)), b) => Err(EvalError { err: String::from(format!("Incompatible type. Expected a boolean, got '{b:?}'")) }),
                _ => Err(EvalError { err: String::from(format!("Incompatible type. Expected a boolean.'")) })
            }
        },
    };

    let mut temp = None;
    while let Some(obj) = objs.next() {
        let result = eval(obj.clone(), env.clone())?;
        temp = Some(f(temp, result)?);
    }
    if temp.is_none() {
        return Ok(Object::Void); 
    }
    Ok(temp.unwrap())
}

fn eval_function_call(symbol: &str, objs: &[Object], env: EnvType) -> Result<Object, EvalError> {
    let Ok(Object::Lambda(strs, obj)) = eval_symbol(symbol, env.clone()) else { 
        return Err(EvalError {
            err: String::from(format!("Symbol '{symbol}' not found within context."))
        });
    };

    let (mut params, mut values) = (strs.iter(), objs.iter());

    let mut n_env = Env {
        parent: Some(env.clone()),
        defs: HashMap::new() 
    };

    loop {
        let (param, value) = (params.next(), values.next());
        // As long as I use param before this match expression I can use peek no problem.
        match (param, value) {
            (Some(param), Some(value)) => { n_env.defs.insert(param.clone(), value.clone()); },
            (Some(param), None) => return Err(EvalError {
                err: String::from(format!("Expected parameter '{param}', got empty."))
            }),
            (None, Some(_)) => return Err(EvalError {
                err: String::from(format!("Function '{symbol}' expected {} parameters, got {}.", strs.len(), objs.len()))
            }),
            _ => break
        }
    };
    eval(Object::List(obj), Rc::new(RefCell::new(n_env)))
}

