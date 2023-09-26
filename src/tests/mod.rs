mod tests {
    use crate::mutator::timestamp::TimeStamp;
    use crate::utils::defaults::DEFAULT_FPS;

    // timestamp tests
    #[test]
    fn test_timestamp_incrementer() {
        let mut ts = TimeStamp::new(1, 3, 2);
        for _n in 0..DEFAULT_FPS + 1{
            ts.increment();
        }
        assert_eq!(ts, TimeStamp::new(1, 4, 2));
    }

    #[test]
    fn test_timestamp_minute_rollover() {
        let mut ts = TimeStamp::new(1, 59, DEFAULT_FPS);
        ts.increment();
        assert_eq!(ts, TimeStamp::new(2, 0, 0));
    }

    #[test]
    fn test_timestamp_lt() {
        let ts_less = TimeStamp::new(1, 3, 2);
        let ts_more = TimeStamp::new(3, 0, 4);
        assert!(ts_less < ts_more);
    }

    #[test]
    fn test_timestamp_gt() {
        let ts_less = TimeStamp::new(1, 3, 2);
        let ts_more = TimeStamp::new(3, 0, 4);
        assert!(ts_more > ts_less);
    }

    #[test]
    fn test_timestamp_eq() {
        let ts_one = TimeStamp::new(1, 0, 0);
        let ts_two = TimeStamp::new(1, 0, 0);
        assert_eq!(ts_one, ts_two);
    }

    #[test]
    fn test_timestamp_leq() {
        let ts_less = TimeStamp::new(1, 3, 2);
        let ts_more = TimeStamp::new(3, 0, 4);
        assert!(ts_less <= ts_more);
    }

    #[test]
    fn test_timestamp_geq() {
        let ts_less = TimeStamp::new(1, 3, 2);
        let ts_more = TimeStamp::new(3, 0, 4);
        assert!(ts_more >= ts_less);
    }

    #[test]
    fn test_timestamp_array() {
        assert_eq!(TimeStamp::new(1, 3, 2).time_as_array(), [1, 3, 2]);
    }
}