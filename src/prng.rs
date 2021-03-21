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
    fn test_exponential() {
        let mut rng = PRNG::new();
        for _ in 0..10 {
            assert!(rng.exponential(0.5) > 0.0);
            assert!(rng.exponential(1.0) > 0.0);
        }
    }
}
