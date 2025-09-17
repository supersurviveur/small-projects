mod tests {
    use virtual_float::{integer::Integer, natural::Natural};

    #[test]
    fn test_shl() {
        let a = Integer::from(1u8);
        assert_eq!(a.clone() << 1, Integer::from(2u64));
        assert_eq!(a.clone() << 10, Integer::from(1024u64));

        let a = Integer::from(-1i8);
        assert_eq!(a.clone() << 1, Integer::from(-2i64));
        assert_eq!(a.clone() << 10, Integer::from(-1024i64));
    }

    #[test]
    fn test_shr() {
        let b = Integer::from(64u8);
        assert_eq!(b.clone() >> 1, Integer::from(32u8));
        assert_eq!(b.clone() >> 6, Integer::from(1u8));
        assert_eq!(b.clone() >> 7, Integer::from(0u8));

        let b = Integer::from(-64i8);
        assert_eq!(b.clone() >> 1, Integer::from(-32i8));
        assert_eq!(b.clone() >> 6, Integer::from(-1i8));
        assert_eq!(b.clone() >> 7, Integer::from(-0i8));

        let b = Integer::from(65i8);
        assert_eq!(b.clone() >> 1, Integer::from(32i8));

        let b = Integer::from(-65i8);
        assert_eq!(b.clone() >> 1, Integer::from(-64i8));
    }

    #[test]
    fn test_add() {
        let a = Integer::from(1i64);
        let b = Integer::from(1i64);
        assert_eq!(a + b, Integer::from(2i64));

        let a = Integer::from(-1i64);
        let b = Integer::from(1i64);
        assert_eq!(a + b, Integer::from(0i64));

        let a = Integer::from(1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a + b, Integer::from(0i64));

        let a = Integer::from(1i64);
        let b = Integer::from(-2i64);
        assert_eq!(a + b, Integer::from(-1i64));

        let a = Integer::from(2i64);
        let b = Integer::from(-1i64);
        assert_eq!(a + b, Integer::from(1i64));

        let a = Integer::from(-1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a + b, Integer::from(-2i64));
    }

    #[test]
    fn test_sub() {
        let a = Integer::from(1i64);
        let b = Integer::from(1i64);
        assert_eq!(a - b, Integer::from(0i64));

        let a = Integer::from(-1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a - b, Integer::from(0i64));

        let a = Integer::from(1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a - b, Integer::from(2i64));

        let a = Integer::from(-1i64);
        let b = Integer::from(1i64);
        assert_eq!(a - b, Integer::from(-2i64));

        let a = Integer::from(3i64);
        let b = Integer::from(5i64);
        assert_eq!(a - b, Integer::from(-2i64));
    }

    #[test]
    fn test_cmp() {
        let a = Integer::from(0u64);
        let b = Integer::from(-0i64);
        assert!(a == b);
        assert!(a <= b);
        assert!(a >= b);

        let a = Integer::from(1u64);
        let b = Integer::from(1u64);
        assert!(a == b);
        assert!(a <= b);
        assert!(a >= b);

        let a = Integer::from(1u64);
        let b = Integer::from(-2i64);
        assert!(a > b);

        let a = Integer::from(-1i64);
        let b = Integer::from(-2i64);
        assert!(a > b);

        let a = Integer::from(-2i64);
        let b = Integer::from(-2i64);
        assert!(a == b);
        assert!(a <= b);
        assert!(a >= b);

        let a = Integer::from(-1i64);
        let b = Integer::from(2u64);
        assert!(a < b);
    }

    #[test]
    fn test_quo_rem() {
        let a = Integer::from(-1i64);
        let b = Integer::from(1i64);
        assert_eq!(a.quot_rem(b), ((-1i8).into(), 0u8.into()));

        let a = Integer::from(1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a.quot_rem(b), ((-1i8).into(), 0u8.into()));

        let a = Integer::from(-1i64);
        let b = Integer::from(-1i64);
        assert_eq!(a.quot_rem(b), ((1i8).into(), 0u8.into()));

        let a = Integer::from(-5i64);
        let b = Integer::from(2i64);
        assert_eq!(a.quot_rem(b), ((-3i8).into(), 1u8.into()));

        let a = Integer::from(5i64);
        let b = Integer::from(-2i64);
        assert_eq!(a.quot_rem(b), ((-2i8).into(), 1u8.into()));

        let a = Integer::from(-5i64);
        let b = Integer::from(-2i64);
        assert_eq!(a.quot_rem(b), ((3i8).into(), 1u8.into()));
    }

    #[test]
    fn test_mul() {
        let a = Natural::from(1u64);
        let b = Natural::from(1u64);
        assert_eq!(a * b, 1u8.into());

        let a = Natural::from(10u64);
        let b = Natural::from(10u64);
        assert_eq!(a * b, 100u8.into());

        let a = Natural::from(u64::MAX);
        let b = Natural::from(2u8);
        assert_eq!(a.clone() * b, a << 1);

        let a = Natural::from(u64::MAX);
        let b = Natural::from(0u8);
        assert_eq!(a * b, 0u8.into());

        let a = Natural::from(u64::MAX);
        let b = Natural::from(u64::MAX);
        assert_eq!(a * b, Natural::new(vec![1, !1]));

        let a = Natural::from(0u8);
        let b = Natural::from(0u8);
        assert_eq!(a * b, 0u8.into());
    }

    #[test]
    fn test_parsing() {
        let a: Result<Integer, _> = "0".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Integer::zero());

        let a: Result<Integer, _> = "-0".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Integer::zero());

        let a: Result<Integer, _> = "1".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Integer::one());

        let a: Result<Integer, _> = "-1".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Integer::new(true, Natural::one()));
    }
}
