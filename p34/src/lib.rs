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
        assert!(BigUint4096::from(0).eq(&(BigUint4096::from(0u64) + BigUint4096::from(0u64))));
    }
}
