
// Some tests of dependencies, more as a demo than to serve as verification.
#[cfg(test)]
mod tests {
    use rand::prelude::StdRng;
    use rand::Rng;
    use rand::thread_rng;
    use rand::SeedableRng;
    use lazy_static::lazy_static;

    // https://rust-random.github.io/book/overview.html
    #[test]
    fn rand_autoseed() {
        let mut rng = thread_rng();
        let uni: f64 = rng.gen();
        assert!(uni >= 0.0);
        assert!(uni < 1.0);
    }

    #[test]
    fn rand_repeatable() {
        const MY_SEED: [u8; 32] = [
            123, 164, 185,  95, 103, 243,  38, 140,
            133,  27,  36, 178, 255, 156,  87, 155,
            130,  52,  56, 167, 183,  98,    6, 242,
            214,  42,  82, 202, 230, 246,  83, 234];
        fn random_from_seed() -> f64 {
            let mut rng = StdRng::from_seed(MY_SEED);
            rng.gen::<f64>()
        }
        assert_eq!(random_from_seed(), random_from_seed());
    }

    lazy_static! {
        static ref MY_VEC: Vec<&'static str> = vec!["hello", "world"];
    }

    #[test]
    fn lazy_static_vec() {
        assert_eq!("hello", MY_VEC[0]);
    }
}
