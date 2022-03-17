pub mod asserts {
    /// Checks whether all elements in `left` (the first collection) are contained
    /// within `right` (the second collection).
    ///
    /// ## Example(s)
    /// ```rust
    /// # use stupid_utils::assert_contains_all;
    /// let first = vec![1, 2, 3];
    /// let second = vec![5, 4, 3, 2, 1];
    /// assert_contains_all!(first, second);
    /// ```
    #[macro_export]
    macro_rules! assert_contains_all {
        ($left:expr, $right:expr $(,) ?) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    for n in left_val.iter() {
                        assert!(
                            right_val.contains(n),
                            "Expected right {:?} to contain value {:?}",
                            right_val,
                            n
                        );
                    }
                }
            }
        }};
        ($left:expr, $right:expr, $($arg:tt) +) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    for n in left_val.iter() {
                        assert!(
                            right_val.contains(n),
                            $($arg)*
                        );
                    }
                }
            }
        }};
    }

    /// Macro to check if the contents of the first argument are equal to the contents
    /// of the second, regardless of the order.
    ///
    /// ## Example(s)
    /// ```rust
    /// # use stupid_utils::assert_unordered_match;
    /// let first = vec![3, 2, 1];
    /// let second = vec![1, 2, 3];
    /// assert_unordered_match!(first, second);
    /// ```
    #[macro_export]
    macro_rules! assert_unordered_match {
        ($left:expr, $right:expr $(,) ?) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    {
                        let _lcount = left_val.iter().count();
                        let _rcount = right_val.iter().count();
                        assert_eq!(_lcount, _rcount, "Expected: size of left and right collections to be equal, Actual: left = {}, right = {}", _lcount, _rcount);
                    }
                    for n in left_val.iter() {
                        assert!(right_val.contains(n), "Expected right side collection to contain value {:?} from left side collection", n);
                    }
                }
            }
        }};
        ($left:expr, $right:expr, $($arg:tt) +) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    {
                        let _lcount = left_val.iter().count();
                        let _rcount = right_val.iter().count();
                        assert_eq!(_lcount, _rcount, $($arg)*);
                    }
                    for n in left_val.iter() {
                        assert!(right_val.contains(n), $($arg)*);
                    }
                }
            }
        }};
    }

    /// Macro to check if the `actual` value is contained in the `range` given.
    ///
    /// ## Example(s)
    /// ```rust
    /// # use stupid_utils::assert_in_range;
    /// let range = 0..10;
    /// let i = 5;
    /// assert_in_range!(i, range, "Range should contain i");
    /// ```
    #[macro_export]
    macro_rules! assert_in_range {
        ($actual:expr, $range:expr $(,) ?) => {{
            match(&$actual, &$range) {
                (actual_val, range_val) => {
                    if !(*range_val).contains(actual_val) {
                        panic!(
                            "Expected range {:?} to contain {:?}",
                            &*range_val, &*actual_val
                        );
                    }
                }
            }
        }};
        ($actual:expr, $range:expr, $($arg:tt) +) => {{
            match(&$actual, &$range) {
                (actual_val, range_val) => {
                    if !(*range_val).contains(actual_val) {
                        panic!($($arg)*);
                    }
                }
            }
        }};
    }

    /// Macro to check if the `actual` value is gte `min` and lte `max`.
    ///
    /// ## Example(s)
    /// ```rust
    /// # use stupid_utils::assert_within;
    /// let i = 5;
    /// assert_within!(i, 0, 10, "i should be within 0..10");
    /// ```
    #[macro_export]
    macro_rules! assert_within {
        ($actual:expr, $min:expr, $max:expr $(,) ?) => {{
            assert!($actual >= $min, "Expected {:?} to be gte {:?}", $actual, $min);
            assert!($actual <= $max, "Expected {:?} to be lte {:?}", $actual, $max);
        }};
        ($actual:expr, $min:expr, $max:expr, $($arg:tt) +) => {{
            assert!($actual >= $min, $($arg)*);
            assert!($actual <= $max, $($arg)*);
        }};
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn contains_all() {
            let first = vec![1, 2, 3];
            let second = vec![5, 4, 3, 2, 1];
            assert_contains_all!(first, second);
            assert_contains_all!(
                first,
                second,
                "Expected {:?} to contain {:?}",
                second,
                first
            );
        }

        #[test]
        #[should_panic]
        fn not_contains_all() {
            let first = vec![1, 2, 3];
            let second = vec![5, 4, 3, 2, 1];
            assert_contains_all!(second, first);
        }

        #[test]
        #[should_panic]
        fn not_contains_all2() {
            let first = vec![1, 2, 3];
            let second = vec![5, 4, 3, 2, 1];
            assert_contains_all!(
                second,
                first,
                "Expected {:?} to contain {:?}",
                second,
                first
            );
        }

        #[test]
        fn unordered_match() {
            let first = vec![1, 2, 3];
            let second = vec![3, 2, 1];
            assert_unordered_match!(first, second);
            assert_unordered_match!(second, first);
            assert_unordered_match!(
                first,
                second,
                "Expected {:?} to match contents of {:?}",
                second,
                first
            );
            assert_unordered_match!(
                second,
                first,
                "Expected {:?} to match contents of {:?}",
                second,
                first
            );
        }

        #[test]
        #[should_panic]
        fn not_unordered_match() {
            let first = vec![1, 2, 3];
            let second = vec![5, 4, 3];
            assert_unordered_match!(first, second);
        }

        #[test]
        #[should_panic]
        fn not_unordered_match2() {
            let first = vec![1, 2, 3];
            let second = vec![5, 4, 3];
            assert_unordered_match!(
                first,
                second,
                "Expected {:?} to match contents of {:?}",
                second,
                first
            );
        }

        #[test]
        fn unordered_match_different_types() {
            let first = vec![1, 2, 3];
            let second = [3, 2, 1];
            let third = {
                let mut set = std::collections::HashSet::new();
                assert!(set.insert(1), "Unable to add 1 to HashSet");
                assert!(set.insert(2), "Unable to add 2 to HashSet");
                assert!(set.insert(3), "Unable to add 3 to HashSet");
                set
            };
            let fourth = std::collections::LinkedList::from([2, 1, 3]);

            assert_unordered_match!(first, second);
            assert_unordered_match!(second, third);
            assert_unordered_match!(third, fourth);
            assert_unordered_match!(fourth, first);

            assert_unordered_match!(
                first,
                second,
                "Expected {:?} to match contents of {:?}",
                first,
                second
            );
            assert_unordered_match!(
                second,
                third,
                "Expected {:?} to match contents of {:?}",
                second,
                third
            );
            assert_unordered_match!(
                third,
                fourth,
                "Expected {:?} to match contents of {:?}",
                third,
                fourth
            );
            assert_unordered_match!(
                fourth,
                first,
                "Expected {:?} to match contents of {:?}",
                fourth,
                first
            );
        }

        #[test]
        fn in_range() {
            let range = 0..10;
            let i = 5;
            assert_in_range!(i, range);
            assert_in_range!(i, range, "Expected i to be in range!");
        }

        #[test]
        #[should_panic]
        fn not_in_range() {
            let range = 0..10;
            let i = 11;
            assert_in_range!(i, range);
        }

        #[test]
        #[should_panic]
        fn not_in_range2() {
            let range = 0..10;
            let i = 11;
            assert_in_range!(i, range, "whoops!");
        }

        #[test]
        fn within() {
            let i = 5;
            assert_within!(i, 0, 10);
            assert_within!(i, 0, 10, "Expected i to be in range!");
        }

        #[test]
        #[should_panic]
        fn not_within() {
            let i = 11;
            assert_within!(i, 0, 10);
        }

        #[test]
        #[should_panic]
        fn not_within2() {
            let i = 11;
            assert_within!(i, 0, 10, "whoops!");
        }
    }
}
