use rand::Rng;


// Randomly add or subtract x with y with 50% chance
pub fn rand_add_sub(val: u64, delta: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let rnd_inx = rng.gen_range(0..2) as usize;

    let v = [val + delta, val - delta];
    v[rnd_inx]
}

pub fn gen_vec(n: usize, x: u64, y: u64) -> Vec<u64> {
    let mut v = vec![x; n];
    
    let mut rng = rand::thread_rng();
    let rnd_idx = rng.gen_range(0..n);
    v[rnd_idx] = y;
    v
}
