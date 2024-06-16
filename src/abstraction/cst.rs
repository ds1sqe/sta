#![allow(dead_code)]

use crate::ast::{BinaryOperator, Const, Relation};

#[derive(PartialEq, Clone)]
pub enum Value {
    Bottom,
    Const(i32),
    Top,
}

pub fn incl(left: &Value, right: &Value) -> bool {
    *left == Value::Bottom || *right == Value::Top || *left == *right
}

pub fn cst(n: &Const) -> Value {
    Value::Const(n.value)
}

pub fn sat(rel: Relation, n: &Const, v: &Value) -> Value {
    match (rel, v) {
        (_, Value::Bottom) => Value::Bottom,
        (Relation::Infeq, Value::Const(val)) => {
            if *val > n.value {
                Value::Bottom
            } else {
                v.to_owned()
            }
        }
        (Relation::Sup, Value::Const(val)) => {
            if *val <= n.value {
                Value::Bottom
            } else {
                v.to_owned()
            }
        }
        (_, _) => v.to_owned(),
    }
}

pub fn join(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Bottom, any) | (any, Value::Bottom) => any.to_owned(),
        (Value::Top, _) | (_, Value::Top) => Value::Top,
        (Value::Const(l_val), Value::Const(r_val)) => {
            if *l_val == *r_val {
                left.clone()
            } else {
                Value::Top
            }
        }
    }
}

pub fn binop(bop: BinaryOperator, left: &Value, right: &Value) -> Value {
    match (bop, left, right) {
        (_, Value::Bottom, _) | (_, _, Value::Bottom) => Value::Bottom,
        (BinaryOperator::Add, Value::Top, _) | (BinaryOperator::Add, _, Value::Top) => Value::Top,
        (BinaryOperator::Add, Value::Const(l_val), Value::Const(r_val)) => {
            Value::Const(*l_val + *r_val)
        }
        (BinaryOperator::Sub, Value::Const(_), Value::Const(_)) => todo!(),
        (BinaryOperator::Sub, Value::Const(_), Value::Top) => todo!(),
        (BinaryOperator::Sub, Value::Top, Value::Const(_)) => todo!(),
        (BinaryOperator::Sub, Value::Top, Value::Top) => todo!(),
        (BinaryOperator::Mul, Value::Const(_), Value::Const(_)) => todo!(),
        (BinaryOperator::Mul, Value::Const(_), Value::Top) => todo!(),
        (BinaryOperator::Mul, Value::Top, Value::Const(_)) => todo!(),
        (BinaryOperator::Mul, Value::Top, Value::Top) => todo!(),
    }
}
