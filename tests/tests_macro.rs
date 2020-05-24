extern crate partial_function;

// All these tests are the same as those in
// tests.rs, the only difference is that the
// macro version is used.

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use partial_function::*;
    #[test]
    fn single() {
        let p = partfn! {
            [0.0, 1.0]: x -> x,
        };
        assert_eq!(Some(0.5), p.eval(0.5));
    }
    #[test]
    fn single_start() {
        let p = partfn! {
            [0.0, 1.0]: x -> x,
        };
        assert_eq!(Some(0.0), p.eval(0.0));
    }
    #[test]
    fn single_ending() {
        let p = partfn! {
            [0.0, 1.0]: x -> x,
        };
        assert_eq!(Some(1.0), p.eval(1.0));
    }
    #[test]
    fn single_nan() {
        let p = partfn! {
            [0.0, 1.0]: x -> x,
        };
        assert!(p.eval(999.0).is_none());
    }
    #[test]
    fn dual_start() {
        let p = partfn! {
            [1.0, 2.0]: x -> 5.0,
            [0.0, 1.0]: x -> x,
        };
        assert_eq!(Some(5.0), p.eval(1.0));
    }
    #[test]
    fn dual_end() {
        let p = partfn! {
            [0.0, 1.0]: x -> x,
            [1.0, 2.0]: x -> 5.0,
        };
        assert_eq!(Some(5.0), p.eval(1.0));
    }
    #[test]
    #[should_panic]
    fn intersect_start() {
        partfn! {
            [0.0, 1.0]: x -> x,
            [-0.5, 0.5]: x -> 5.0,
        };
    }
    #[test]
    #[should_panic]
    fn intersect_end() {
        partfn! {
            [0.0, 1.0]: x -> x,
            [0.5, 2.0]: x -> 5.0,
        };
    }
    #[test]
    #[should_panic]
    fn intersect_inner() {
        partfn! {
            [0.0, 1.0]: x -> x,
            [0.4, 0.6]: x -> 5.0,
        };
    }
    #[test]
    #[should_panic]
    fn intersect_outer() {
        partfn! {
            [0.0, 1.0]: x -> x,
            [-2.0, 2.0]: x -> 5.0,
        };
    }
    #[test]
    #[should_panic]
    fn intersect_same() {
        partfn! {
            [0.0, 1.0]: x -> x,
            [0.0, 1.0]: x -> 5.0,
        };
    }

    #[test]
    fn lower_partial_normal() {
        let f = lowpartfn! {
            [0.0]: x -> 1,
            [1.0]: x -> 2,
        };
        assert_eq!(f.eval(-1.0), None);
        assert_eq!(f.eval(0.0), Some(1));
        assert_eq!(f.eval(0.5), Some(1));
        assert_eq!(f.eval(1.0), Some(2));
        assert_eq!(f.eval(1000.0), Some(2));
    }

    #[test]
    fn lower_partial_inverse_insert() {
        let f = lowpartfn! {
            [1.0]: x -> 2,
            [0.0]: x -> 1,
        };
        assert_eq!(f.eval(-1.0), None);
        assert_eq!(f.eval(0.0), Some(1));
        assert_eq!(f.eval(0.5), Some(1));
        assert_eq!(f.eval(1.0), Some(2));
        assert_eq!(f.eval(1000.0), Some(2));
    }

    #[test]
    #[should_panic]
    fn lower_partial_overlap() {
        let f = lowpartfn! {
            [0.0]: x -> 1,
            [0.0]: x -> 2,
        };
    }
}
