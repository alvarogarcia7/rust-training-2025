use std::ops;

#[derive(Debug, Hash, PartialEq)]
pub struct BigUint4096 {
    values: [u64; 64],
}

impl Default for BigUint4096 {
    fn default() -> Self {
        Self { values: [0; 64] }
    }
}

impl ops::Add<BigUint4096> for BigUint4096 {
    type Output = BigUint4096;

    fn add(self, _rhs: BigUint4096) -> BigUint4096 {
        &self + &_rhs
    }
}
impl ops::Add<&BigUint4096> for &BigUint4096 {
    type Output = BigUint4096;

    fn add(self, _rhs: &BigUint4096) -> BigUint4096 {
        let mut values = self.values;
        // for i in 0..64 {
        let mut carry = false;
        for (i, item) in values.iter_mut().enumerate() {
            let mut carry_from_first = false;
            if carry {
                (*item, carry_from_first) = item.overflowing_add(1);
            }
            let carry_from_second;
            (*item, carry_from_second) = item.overflowing_add(_rhs.values[i]);
            carry = carry_from_first || carry_from_second;
        }
        if carry {
            panic!("attempt to add with overflow")
        }
        BigUint4096 { values }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromUint4096Error(pub(crate) ());

impl TryFrom<u64> for BigUint4096 {
    type Error = TryFromUint4096Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let mut values = [0; 64];
        values[0] = value;
        Ok(Self { values })
    }
}

impl TryFrom<Vec<u64>> for BigUint4096 {
    type Error = TryFromUint4096Error;
    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        if value.len() > 64 || value.is_empty() {
            return Err(TryFromUint4096Error(()));
        }
        let mut values = [0; 64];
        values[..value.len()].copy_from_slice(&value[..]);
        // for i in 0..value.len() {
        //     values[i] = value[i];
        // }
        Ok(Self { values })
    }
}
impl TryFrom<String> for BigUint4096 {
    type Error = TryFromUint4096Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 1024 || value.is_empty() || value.len() % 2 != 0 {
            return Err(TryFromUint4096Error(()));
        }
        let mut bs = (0..value.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&value[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();
        bs.reverse();

        let mut values = [0u64; 64];
        let mut i = 0;
        let mut j = 0;
        for byte in bs.iter() {
            println!("{:?}", i);
            values[j] += *byte as u64 >> (i * 8);
            i += 1;
            if i == 8 {
                j += 1;
                i = 0;
            }
        }

        Ok(Self { values })
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
    fn cannot_try_from_an_array_too_big() {
        let too_big_input = vec![0; 65];

        assert!(BigUint4096::try_from(too_big_input).is_err());
    }

    #[test]
    fn try_from_maximum_array_size() {
        assert!(BigUint4096::try_from(vec![0; 64]).is_ok());
    }

    #[test]
    fn try_from_minimum_array_size() {
        assert!(BigUint4096::try_from(vec![0; 1]).is_ok());
    }
    #[test]
    fn cannot_try_from_with_array_size_zero() {
        assert!(BigUint4096::try_from(vec![0; 0]).is_err());
    }

    #[test]
    fn try_from_string_one_byte() {
        assert_eq!(
            BigUint4096::try_from("00".to_string()).unwrap(),
            BigUint4096::try_from(vec![0u64; 64]).unwrap()
        );
    }

    #[test]
    fn try_from_string_two_bytes() {
        assert_eq!(
            BigUint4096::try_from("0000".to_string()).unwrap(),
            BigUint4096::try_from(vec![0u64; 64]).unwrap()
        );
    }
    #[test]
    fn try_from_string_one_with_padding_4_char() {
        let mut one = vec![0u64; 64];
        one[0] = 1;
        assert_eq!(
            BigUint4096::try_from("0001".to_string()).unwrap(),
            BigUint4096::try_from(one).unwrap()
        );
    }
    #[test]
    fn try_from_string_one_with_padding_9_char() {
        let mut one = vec![0u64; 64];
        one[0] = 1;
        assert_eq!(
            BigUint4096::try_from("00000001".to_string()).unwrap(),
            BigUint4096::try_from(one).unwrap()
        );
    }
    #[test]
    fn try_from_string_two_u64_digits() {
        let mut expected = vec![0u64; 64];
        expected[0] = 1;
        expected[1] = 1;
        assert_eq!(
            BigUint4096::try_from("010000000000000001".to_string()).unwrap(),
            BigUint4096::try_from(expected).unwrap()
        );
    }

    #[test]
    fn try_from_string_three_u64_digits() {
        let mut expected = vec![0u64; 64];
        expected[0] = 1;
        expected[1] = 1;
        expected[2] = 1;
        assert_eq!(
            BigUint4096::try_from("0100000000000000010000000000000001".to_string()).unwrap(),
            BigUint4096::try_from(expected).unwrap()
        );
    }

    #[test]
    fn add_0_0() {
        assert_eq_biguint4096(0, add(0, 0));
    }

    #[test]
    fn add_references_0_0() {
        assert_eq_biguint4096(
            0,
            &BigUint4096::try_from(0).unwrap() + &BigUint4096::try_from(0).unwrap(),
        );
    }

    #[test]
    fn add_0_1() {
        assert_eq_biguint4096(1, add(0, 1));
    }

    #[test]
    fn add_references_0_1() {
        assert_eq_biguint4096(
            1,
            &BigUint4096::try_from(0).unwrap() + &BigUint4096::try_from(1).unwrap(),
        );
    }
    #[test]
    fn add_overflow_first_element() {
        assert_eq!(
            BigUint4096::try_from(vec![0u64, 1u64]).unwrap(),
            add(u64::MAX, 1)
        );
    }
    #[test]
    fn add_overflow_second_element() {
        assert_eq!(
            BigUint4096::try_from(vec![0u64, 0u64, 1u64]).unwrap(),
            BigUint4096::try_from(vec![u64::MAX, u64::MAX]).unwrap()
                + BigUint4096::try_from(vec![1u64]).unwrap()
        )
    }
    #[test]
    fn add_to_max_result() {
        let mut max_minus_one = vec![u64::MAX; 64];
        max_minus_one[0] -= 1;

        assert_eq!(
            BigUint4096::try_from(vec![u64::MAX; 64]).unwrap(),
            BigUint4096::try_from(max_minus_one).unwrap()
                + BigUint4096::try_from(vec![1u64]).unwrap()
        )
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_max_element() {
        let _ = BigUint4096::try_from(vec![u64::MAX; 64]).unwrap()
            + BigUint4096::try_from(vec![1u64]).unwrap();
    }

    fn assert_eq_biguint4096(expected: u64, actual: BigUint4096) {
        assert_eq!(BigUint4096::try_from(expected).unwrap(), actual);
    }

    fn add(p0: u64, p1: u64) -> BigUint4096 {
        BigUint4096::try_from(p0).unwrap() + BigUint4096::try_from(p1).unwrap()
    }
}
