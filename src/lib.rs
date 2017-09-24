pub mod mod_exp {
    extern crate num;

    use self::num::bigint::{BigUint};

    // Modular exponentiation by squaring
    pub fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        let zero = BigUint::new(vec![0]);
        let one = BigUint::new(vec![1]);
        let two = BigUint::new(vec![2]);
        let mut exp = exponent.clone();
        let mut result = one.clone();
        let mut base = base % modulus;
        while exp > zero {
            if &exp % &two == one {
                result = (result * &base) % modulus;
            }
            exp = exp >> 1;
            base = (&base * &base) % modulus;
        }
        result
    }
}
