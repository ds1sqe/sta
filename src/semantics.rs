use std::io::stdin;

use crate::{
    ast::{BinaryOperator, Command, Cond, Const, Expression, LCom, Relation, Variable},
    memory::{read, write, Memory},
};

/// HACK: may panic
fn input() -> Const {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    Const {
        value: buffer.parse::<i32>().unwrap(),
    }
}

fn binop(bop: &BinaryOperator, c0: &Const, c1: &Const) -> Const {
    match bop {
        BinaryOperator::Add => Const {
            value: c0.value + c1.value,
        },
        BinaryOperator::Sub => Const {
            value: c0.value - c1.value,
        },
        BinaryOperator::Mul => Const {
            value: c0.value * c1.value,
        },
    }
}

/// [[E]]: M --> V
pub fn sem_expr<'a>(expr: &'a Expression, mem: &'a Memory) -> Const {
    match expr {
        // [[n]](m) == n
        Expression::Con(cst) => *cst,
        // [[x]](m) == m(x)
        Expression::Var(var) => *read(var, mem),
        // [[E0 (.) E1]](m) = f(.) ([[E0]](m), [[E1]](m))
        Expression::Bop { left, bop, right } => {
            binop(bop, &sem_expr(left, mem), &sem_expr(right, mem))
        }
    }
}

fn relop(rel: &Relation, var_value: &Const, cst: &Const) -> bool {
    match rel {
        Relation::Infeq => var_value.value <= cst.value,
        Relation::Sup => var_value.value > cst.value,
    }
}

/// [[B]]: M --> B
pub fn sem_cond(cond: &Cond, mem: &Memory) -> bool {
    // [[x (<) n]](m) == f(<) (m(x),n)
    relop(&cond.rel, read(&cond.var, mem), &cond.con)
}

/// [[C]]p: P(M) --> P(M)
pub fn sem_command<'a>(l: &LCom, mem: &'a mut Memory) -> &'a mut Memory {
    match l.command.as_ref() {
        // [[skip]]p(M)  == M
        Command::Skip => mem,
        // [[C0;C1]]p(M)  == [[C1]]p([[C0]]p(M))
        Command::Seq { c0, c1 } => sem_command(c1, sem_command(c0, mem)),
        // [[x:=E]]p(M)   == {m[x|-> [[E]](m) | m <- M ] }
        Command::Assign { var, expr } => write(var, &sem_expr(expr, mem), mem),
        // [[input(x)]]p(M)  == {m[x|-> n] | m <- M, n <- V}
        Command::Input { var } => write(var, &input(), mem),
        // [[if(B){C0}else{C1}]]p(M) == [[C0]]p(F(M)) U [[C1]]p(F!(M))
        Command::If { cond, c0, c1 } => {
            if sem_cond(cond, mem) {
                sem_command(c0, mem)
            } else {
                sem_command(c1, mem)
            }
        }
        // [[while(B){C}]]p(M)  == F\(!B) ( U \(i>=b) ([[C]]p o F\(B))^i (M) )
        Command::While { cond, c } => {
            if sem_cond(cond, mem) {
                sem_command(
                    &LCom {
                        label: l.label,
                        command: Box::new(Command::While {
                            cond: *cond,
                            c: c.clone(),
                        }),
                    },
                    sem_command(c, mem),
                )
            } else {
                mem
            }
        }
    }
}
