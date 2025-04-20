use std::ops;

#[derive(Debug)]
struct BigUint4096 {
    values: [u64; 64],
}

impl PartialEq for BigUint4096 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..64 {
            if self.values[i] != other.values[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for BigUint4096 {}

impl ops::Add<BigUint4096> for BigUint4096 {
    type Output = BigUint4096;

    fn add(self, _rhs: BigUint4096) -> BigUint4096 {
        let mut values = self.values;
        // for i in 0..64 {
        for (i, item) in values.iter_mut().enumerate() {
            *item += _rhs.values[i];
        }
        BigUint4096 { values }
    }
}

impl From<u64> for BigUint4096 {
    fn from(value: u64) -> Self {
        let mut values = [0; 64];
        values[0] = value;
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_0_0() {
        assert_eq_biguint4096(0, add(0,0));
    }

    fn assert_eq_biguint4096(expected: u64, actual: BigUint4096) {
        assert_eq!(BigUint4096::from(expected), actual);
    }

    fn add(p0: u64, p1: u64) -> BigUint4096 {
        BigUint4096::from(p0) + BigUint4096::from(p1)
    }

    #[test]
    fn add_0_1() {
        assert_eq_biguint4096(1, add(0, 1));
    }
}
