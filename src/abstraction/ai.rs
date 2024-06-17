#![allow(dead_code)]

use crate::ast::{Command, Cond, Expression, LCom};

use super::no_rel::{binop, cst, sat, NrEnv, NrValue};

pub fn expr(e: &Expression, env: &NrEnv) -> NrValue {
    match e {
        Expression::Con(n) => cst(n),
        Expression::Var(x) => env.mem[x.id].clone(),
        Expression::Bop { left, bop, right } => {
            //
            binop(bop, &expr(left, env), &expr(right, env))
        }
    }
}

pub fn cond(cond: &Cond, env: &NrEnv) -> NrEnv {
    let mut r_env = env.clone();
    let nrv = sat(&cond.rel, &cond.con, &env.mem[cond.var.id]);
    if nrv == NrValue::Bottom {
        r_env.map_bot();
    } else {
        r_env.mem[cond.var.id] = nrv;
    }
    r_env
}

pub fn post_lfp<F>(func: F, env: &NrEnv) -> NrEnv
where
    F: Fn(&NrEnv) -> NrEnv,
{
    let next_env = func(env);
    if next_env.is_le(env) {
        env.to_owned()
    } else {
        post_lfp(func, &env.join(&next_env))
    }
}

pub fn com(lcom: &LCom, env: NrEnv) -> NrEnv {
    if env.is_bot() {
        env
    } else {
        match *lcom.command.clone() {
            Command::Skip => env,
            Command::Seq { c0, c1 } => com(&c1, com(&c0, env)),
            Command::Assign { var, expr } => {
                let nrv = self::expr(&expr, &env);
                let mut r_env = env.clone();
                r_env.mem[var.id] = nrv;
                r_env
            }
            Command::Input { var } => {
                let mut r_env = env.clone();
                r_env.mem[var.id] = NrValue::Top;
                r_env
            }
            Command::If { cond, c0, c1 } => {
                com(&c0, self::cond(&cond, &env)).join(&com(&c1, self::cond(&cond.negate(), &env)))
            }
            Command::While { cond, c } => self::cond(
                &cond.negate(),
                &post_lfp(
                    |inner_env| self::com(&c, self::cond(&cond, inner_env)),
                    &env,
                ),
            ),
        }
    }
}
