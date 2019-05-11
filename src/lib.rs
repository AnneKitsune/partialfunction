#[macro_use]
extern crate serde;

use serde::Deserialize;
use std::cmp::Ordering;
use std::ops::{Add, Mul};

/*

Function
    StdFunction Fn(I)->I
    Custom Fn(I)->O
build()
-> Fn(I) -> O


DualBoundedFunction
    Fn(I) -> O
    lower: I
    higher: I

PartialFunction
    Vec<DualBoundedFunction>
eval(I)
-> O


PartialFunction<I,I>
eval(I) -> I

PartialFunction<I,O>
eval(I) -> O


O: From<I>

*/

/// Defines the most commonly used functions to allow deserializing a partial function from file.
#[derive(Deserialize, Serialize)]
pub enum StdFunction<T: Add + Mul> {
    /// A constant function.
    /// `f(x) = a`
    Constant { a: T },
    /// An affine (or linear) function.
    /// `f(x) = a * x + b`
    Affine { a: T, b: T },
    /// A logarithmic function.
    /// `f(x) = a * log(x - b) + c
    Logarithmic { a: T, b: T, c: T },
    /// An exponential function.
    /// `f(x) = a * b ^ (x - c) + d`
    Exponential { a: T, b: T, c: T, d: T },
    /// An inverse function.
    /// `f(x) = a * (1 / (b * x - c)) + d`
    Inverse { a: T, b: T, c: T },
    /// A polynomial function.
    /// `f(x) = a * x ^ 2 + b * x + c`
    Polynomial { a: T, b: T, c: T },
}

#[derive(Deserialize, Serialize)]
pub enum Function<I: Add + Mul + PartialOrd, O> {
    Std(StdFunction<I>),
    #[serde(skip)]
    Custom(Box<Fn(I) -> O>),
}

impl<I: Add + Mul + PartialOrd + 'static> Function<I, I> {
    pub fn to_closure(&self) -> Box<Fn(I) -> I> {
        match *self {
            Function::Std(f) => match f {
                StdFunction::Constant { a } => Box::new(move |x| a),
            },
            Function::Custom(f) => f,
        }
    }
}

#[derive(Deserialize)]
pub struct DualBoundedFunctionBuilder<B: Add + Mul + PartialOrd, O> {
    /// The stored function f(x) = ???
    pub func: Function<B, O>,
    /// The lower bound of the function.
    pub lower: B,
    /// The higher bound of the function.
    pub higher: B,
}

impl<B: Add + Mul + PartialOrd, O> DualBoundedFunctionBuilder<B, O> {
    pub fn build(self) -> DualBoundedFunction<B, O> {
        DualBoundedFunction {
            func: self.func.to_closure(),
            lower: self.lower,
            higher: self.higher,
        }
    }
}

/// A regular function that is only defined between lower and higher.
/// If two functions intersect their higher and lower bounds respectively.
/// The second will take precedence where f(lower).
pub struct DualBoundedFunction<B, O>
where
    B: Add + Mul + PartialOrd,
{
    /// The stored function f(x) = ???
    pub func: fn(B) -> O,
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
pub struct PartialFunction<B, O>
where
    B: Add + Mul + PartialOrd,
{
    funcs: Vec<DualBoundedFunction<B, O>>,
}

impl<B, O> PartialFunction<B, O>
where
    B: Add + Mul + PartialOrd,
{
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
                let f = bounded.func;
                return Some(f(x));
            }
        }
        None
    }
}

/// A builder to create an immutable PartialFunction.
pub struct PartialFunctionBuilder<B, O>
where
    B: Add + Mul + PartialOrd,
{
    funcs: Vec<DualBoundedFunction<B, O>>,
}

impl<B, O> PartialFunctionBuilder<B, O>
where
    B: Add + Mul + PartialOrd,
{
    /// Creates a new PartialFunctionBuilder.
    pub fn new() -> Self {
        PartialFunctionBuilder { funcs: vec![] }
    }

    /// Adds a bounded function bounded between [lower,higher[ of function func.
    pub fn with(mut self, lower: B, higher: B, func: fn(B) -> O) -> Self {
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
struct LowerBoundedFunction<B, O>
where
    B: PartialOrd,
{
    /// The stored function f(x) = ???
    pub func: Box<Fn(B) -> O>,
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
pub struct LowerPartialFunctionBuilder<B, O>
where
    B: PartialOrd,
{
    funcs: Vec<LowerBoundedFunction<B, O>>,
}

impl<B, O> LowerPartialFunctionBuilder<B, O>
where
    B: PartialOrd,
{
    /// Creates a new PartialFunctionBuilder.
    pub fn new() -> Self {
        LowerPartialFunctionBuilder { funcs: vec![] }
    }

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
