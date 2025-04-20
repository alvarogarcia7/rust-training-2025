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
        let mut carry = false;
        for (i, item) in values.iter_mut().enumerate() {
            let mut carry_from_first = false;
            let carry_from_second;
            if carry {
                (*item, carry_from_first) = item.overflowing_add(1);
            }
            (*item, carry_from_second) = item.overflowing_add(_rhs.values[i]);
            carry = carry_from_first || carry_from_second;
        }
        if carry {
            panic!("attempt to add with overflow")
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

impl From<Vec<u64>> for BigUint4096 {
    fn from(value: Vec<u64>) -> Self {
        let mut values = [0; 64];
        values[..value.len()].copy_from_slice(&value[..]);
        // for i in 0..value.len() {
        //     values[i] = value[i];
        // }
        Self { values }
    }
}
// impl From<&[u64]> for BigUint4096 {
//     fn from(value: &[u64]) -> Self {
//         let mut values = [0; 64];
//         // values[..value.len()].copy_from_slice(&value[..]);
//         // for i in 0..value.len() {
//         //     values[i] = value[i];
//         // }
//         Self { values }
//     }
// }
//
// impl BigUint4096 {
//     // pub const MAX: BigUint4096 =
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_0_0() {
        assert_eq_biguint4096(0, add(0, 0));
    }

    #[test]
    fn add_0_1() {
        assert_eq_biguint4096(1, add(0, 1));
    }
    #[test]
    fn add_overflow_first_element() {
        assert_eq!(BigUint4096::from(vec![0u64, 1u64]), add(u64::MAX, 1));
    }
    #[test]
    fn add_overflow_second_element() {
        assert_eq!(
            BigUint4096::from(vec![0u64, 0u64, 1u64]),
            BigUint4096::from(vec![u64::MAX, u64::MAX]) + BigUint4096::from(vec![1u64])
        )
    }
    #[test]
    fn add_to_max_result() {
        let mut max_minus_one = vec![u64::MAX; 64];
        max_minus_one[0] -= 1;

        assert_eq!(
            BigUint4096::from(vec![u64::MAX; 64]),
            BigUint4096::from(max_minus_one) + BigUint4096::from(vec![1u64])
        )
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_max_element() {
        let _ = BigUint4096::from(vec![u64::MAX; 64]) + BigUint4096::from(vec![1u64]);
    }

    fn assert_eq_biguint4096(expected: u64, actual: BigUint4096) {
        assert_eq!(BigUint4096::from(expected), actual);
    }

    /// Create a BigUint4096 from a smaller number
    fn bu(value: u64) -> BigUint4096 {
        BigUint4096::from(value)
    }

    fn add(p0: u64, p1: u64) -> BigUint4096 {
        bu(p0) + BigUint4096::from(p1)
    }
}
