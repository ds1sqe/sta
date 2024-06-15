use crate::ast::{Const, Label, Variable};

pub type Memory = [Const; 100];

pub fn read<'a>(x: &Variable, m: &'a Memory) -> &'a Const {
    &m[x.id]
}
pub fn write<'a>(x: &Variable, n: &Const, m: &'a mut Memory) -> &'a mut Memory {
    m[x.id] = *n;
    m
}

pub struct State {
    pub label: Label,
    pub mem: Memory,
}
