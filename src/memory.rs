use crate::ast::{Const, Label, Variable};

pub type Memory = [Const; 100];

pub fn read<'a>(x: &'a Variable, m: &'a Memory) -> &'a Const {
    &m[x.id]
}
pub fn write<'a>(x: &'a Variable, n: &'a Const, m: &'a Memory) -> Memory {
    let mut new_memory = m.clone();
    new_memory[x.id] = *n;
    new_memory
}

pub struct State {
    pub label: Label,
    pub mem: Memory,
}
