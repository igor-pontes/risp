use crate::parser::{ 
    Object,
    ObjectType,
    FieldType
};

use crate::parser::Op;

pub fn eval (obj: &mut Vec<ObjectType>) {
    match obj.pop() {
        Some(obj) => println!("{:?}", eval_exp(obj)),
        None => ()
    }
}

fn eval_exp<'a>((obj, field_t): ObjectType) -> FieldType {
    match obj {
        Object::Expr(op, vec) => {
            match vec {
                Some(mut vec) => eval_op(op, &mut vec),
                None => FieldType::Unit
            }
        }
        Object::Number(str) => {
            match field_t {
                FieldType::Integer(_) => {
                    match str.parse::<isize>() {
                        Ok(number) => FieldType::Integer(number),
                        Err(..) => panic!("Error evaluating number.")
                    }
                },
                _ => {
                    match str.parse::<f64>() {
                        Ok(number) => FieldType::Float(number),
                        Err(..) => panic!("Error evaluating number.")
                    }
                }
            }
        }
    }
}

fn eval_op (op: Op, obj: &mut Vec<(Object, FieldType)>) -> FieldType {
    let mut out = match obj.pop() {
        Some(obj) => eval_exp(obj),
        None => FieldType::Unit
    };
    match op {
        Op::Add => {
            while let Some(ft) = obj.pop() {
                match (eval_exp(ft), out) {
                    (FieldType::Integer(a), FieldType::Integer(b)) => out = FieldType::Integer(b + a),
                    (FieldType::Float(a), FieldType::Float(b)) => out = FieldType::Float(b + a),
                    (ft, FieldType::Unit) => out = ft,
                    _ => continue
                }
            }
        }
        Op::Sub => {
            while let Some(ft) = obj.pop() {
                match (eval_exp(ft), out) {
                    (FieldType::Integer(a), FieldType::Integer(b)) => out = FieldType::Integer(a - b),
                    (FieldType::Float(a), FieldType::Float(b)) => out = FieldType::Float(a - b),
                    (_, FieldType::Integer(b)) => out = FieldType::Integer(-b),
                    (_, FieldType::Float(b)) => out = FieldType::Float(-b),
                    (ft, FieldType::Unit) => out = ft,
                }
            }
        }
        Op::Mul => {
            while let Some(ft) = obj.pop() {
                match (eval_exp(ft), out) {
                    (FieldType::Integer(a), FieldType::Integer(b)) => out = FieldType::Integer(a * b),
                    (FieldType::Float(a), FieldType::Float(b)) => out = FieldType::Float(a * b),
                    (ft, FieldType::Unit) => out = ft,
                    _ => continue
                }
            }
        }
        Op::Div => {
            while let Some(ft) = obj.pop() {
                match (eval_exp(ft), out) {
                    (FieldType::Integer(_), FieldType::Integer(0)) => panic!("Error: DivisionByZero"),
                    (FieldType::Float(_), FieldType::Float(b)) if b == 0.0 => panic!("Error: DivisionByZero"),
                    (FieldType::Integer(a), FieldType::Integer(b)) => out = FieldType::Integer(a - b),
                    (FieldType::Float(a), FieldType::Float(b)) => out = FieldType::Float(a - b),
                    (ft, FieldType::Unit) => out = ft,
                    _ => continue
                }
            }
        }
        Op::Empty => ()
    }
    out
}

