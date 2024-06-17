#![allow(dead_code)]

use crate::{
    ast::{Command, Cond, Expression, LCom, Label},
    program::Program,
};

use super::{
    no_rel::{binop, cst, sat, NrEnv, NrValue},
    Storage,
};

pub fn expr(e: &Expression, env: &NrEnv) -> NrValue {
    match e {
        Expression::Con(n) => cst(n),
        Expression::Var(x) => env.mem[x.id].clone(),
        Expression::Bop { left, bop, right } => binop(bop, &expr(left, env), &expr(right, env)),
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

pub fn post_lfp<F>(mut func: F, env: &mut NrEnv) -> NrEnv
where
    F: FnMut(&NrEnv) -> NrEnv,
{
    let next_env = func(env);
    if next_env.is_le(env) {
        env.to_owned()
    } else {
        post_lfp(func, &mut env.join(&next_env))
    }
}

pub fn com(storage: &mut Storage, lcom: &LCom, env: NrEnv) -> NrEnv {
    storage.add(&lcom.label, env.join(storage.find(&lcom.label)));

    if env.is_bot() {
        env
    } else {
        match *lcom.command.clone() {
            Command::Skip => env,
            Command::Seq { c0, c1 } => {
                let c0_env = com(storage, &c0, env);

                com(storage, &c1, c0_env)
            }
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
            Command::If { cond, c0, c1 } => com(storage, &c0, self::cond(&cond, &env)).join(&com(
                storage,
                &c1,
                self::cond(&cond.negate(), &env),
            )),
            Command::While { cond, c } => {
                let mut env = env;
                self::cond(
                    &cond.negate(),
                    &post_lfp(
                        |inner_env| self::com(storage, &c, self::cond(&cond, inner_env)),
                        &mut env,
                    ),
                )
            }
        }
    }
}

pub fn step(l: &LCom, next: &Label, env: &NrEnv) -> Vec<(Label, NrEnv)> {
    match *l.command.clone() {
        Command::Skip => vec![(*next, env.clone())],
        Command::Seq { c0, c1 } => self::step(&c0, &c1.label, env),
        Command::Assign { var, expr } => {
            let nrv = self::expr(&expr, env);
            let mut new_env = env.clone();
            new_env.mem[var.id] = nrv;

            vec![(*next, new_env)]
        }
        Command::Input { var } => {
            let mut new_env = env.clone();
            new_env.mem[var.id] = NrValue::Top;
            vec![(*next, new_env)]
        }
        Command::If { cond, c0, c1 } => {
            vec![
                (c0.label, self::cond(&cond, env)),
                (c1.label, self::cond(&cond.negate(), env)),
            ]
        }
        Command::While { cond, c } => {
            vec![
                (c.label, self::cond(&cond, env)),
                (*next, self::cond(&cond.negate(), env)),
            ]
        }
    }
}

pub fn iterate(prog: Program, storage: &mut Storage, env: &NrEnv) {
    assert!(!prog.ins.is_empty());

    let mut jobs = vec![Label { id: 0 }];
    let mut envs = Storage {
        mem: vec![env.clone()],
    };

    while let Some(label) = jobs.pop() {
        let command = prog.find(&label);
        let next = prog.next(&label);

        let env = envs.find(&label);

        let posts = self::step(
            &LCom {
                label,
                command: Box::new(command.clone()),
            },
            next,
            env,
        );

        posts.iter().for_each(|(post_label, post_env)| {
            let old_env = storage.find(post_label);
            if !post_env.is_le(old_env) {
                let new_env = old_env.join(post_env);
                envs.add(post_label, new_env);
                jobs.push(*post_label);
            }
        })
    }
}
