// generate unit test for utils.rs
// cargo test --test integration_test
#[cfg(test)]
mod test_utils {
    use weighing_problem::utils::{rand_add_sub, gen_vec};

    #[test]
    fn test_rand_add_sub() {
        let x = 10;
        let delta = 2;
        let y = rand_add_sub(x, delta);
        assert!(y == x + delta || y == x - delta);
    }

    #[test]
    fn test_gen_vec() {
        let n = 10;
        let x = 10;
        let y = 12;
        let v = gen_vec(n, x, y);
        let count = v.iter().filter(|&x| *x == y).count();
        assert_eq!(count, 1);
    }
}


// generate unit test for odd_balls.rs
// cargo test --test integration_test -- --ignored