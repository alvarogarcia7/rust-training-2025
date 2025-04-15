fn f1<'a>(a: &'a mut (u32, u32), b: bool) -> &'a mut u32 {
    if !b { &mut a.0 } else { &mut a.1 }
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
}
