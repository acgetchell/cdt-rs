use rand::prelude::*;

pub fn generate_random_float() -> f64 {
    let mut rng = rand::thread_rng();

    let result: f64 = rng.gen();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_random_int() {
        let result = generate_random_float();
        assert!(result > 0.0);
    }
}
