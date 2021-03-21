use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct PRNG {
    rng: ChaCha8Rng,
}

impl PRNG {
    pub fn new() -> Self {
        let rng = thread_rng();
        let rng_chacha = ChaCha8Rng::from_rng(rng).unwrap();
        Self { rng: rng_chacha }
    }

    pub fn uniform(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }

    pub fn gaussian(&mut self, mu: f64, vr: f64) -> f64 {
        // Box-Muller's Method
        let x = self.uniform(0.0, 1.0);
        let y = self.uniform(0.0, std::f64::consts::PI * 2.0);
        mu + (-2.0 * x.ln()).sqrt() * y.cos() * vr.sqrt()
    }

    pub fn exponential(&mut self, lambda: f64) -> f64 {
        let x = self.uniform(0.0, 1.0);
        -x.ln() / lambda
    }

    pub fn binom(&mut self, n: usize, p: f64) -> f64 {
        let mut r: usize = 0;
        for _ in 0..n {
            if self.uniform(0.0, 1.0) < p {
                r += 1
            }
        }
        r as f64
    }

    pub fn setseed(&mut self, state: u64) {
        self.rng = ChaCha8Rng::seed_from_u64(state);
    }
}

#[cfg(test)]
mod test_parser {
    use crate::prng::*;
    #[test]
    fn seeding() {
        let mut rng = PRNG::new();
        {
            rng.setseed(42);
            let x = rng.uniform(10.0, 20.0);
            rng.setseed(42);
            let y = rng.uniform(10.0, 20.0);
            assert_eq!(x, y);
        }
        {
            rng.setseed(42);
            let x = rng.gaussian(2.0, 1.0);
            rng.setseed(42);
            let y = rng.gaussian(2.0, 1.0);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_uniform() {
        let mut rng = PRNG::new();
        for _ in 0..10 {
            let x = rng.uniform(0.0, 1.0);
            assert!(0.0 <= x && x <= 1.0);
        }
        for _ in 0..10 {
            let x = rng.uniform(-1.0, 0.5);
            assert!(-1.0 <= x && x <= 0.5);
        }
    }

    #[test]
    fn test_gaussian() {
        let mut rng = PRNG::new();
        {
            let mut sum = 0.0;
            let n = 10;
            for _ in 0..n {
                sum += rng.gaussian(120.0, 1.0);
            }
            let mu = sum / n as f64;
            assert!(100.0 < mu && mu < 140.0);
        }
    }

    #[test]
    fn test_exponential() {
        let mut rng = PRNG::new();
        for _ in 0..10 {
            assert!(rng.exponential(0.5) > 0.0);
            assert!(rng.exponential(1.0) > 0.0);
        }
    }

    #[test]
    fn test_binom() {
        let mut rng = PRNG::new();
        for _ in 0..10 {
            let m = rng.binom(5, 0.3);
            assert!(0.0 <= m && m <= 5.0);
        }
    }
}
