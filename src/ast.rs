#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Label {
    pub id: usize,
}

/// n <- V
#[derive(Clone, Copy)]
pub struct Const {
    pub value: i32,
}

impl PartialOrd for Const {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl PartialEq for Const {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// x <- X
#[derive(Clone, Copy)]
pub struct Variable {
    pub id: usize,
}

/// (.) ::= + | - | * ...
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
}

/// (<) ::= < | <= | == | ...
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Relation {
    /// <
    Infeq,
    /// >=
    Sup,
}

/// E ::= n | x | E (.) E
#[allow(dead_code)]
#[derive(Clone)]
pub enum Expression {
    Con(Const),
    Var(Variable),
    Bop {
        left: Box<Expression>,
        bop: BinaryOperator,
        right: Box<Expression>,
    },
}

/// B ::= x (<) n
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Cond {
    pub var: Variable,
    pub rel: Relation,
    pub con: Const,
}
impl Cond {
    pub fn negate(&self) -> Self {
        let mut new_cond = *self;
        match new_cond.rel {
            Relation::Infeq => new_cond.rel = Relation::Sup,
            Relation::Sup => new_cond.rel = Relation::Infeq,
        }
        new_cond
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct LCom {
    pub label: Label,
    pub command: Box<Command>,
}

/// C ::= commands
#[allow(dead_code)]
#[derive(Clone)]
pub enum Command {
    /// skip
    Skip,
    /// C; C
    Seq { c0: LCom, c1: LCom },
    /// x := E
    Assign { var: Variable, expr: Expression },
    /// input(x)
    Input { var: Variable },
    /// if (B) {C} else {C}
    If { cond: Cond, c0: LCom, c1: LCom },
    /// while (B) {C}
    While { cond: Cond, c: LCom },
}
