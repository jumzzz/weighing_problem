use crate::utils::{rand_add_sub, gen_vec};

pub struct OddBalls {
    pub v: Vec<u64>,
    pub odd_pos: usize,
    pub n_balls: usize,
    pub odd_value: u64,
}

impl OddBalls {
    pub fn new(n: usize, x: u64, delta: u64) -> Self {
        // panic if n is not divisble by 3
        assert!(n % 3 == 0, "n must be divisible by 3. Got: {}", n);

        let odd = rand_add_sub(x, delta);
        let v = gen_vec(n, x, odd);
        let odd_pos = v.iter().position(|&x| x == odd).unwrap();
        Self { v, odd_pos, n_balls: n, odd_value: odd }
    }

}