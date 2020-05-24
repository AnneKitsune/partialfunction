extern crate partial_function;

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use partial_function::*;
    #[test]
    fn single() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .build();
        assert_eq!(Some(0.5), p.eval(0.5));
    }
    #[test]
    fn single_start() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .build();
        assert_eq!(Some(0.0), p.eval(0.0));
    }
    #[test]
    fn single_ending() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .build();
        assert_eq!(Some(1.0), p.eval(1.0));
    }
    #[test]
    fn single_nan() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .build();
        assert!(p.eval(999.0).is_none());
    }
    #[test]
    fn dual_start() {
        let p = PartialFunction::new()
            .with(1.0, 2.0, Box::new(|x| 5.0))
            .with(0.0, 1.0, Box::new(|x| x))
            .build();
        assert_eq!(Some(5.0), p.eval(1.0));
    }
    #[test]
    fn dual_end() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(1.0, 2.0, Box::new(|x| 5.0))
            .build();
        assert_eq!(Some(5.0), p.eval(1.0));
    }
    #[test]
    #[should_panic]
    fn intersect_start() {
        PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(-0.5, 0.5, Box::new(|x| 5.0))
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_end() {
        PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(0.5, 2.0, Box::new(|x| 5.0))
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_inner() {
        PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(0.4, 0.6, Box::new(|x| 5.0))
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_outer() {
        PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(-2.0, 2.0, Box::new(|x| 5.0))
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_same() {
        PartialFunction::new()
            .with(0.0, 1.0, Box::new(|x| x))
            .with(0.0, 1.0, Box::new(|x| 5.0))
            .build();
    }

    #[test]
    fn lower_partial_normal() {
        let f = LowerPartialFunction::new()
            .with(0.0, Box::new(|x| 1))
            .with(1.0, Box::new(|x| 2))
            .build();
        assert_eq!(f.eval(-1.0), None);
        assert_eq!(f.eval(0.0), Some(1));
        assert_eq!(f.eval(0.5), Some(1));
        assert_eq!(f.eval(1.0), Some(2));
        assert_eq!(f.eval(1000.0), Some(2));
    }

    #[test]
    fn lower_partial_inverse_insert() {
        let f = LowerPartialFunction::new()
            .with(1.0, Box::new(|x| 2))
            .with(0.0, Box::new(|x| 1))
            .build();
        assert_eq!(f.eval(-1.0), None);
        assert_eq!(f.eval(0.0), Some(1));
        assert_eq!(f.eval(0.5), Some(1));
        assert_eq!(f.eval(1.0), Some(2));
        assert_eq!(f.eval(1000.0), Some(2));
    }

    #[test]
    #[should_panic]
    fn lower_partial_overlap() {
        let f = LowerPartialFunction::new()
            .with(0.0, Box::new(|x| 1))
            .with(0.0, Box::new(|x| 2))
            .build();
    }
}
