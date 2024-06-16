#![allow(dead_code)]

use crate::ast::{BinaryOperator, Const, Relation};

#[derive(PartialEq)]
pub enum NrValue {
    Bottom,
    Top,
    Positive,
    Negative,
}

pub struct NrEnv {
    mem: Vec<NrValue>,
}

pub fn includes(left: NrValue, right: NrValue) -> bool {
    left == NrValue::Bottom || right == NrValue::Top || left == right
}

pub fn nrv_cst(n: &Const) -> NrValue {
    if n.value < 0 {
        NrValue::Negative
    } else {
        NrValue::Positive
    }
}

pub fn nrv_sat(rel: Relation, n: &Const, nrv: NrValue) -> NrValue {
    if nrv == NrValue::Bottom {
        NrValue::Bottom
    } else if rel == Relation::Infeq && n.value < 0 {
        if nrv == NrValue::Positive {
            NrValue::Bottom
        } else {
            NrValue::Negative
        }
    } else if rel == Relation::Sup && n.value >= 0 {
        if nrv == NrValue::Negative {
            NrValue::Bottom
        } else {
            NrValue::Positive
        }
    } else {
        nrv
    }
}

pub fn nrv_join(left: NrValue, right: NrValue) -> NrValue {
    match (left, right) {
        (NrValue::Bottom, any) | (any, NrValue::Bottom) => any,
        (NrValue::Top, _)
        | (_, NrValue::Top)
        | (NrValue::Positive, NrValue::Negative)
        | (NrValue::Negative, NrValue::Positive) => NrValue::Top,
        (NrValue::Positive, NrValue::Positive) => NrValue::Positive,
        (NrValue::Negative, NrValue::Negative) => NrValue::Negative,
    }
}

pub fn nrv_binop(bop: BinaryOperator, left: NrValue, right: NrValue) -> NrValue {
    match (bop, left, right) {
        (_, NrValue::Bottom, _) | (_, _, NrValue::Bottom) => NrValue::Bottom,

        (BinaryOperator::Add, NrValue::Positive, NrValue::Positive) => NrValue::Positive,
        (BinaryOperator::Add, NrValue::Negative, NrValue::Negative) => NrValue::Negative,

        (BinaryOperator::Mul, NrValue::Positive, NrValue::Positive)
        | (BinaryOperator::Mul, NrValue::Negative, NrValue::Negative) => NrValue::Positive,
        (BinaryOperator::Mul, NrValue::Positive, NrValue::Negative)
        | (BinaryOperator::Mul, NrValue::Negative, NrValue::Positive) => NrValue::Negative,

        (BinaryOperator::Add, NrValue::Top, NrValue::Top) => todo!(),
        (BinaryOperator::Add, NrValue::Top, NrValue::Positive) => todo!(),
        (BinaryOperator::Add, NrValue::Top, NrValue::Negative) => todo!(),
        (BinaryOperator::Add, NrValue::Positive, NrValue::Top) => todo!(),
        (BinaryOperator::Add, NrValue::Positive, NrValue::Negative) => todo!(),
        (BinaryOperator::Add, NrValue::Negative, NrValue::Top) => todo!(),
        (BinaryOperator::Add, NrValue::Negative, NrValue::Positive) => todo!(),
        (BinaryOperator::Sub, NrValue::Top, NrValue::Top) => todo!(),
        (BinaryOperator::Sub, NrValue::Top, NrValue::Positive) => todo!(),
        (BinaryOperator::Sub, NrValue::Top, NrValue::Negative) => todo!(),
        (BinaryOperator::Sub, NrValue::Positive, NrValue::Top) => todo!(),
        (BinaryOperator::Sub, NrValue::Positive, NrValue::Positive) => todo!(),
        (BinaryOperator::Sub, NrValue::Positive, NrValue::Negative) => todo!(),
        (BinaryOperator::Sub, NrValue::Negative, NrValue::Top) => todo!(),
        (BinaryOperator::Sub, NrValue::Negative, NrValue::Positive) => todo!(),
        (BinaryOperator::Sub, NrValue::Negative, NrValue::Negative) => todo!(),
        (BinaryOperator::Mul, NrValue::Top, NrValue::Top) => todo!(),
        (BinaryOperator::Mul, NrValue::Top, NrValue::Positive) => todo!(),
        (BinaryOperator::Mul, NrValue::Top, NrValue::Negative) => todo!(),
        (BinaryOperator::Mul, NrValue::Positive, NrValue::Top) => todo!(),
        (BinaryOperator::Mul, NrValue::Negative, NrValue::Top) => todo!(),
    }
}

impl NrEnv {
    pub fn map_bot(&mut self) {
        self.mem = self.mem.iter_mut().map(|_| NrValue::Bottom).collect();
    }

    pub fn is_bot(&self) -> bool {
        self.mem.iter().any(|nrv| *nrv == NrValue::Bottom)
    }

    pub fn is_le(&self, other: &Self) {

        // self.mem.iter()
    }
}
