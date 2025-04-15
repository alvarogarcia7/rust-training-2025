fn f1(a: &mut (u32, u32), b: bool) -> &mut u32 {
    if !b { &mut a.0 } else { &mut a.1 }
}

fn f2(a: &mut [u32], n: usize) -> &mut u32 {
    &mut a[n]
}

fn f3(a: &mut [u32], n: usize) -> &mut u32 {
    &mut a[a.len() - 1 - n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f1_true() {
        let i0 = 0;
        let i1 = 1;
        let tuple = &mut (i0, i1);

        let result = f1(tuple, true);

        assert_eq!(*result, 1);
    }
    #[test]
    fn f1_false() {
        let i0 = 0;
        let i1 = 1;
        let tuple = &mut (i0, i1);

        let result = f1(tuple, false);

        assert_eq!(*result, 0);
    }
    #[test]
    #[allow(clippy::useless_vec)]
    fn f2_0() {
        let mut input = vec![0, 1, 2, 3];
        let slice = &mut input[0..=3];

        let result = f2(slice, 0);

        assert_eq!(*result, 0);
    }
    #[test]
    #[allow(clippy::useless_vec)]
    fn f2_2() {
        let mut input = vec![0, 1, 2, 3];
        let slice = &mut input[0..=3];

        let result = f2(slice, 2);

        assert_eq!(*result, 2);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn f3_0() {
        let mut vec1 = vec![0, 1, 2, 3];
        vec1.reverse();
        let mut input = vec1;
        let slice = &mut input[0..=3];

        let result = f3(slice, 0);

        assert_eq!(*result, 0);
    }
    #[test]
    #[allow(clippy::useless_vec)]
    fn f3_2() {
        let mut vec1 = vec![0, 1, 2, 3];
        vec1.reverse();
        let mut input = vec1;
        let slice = &mut input[0..=3];

        let result = f3(slice, 2);

        assert_eq!(*result, 2);
    }
}
