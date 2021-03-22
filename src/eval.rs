use crate::expr::Expr;
use crate::prng::PRNG;

extern crate anyhow;
use anyhow::{bail, Result};

macro_rules! assert_arity {
    (= $arity:expr, $term:expr) => {
        if $arity != $term.1.len() {
            bail!("Arity Error: {} required {} args", $term.0, $arity);
        }
    };
    (<= $arity:expr, $term:expr) => {
        if $arity < $term.1.len() {
            bail!("Arity Error: {} required <= {} args", $term.0, $arity);
        }
    };
}

pub fn eval(expr: &Expr) -> Result<f64> {
    let mut rng = PRNG::new();
    let mut ret: f64 = 0.0;
    for t in expr.code.iter() {
        match t.0.as_str() {
            "seed" => {
                assert_arity!(= 1, &t);
                let state = t.1[0] as u64;
                rng.setseed(state);
            }
            "int" | "floor" => {
                assert_arity!(= 0, &t);
                ret = ret.floor();
            }
            "round" => {
                assert_arity!(= 0, &t);
                ret = ret.round();
            }
            "uniform" => {
                assert_arity!(<= 2, &t);
                let args = fillarg(&t.1, &mut vec![0.0, 1.0]);
                if args[0] >= args[1] {
                    bail!("uniform required non-empty range");
                }
                ret = rng.uniform(args[0], args[1]);
            }
            "gauss" | "gaussian" | "normal" | "norm" => {
                assert_arity!(<= 2, &t);
                let args = fillarg(&t.1, &mut vec![0.0, 1.0]);
                if args[1] <= 0.0 {
                    bail!("variance for gauss must be positive");
                }
                ret = rng.gaussian(args[0], args[1]);
            }
            "exp" | "exponential" => {
                assert_arity!(<= 1, &t);
                let args = fillarg(&t.1, &mut vec![1.0]);
                if args[0] <= 0.0 {
                    bail!("lambda for exp must be positive");
                }
                ret = rng.exponential(args[0]);
            }
            "binom" => {
                assert_arity!(= 2, &t);
                let n = t.1[0] as usize;
                let p = t.1[1];
                if n as f64 != t.1[0] {
                    bail!("n for binom must be unsigned int");
                }
                if p < 0.0 || p > 1.0 {
                    bail!("p for binom must be `0 <= p <= 1`");
                }
                ret = rng.binom(n, p);
            }
            "bernoulli" => {
                assert_arity!(<= 1, &t);
                let args = fillarg(&t.1, &mut vec![0.5]);
                let p = args[0];
                if p < 0.0 || p > 1.0 {
                    bail!("p for bernoulli must be `0 <= p <= 1`");
                }
                ret = rng.binom(1, p);
            }
            _ => {
                bail!("Unknown function: {}", t.0);
            }
        }
    }
    Ok(ret)
}

fn fillarg(given: &Vec<f64>, args: &mut Vec<f64>) -> Vec<f64> {
    let n = given.len();
    let m = args.len();
    for i in 0..n.min(m) {
        args[i] = given[i];
    }
    args.to_vec()
}
