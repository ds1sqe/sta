use crate::ast::Label;

use self::no_rel::NrEnv;

mod ai;
mod cst;
mod no_rel;

pub struct Storage {
    mem: Vec<NrEnv>,
}

impl Storage {
    /// retrieve abstract state at given label
    pub fn find<'a>(&'a self, l: &Label) -> &'a NrEnv {
        &self.mem[l.id]
    }
    pub fn add(&mut self, l: &Label, nr_env: NrEnv) {
        self.mem[l.id] = nr_env;
    }
}
