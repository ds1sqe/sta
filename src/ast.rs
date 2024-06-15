pub struct Label {
    pub id: i32,
}

/// n <- V
#[derive(Clone, Copy)]
pub struct Const {
    pub value: i32,
}

/// x <- X
pub struct Variable {
    pub id: usize,
}

/// (.) ::= + | - | * ...
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
}

/// (<) ::= < | <= | == | ...
pub enum Relation {
    Infeq,
    Sup,
}

/// E ::= n | x | E (.) E
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
pub struct Cond {
    pub var: Variable,
    pub rel: Relation,
    pub con: Const,
}

pub struct LCom {
    label: Label,
    command: Box<Command>,
}

/// C ::= commands
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
