mod bad_lifetimes;
fn f1(a: &mut (u32, u32), b: bool) -> &mut u32 {
    if !b { &mut a.0 } else { &mut a.1 }
}

fn f2(a: &mut [u32], n: usize) -> &mut u32 {
    &mut a[n]
}

fn f3(a: &mut [u32], n: usize) -> &mut u32 {
    &mut a[a.len() - 1 - n]
}

fn f4(a: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let increments = match a.len() % 4 {
        0 => [0, 0, 0, 0],
        1 => [1, 0, 0, 0],
        2 => [1, 0, 1, 0],
        3 => [1, 0, 1, 1],
        _ => unreachable!(),
    };

    let remaining = a.len() - a.len() % 4;

    let elements = (1..=4).map(|_| remaining / 4).collect::<Vec<_>>();

    let mut result = [0, 0, 0, 0];

    for (i, (a, b)) in elements.iter().zip(&increments).enumerate() {
        result[i] = a + b;
    }

    (
        &a[0..result[0]],
        &a[result[0]..result[0] + result[1]],
        &a[result[0] + result[1]..result[0] + result[1] + result[2]],
        &a[result[0] + result[1] + result[2]..result[0] + result[1] + result[2] + result[3]],
    )
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
    fn f2_0() {
        let mut vec1 = vec![0, 1, 2, 3];
        let slice = vec1.as_mut_slice();

        let result = f2(slice, 0);

        assert_eq!(*result, 0);
    }
    #[test]
    fn f2_2() {
        let mut vec1 = vec![0, 1, 2, 3];
        let slice = vec1.as_mut_slice();

        let result = f2(slice, 2);

        assert_eq!(*result, 2);
    }

    #[test]
    fn f3_0() {
        let mut vec1 = vec![0, 1, 2, 3];
        vec1.reverse();
        let vec1 = vec1.as_mut_slice();
        let slice = vec1;

        let result = f3(slice, 0);

        assert_eq!(*result, 0);
    }
    #[test]
    #[allow(clippy::useless_vec)]
    fn f3_2() {
        let mut vec1 = vec![0, 1, 2, 3];
        vec1.reverse();
        let slice = &mut vec1[0..=3];

        let result = f3(slice, 2);

        assert_eq!(*result, 2);
    }

    #[test]
    fn f4_equal_parts() {
        let vec1 = vec![0, 1, 2, 3];
        let slice = vec1.as_slice();

        let (slice1, slice2, slice3, slice4) = f4(slice);

        assert_eq!(slice1, [0].as_slice());
        assert_eq!(slice2, [1].as_slice());
        assert_eq!(slice3, [2].as_slice());
        assert_eq!(slice4, [3].as_slice());
    }
    #[test]
    fn f4_smaller_than_4() {
        let vec1 = vec![0, 1];
        let slice = vec1.as_slice();

        let (slice1, slice2, slice3, slice4) = f4(slice);

        assert_eq!(slice1, [0].as_slice());
        assert_eq!(slice2, [].as_slice());
        assert_eq!(slice3, [1].as_slice());
        assert_eq!(slice4, [].as_slice());
    }

    #[test]
    fn f4_smaller_than_6() {
        let vec1: Vec<u32> = (0..6).collect();
        let slice = vec1.as_slice();

        let (slice1, slice2, slice3, slice4) = f4(slice);

        assert_eq!(slice1, [0, 1].as_slice());
        assert_eq!(slice2, [2].as_slice());
        assert_eq!(slice3, [3, 4].as_slice());
        assert_eq!(slice4, [5].as_slice());
    }
}
