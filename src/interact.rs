pub mod n2 {

    #[macro_export]
    macro_rules! foreach {
        (<$tp:ty> in $elements:expr;
            step($a:ident, $b:ident) $step:block
        ) => {{
            let cloned = ($elements).iter().map(|b| b.clone()).collect::<Vec<$tp>>();
            for element in &mut *($elements) {
                for other in &cloned {
                    (|$a: &mut $tp, $b: &$tp| $step)(element, other);
                }
            }
        }};

        (<$tp:ty> in $elements:expr;
            setup($su_a:ident) $setup:block
            step($a:ident, $b:ident) $step:block
        ) => {{
            let cloned = ($elements).iter().map(|b| b.clone()).collect::<Vec<$tp>>();
            for element in &mut *($elements) {
                (|$su_a: &mut $tp| $setup)(element);
                for other in &cloned {
                    (|$a: &mut $tp, $b: &$tp| $step)(element, other);
                }
            }
        }};

        (<$tp:ty> in $elements:expr;
            step($a:ident, $b:ident) $step:block
            teardown($td_a:ident) $teardown:block
        ) => {{
            let cloned = ($elements).iter().map(|b| b.clone()).collect::<Vec<$tp>>();
            for element in &mut *($elements) {
                for other in &cloned {
                    (|$a: &mut $tp, $b: &$tp| $step)(element, other);
                }
                (|$td_a: &mut $tp| $teardown)(element);
            }
        }};

        (<$tp:ty> in $elements:expr;
            setup($su_a:ident) $setup:block
            step($a:ident, $b:ident) $step:block
            teardown($td_a:ident) $teardown:block
        ) => {{
            let cloned = ($elements).iter().map(|b| b.clone()).collect::<Vec<$tp>>();
            for element in &mut *($elements) {
                (|$su_a: &mut $tp| $setup)(element);
                for other in &cloned {
                    (|$a: &mut $tp, $b: &$tp| $step)(element, other);
                }
                (|$td_a: &mut $tp| $teardown)(element);
            }
        }};
    }
    pub(crate) use foreach;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_only() {
        let mut res = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
        n2::foreach! {
            <(i32, i32)> in res;

            step(a, b) {
                a.1 += b.0;
            }
        }

        assert_eq!(res[0], (0, 10));
        assert_eq!(res[1], (1, 11));
        assert_eq!(res[2], (2, 12));
        assert_eq!(res[3], (3, 13));
        assert_eq!(res[4], (4, 14));
    }

    #[test]
    fn step_setup() {
        let mut res = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
        n2::foreach! {
            <(i32, i32)> in res;

            setup(v) {
                v.1 = 0;
            }

            step(a, b) {
                a.1 += b.0;
            }
        }

        assert_eq!(res[0], (0, 10));
        assert_eq!(res[1], (1, 10));
        assert_eq!(res[2], (2, 10));
        assert_eq!(res[3], (3, 10));
        assert_eq!(res[4], (4, 10));
    }

    #[test]
    fn step_teardown() {
        let mut res = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
        n2::foreach! {
            <(i32, i32)> in res;

            step(a, b) {
                a.1 += b.0;
            }

            teardown(v) {
                v.1 = 0;
            }
        }

        assert_eq!(res[0], (0, 0));
        assert_eq!(res[1], (1, 0));
        assert_eq!(res[2], (2, 0));
        assert_eq!(res[3], (3, 0));
        assert_eq!(res[4], (4, 0));
    }

    #[test]
    fn full() {
        let mut res = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
        n2::foreach! {
            <(i32, i32)> in res;

            setup(v) {
                v.1 = 0;
            }

            step(a, b) {
                a.1 += b.0;
            }

            teardown(v) {
                v.0 += 1;
            }
        }

        assert_eq!(res[0], (1, 10));
        assert_eq!(res[1], (2, 10));
        assert_eq!(res[2], (3, 10));
        assert_eq!(res[3], (4, 10));
        assert_eq!(res[4], (5, 10));
    }
}
