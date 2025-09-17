mod tests {
    use virtual_float::natural::Natural;

    #[test]
    fn test_shl() {
        let a = Natural::from(1u8);
        assert_eq!(a.clone() << 1, Natural::from(2u64));
        assert_eq!(a.clone() << 10, Natural::from(1024u64));
        assert_eq!(a.clone() << 64, Natural::new(vec![0, 1]));
        assert_eq!(a.clone() << 65, Natural::new(vec![0, 2]));

        let a = Natural::from(u64::MAX);
        assert_eq!(a.clone() << 1, Natural::new(vec![u64::MAX ^ 1, 1]));
        assert_eq!(a.clone() << 64, Natural::new(vec![0, u64::MAX]));
    }

    #[test]
    fn test_shr() {
        let a = Natural::from(2u8);
        assert_eq!(a.clone() >> 1, Natural::from(1u64));
        let b = Natural::new(vec![0, 1]);
        assert_eq!(b.clone() >> 1, Natural::from(1u64 << 63));
        assert_eq!(b.clone() >> 64, Natural::from(1u64));
        assert_eq!(b.clone() >> 65, Natural::zero());
    }

    #[test]
    fn test_add() {
        let a = Natural::from(1u64);
        let b = Natural::from(1u64);
        assert_eq!(a + b, Natural::from(2u64));

        let a = Natural::from(u64::MAX);
        let b = Natural::from(1u64);
        assert_eq!(a + b, Natural::new(vec![0, 1]));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![0, 1]);
        assert_eq!(a + b, Natural::new(vec![0, 2]));
        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![1]);
        assert_eq!(a + b, Natural::new(vec![1, 1]));
    }

    #[test]
    fn test_sub() {
        let a = Natural::from(1u64);
        let b = Natural::from(1u64);
        assert_eq!(a - b, Natural::from(0u64));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::from(1u64);
        assert_eq!(a - b, Natural::from(u64::MAX));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::from(u64::MAX);
        assert_eq!(a - b, Natural::from(1u8));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![0]);
        assert_eq!(a - b, Natural::new(vec![0, 1]));

        let a = Natural::new(vec![0, 2]);
        let b = Natural::new(vec![0, 1]);
        assert_eq!(a - b, Natural::new(vec![0, 1]));
    }

    #[test]
    fn test_cmp() {
        let a = Natural::from(1u64);
        let b = Natural::from(1u64);
        assert!(a == b);
        assert!(a <= b);
        assert!(a >= b);

        let a = Natural::from(1u64);
        let b = Natural::from(2u64);
        assert!(a < b);

        let a = Natural::from(1u64);
        let b = Natural::from(2u64);
        assert!(b > a);

        let a = Natural::new(vec![0, 1]);
        let b = Natural::from(2u64);
        assert!(a > b);

        let a = Natural::new(vec![0, 1]);
        let b = Natural::from(u64::MAX);
        assert!(a > b);

        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![0, 2]);
        assert!(a < b);

        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![0, 1]);
        assert!(a == b);
    }

    #[test]
    fn test_quo_rem() {
        let a = Natural::from(1u64);
        let b = Natural::from(1u64);
        assert_eq!(a.quot_rem(b), (1u8.into(), 0u8.into()));

        let a = Natural::from(0u64);
        let b = Natural::from(1u64);
        assert_eq!(a.quot_rem(b), (0u8.into(), 0u8.into()));

        let a = Natural::from(22u64);
        let b = Natural::from(7u64);
        assert_eq!(a.quot_rem(b), (3u8.into(), 1u8.into()));

        let a = Natural::from(20000u64);
        let b = Natural::from(200u64);
        assert_eq!(a.quot_rem(b), (100u8.into(), 0u8.into()));

        let a = Natural::from(100u64);
        let b = Natural::from(200u64);
        assert_eq!(a.quot_rem(b), (0u8.into(), 100u8.into()));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::new(vec![0, 1]);
        assert_eq!(a.quot_rem(b), (1u8.into(), 0u8.into()));

        let a = Natural::new(vec![0, 1]);
        let b = Natural::from(u64::MAX);
        assert_eq!(a.quot_rem(b), (1u8.into(), 1u8.into()));
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
        let a: Result<Natural, _> = "0".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Natural::zero());

        let a: Result<Natural, _> = "0000000".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Natural::zero());

        let a: Result<Natural, _> = "1".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Natural::one());

        let a: Result<Natural, _> = "0000001".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Natural::one());

        let a: Result<Natural, _> = "18446744073709551615".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), u64::MAX.into());

        let a: Result<Natural, _> = "18446744073709551616".parse();
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), Natural::new(vec![0, 1]));
    }
}
