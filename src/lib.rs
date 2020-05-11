#[macro_use]
extern crate derive_new;

use std::cmp::Ordering;

/// A regular function that is only defined between lower and higher.
/// If two functions intersect their higher and lower bounds respectively.
/// The second will take precedence where f(lower).
#[derive(new)]
pub struct DualBoundedFunction<B, O> {
    /// The stored function f(x) = ???
    pub func: Box<dyn Fn(B) -> O>,
    /// The lower bound of the function.
    pub lower: B,
    /// The higher bound of the function.
    pub higher: B,
}

/// Define a functions defined by multiple functions parts.
/// See BoundedFunction.
/// Uses bounds as [lower,higher],
/// except in the case of a lower bound overlapping a higher bound.
/// In this case, the lower bound always take precedence.
pub struct PartialFunction<B, O> {
    funcs: Vec<DualBoundedFunction<B, O>>,
}

impl<B: PartialOrd, O> PartialFunction<B, O> {
    /// Creates a new PartialFunctionBuilder
    pub fn new() -> PartialFunctionBuilder<B, O> {
        PartialFunctionBuilder::new()
    }

    /// Evaluates the partial function.
    /// Returns None if no function is defined.
    pub fn eval(&self, x: B) -> Option<O> {
        let iter = self.funcs.iter().enumerate();
        for (i, bounded) in iter {
            let next = self.funcs.get(i + 1);
            if (x >= bounded.lower && x < bounded.higher)
                || (next.is_none() && x == bounded.higher)
                || (next.is_some() && next.unwrap().lower != bounded.higher)
            {
                let f = &bounded.func;
                return Some(f(x));
            }
        }
        None
    }
}

/// A builder to create an immutable PartialFunction.
#[derive(new)]
pub struct PartialFunctionBuilder<B, O> {
    #[new(default)]
    funcs: Vec<DualBoundedFunction<B, O>>,
}

impl<B: PartialOrd, O> PartialFunctionBuilder<B, O> {
    /// Adds a bounded function bounded between [lower,higher[ of function func.
    pub fn with(mut self, lower: B, higher: B, func: Box<dyn Fn(B) -> O>) -> Self {
        debug_assert!(self.can_insert(&lower, &higher));
        let f = DualBoundedFunction {
            func: func,
            lower: lower,
            higher: higher,
        };
        self.funcs.push(f);
        self
    }

    /// Check if you can safely insert into the function list for the specified bounds.
    pub fn can_insert(&self, lower: &B, higher: &B) -> bool {
        !self.funcs.iter().any(|b| {
            (lower >= &b.lower && lower < &b.higher)
                || (higher > &b.lower && higher <= &b.higher)
                || (lower <= &b.lower && higher >= &b.higher)
        })
    }

    /// Builds the PartialFunction from the functions added using with.
    pub fn build(mut self) -> PartialFunction<B, O> {
        self.funcs.sort_by(|a, b| {
            a.lower
                .partial_cmp(&b.lower)
                .unwrap_or(a.higher.partial_cmp(&b.higher).unwrap_or(Ordering::Equal))
        });
        PartialFunction { funcs: self.funcs }
    }
}

/// A lower bounded function is a function that is valid from [x..infinite[, or until it hits another function's start.
#[derive(new)]
struct LowerBoundedFunction<B, O> {
    /// The stored function f(x) = ???
    pub func: Box<dyn Fn(B) -> O>,
    /// The lower bound of the function.
    pub lower: B,
}

/// A lower partial function is a function that is defined by segments valid from [x..infinite[, or until it hits another function's start.
/// It starts searching at -infinity and goes up to infinity, and takes the last seen function that contains the desired invariable value (x).
///
/// Example:
/// [0..infinity[ = 5
/// [1..infinity[ = 10
///
/// f(0.5) = 5
/// f(1) = 10
/// f(70) = 10
pub struct LowerPartialFunction<B, O>
where
    B: PartialOrd,
{
    funcs: Vec<LowerBoundedFunction<B, O>>,
}

impl<B, O> LowerPartialFunction<B, O>
where
    B: PartialOrd,
{
    /// Creates a new LowerPartialFunctionBuilder.
    pub fn new() -> LowerPartialFunctionBuilder<B, O> {
        LowerPartialFunctionBuilder::new()
    }

    /// Evaluates the partial function.
    /// Returns None if no function is defined for the searched invariable value (x).
    pub fn eval(&self, x: B) -> Option<O> {
        let iter = self.funcs.iter().enumerate();
        for (i, bounded) in iter {
            let next = self.funcs.get(i + 1);
            if x >= bounded.lower && ((next.is_some() && next.unwrap().lower > x) || next.is_none())
            {
                let f = &bounded.func;
                return Some(f(x));
            }
        }
        None
    }
}

/// A builder to create an immutable PartialFunction.
#[derive(new)]
pub struct LowerPartialFunctionBuilder<B, O> {
    #[new(default)]
    funcs: Vec<LowerBoundedFunction<B, O>>,
}

impl<B: PartialOrd, O> LowerPartialFunctionBuilder<B, O> {
    /// Adds a bounded function bounded between [lower,higher[ of function func.
    pub fn with<F: Fn(B) -> O + 'static>(mut self, lower: B, func: F) -> Self {
        debug_assert!(self.can_insert(&lower));
        let f = LowerBoundedFunction {
            func: Box::new(func),
            lower,
        };
        self.funcs.push(f);
        self
    }

    /// Check if you can safely insert into the function list for the specified bounds.
    pub fn can_insert(&self, lower: &B) -> bool {
        !self.funcs.iter().any(|b| lower == &b.lower)
    }

    /// Builds the PartialFunction from the functions added using with.
    pub fn build(mut self) -> LowerPartialFunction<B, O> {
        self.funcs
            .sort_by(|a, b| a.lower.partial_cmp(&b.lower).unwrap_or(Ordering::Equal));
        LowerPartialFunction { funcs: self.funcs }
    }
}

