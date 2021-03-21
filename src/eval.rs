use crate::expr::Expr;
use crate::prng::PRNG;

pub fn eval(expr: &Expr) -> f64 {
    let mut rng = PRNG::new();
    let mut ret: f64 = 0.0;
    for t in expr.code.iter() {
        match t.0.as_str() {
            "seed" => {
                assert_eq!(t.1.len(), 1);
                let state = t.1[0] as u64;
                rng.setseed(state);
            }
            "int" | "floor" => {
                ret = ret.floor();
            }
            "round" => {
                ret = ret.round();
            }
            "uniform" => {
                let args = fillarg(&t.1, &mut vec![0.0, 1.0]);
                ret = rng.uniform(args[0], args[1]);
            }
            "gauss" | "gaussian" | "normal" | "norm" => {
                let args = fillarg(&t.1, &mut vec![0.0, 1.0]);
                ret = rng.gaussian(args[0], args[1]);
            }
            "exp" | "exponential" => {
                let args = fillarg(&t.1, &mut vec![1.0]);
                ret = rng.exponential(args[0]);
            }
            _ => {
                panic!(format!("Unknown function: {}", t.0));
            }
        }
    }
    ret
}

fn fillarg(given: &Vec<f64>, args: &mut Vec<f64>) -> Vec<f64> {
    let n = given.len();
    let m = args.len();
    for i in 0..n.min(m) {
        args[i] = given[i];
    }
    args.to_vec()
}
