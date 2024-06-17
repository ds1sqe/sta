#![allow(dead_code)]

use crate::{
    ast::{Command, LCom, Label},
    memory::{write, Memory, State},
    semantics::{input, sem_cond, sem_expr},
};

pub struct Program {
    pub ins: Vec<Command>,
}

impl Program {
    /// inputs a label and then returns the successor label
    pub fn next<'a>(&'a self, label: &Label) -> &'a Label {
        todo!()
    }

    /// inputs a label and returns the command
    pub fn find<'a>(&'a self, label: &Label) -> &'a Command {
        &self.ins[label.id]
    }

    pub fn step(&self, mem: &mut Memory, com: &LCom) -> State {
        match self.find(&com.label) {
            Command::Skip => State {
                label: self.next(&com.label).to_owned(),
                mem: mem.to_owned(),
            },
            Command::Seq { c0, c1 } => todo!(),
            Command::Assign { var, expr } => State {
                label: self.next(&com.label).to_owned(),
                mem: write(var, &sem_expr(expr, mem), mem).to_owned(),
            },
            Command::Input { var } => State {
                label: self.next(&com.label).to_owned(),
                mem: write(var, &input(), mem).to_owned(),
            },
            Command::If { cond, c0, c1 } => {
                if sem_cond(cond, mem) {
                    State {
                        label: c0.label,
                        mem: mem.to_owned(),
                    }
                } else {
                    State {
                        label: c1.label,
                        mem: mem.to_owned(),
                    }
                }
            }
            Command::While { cond, c } => {
                if sem_cond(cond, mem) {
                    State {
                        label: c.label,
                        mem: mem.to_owned(),
                    }
                } else {
                    State {
                        label: self.next(&com.label).to_owned(),
                        mem: mem.to_owned(),
                    }
                }
            }
        }
    }
}
