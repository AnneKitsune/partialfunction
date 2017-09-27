extern crate partial_function;

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use partial_function::*;
    #[test]
    fn single() {
        let p = PartialFunction::new().with(0.0, 1.0, |x| x).build();
        assert_eq!(0.5, p.eval(0.5));
    }
    #[test]
    fn single_start() {
        let p = PartialFunction::new().with(0.0, 1.0, |x| x).build();
        assert_eq!(0.0, p.eval(0.0));
    }
    #[test]
    fn single_ending() {
        let p = PartialFunction::new().with(0.0, 1.0, |x| x).build();
        assert_eq!(1.0, p.eval(1.0));
    }
    #[test]
    fn single_nan() {
        let p = PartialFunction::new().with(0.0, 1.0, |x| x).build();
        assert!(p.eval(999.0).is_nan());
    }
    #[test]
    fn dual_start() {
        let p = PartialFunction::new()
            .with(1.0, 2.0, |x| 5.0)
            .with(0.0, 1.0, |x| x)
            .build();
        assert_eq!(5.0, p.eval(1.0));
    }
    #[test]
    fn dual_end() {
        let p = PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(1.0, 2.0, |x| 5.0)
            .build();
        assert_eq!(5.0, p.eval(1.0));
    }
    #[test]
    #[should_panic]
    fn intersect_start() {
        PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(-0.5, 0.5, |x| 5.0)
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_end() {
        PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(0.5, 2.0, |x| 5.0)
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_inner() {
        PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(0.4, 0.6, |x| 5.0)
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_outer() {
        PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(-2.0, 2.0, |x| 5.0)
            .build();
    }
    #[test]
    #[should_panic]
    fn intersect_same() {
        PartialFunction::new()
            .with(0.0, 1.0, |x| x)
            .with(0.0, 1.0, |x| 5.0)
            .build();
    }
}
