use std::cmp::Ordering;
use std::f32;

/// A regular function that is only defined between lower and higher
/// If two functions intersect their higher and lower bounds respectively
/// The second will take precedence where f(lower)
pub struct BoundedFunction {
    /// The stored function f(x) = ???
    pub func: fn(f32) -> f32,
    /// The lower bound of the function
    pub lower: f32,
    /// The higher bound of the function
    pub higher: f32,
}

/// Define a functions defined by multiple functions parts
/// See BoundedFunction
/// Uses bounds as [lower,higher],
/// except in the case of a lower bound overlapping a higher bound.
/// In this case, the lower bound always take precedence.
pub struct PartialFunction {
    funcs: Vec<BoundedFunction>,
}

impl PartialFunction {
    /// Creates a new PartialFunctionBuilder
    pub fn new() -> PartialFunctionBuilder {
        PartialFunctionBuilder::new()
    }

    /// Evaluates the partial function.
    /// Returns NAN if no function is defined.
    pub fn eval(&self, x: f32) -> f32 {
        let iter = self.funcs.iter().enumerate();
        for (i, bounded) in iter {
            let next = self.funcs.get(i + 1);
            if (x >= bounded.lower && x < bounded.higher) ||
                (next.is_none() && x == bounded.higher) ||
                (next.is_some() && next.unwrap().lower != bounded.higher)
            {
                let f = bounded.func;
                return f(x);
            }
        }
        f32::NAN
    }
}

/// A builder to create an immutable PartialFunction
pub struct PartialFunctionBuilder {
    funcs: Vec<BoundedFunction>,
}

impl PartialFunctionBuilder {
    /// Creates a new PartialFunctionBuilder
    pub fn new() -> Self {
        PartialFunctionBuilder { funcs: vec![] }
    }

    /// Adds a bounded function bounded between [lower,higher[ of function func
    pub fn with(mut self, lower: f32, higher: f32, func: fn(f32) -> f32) -> Self {
        let f = BoundedFunction {
            func: func,
            lower: lower,
            higher: higher,
        };
        debug_assert!(self.can_insert(lower, higher));
        self.funcs.push(f);
        self
    }

    /// Check if you can safely insert into the function list for the specified bounds
    pub fn can_insert(&self, lower: f32, higher: f32) -> bool {
        !self.funcs.iter().any(|b| {
            (lower >= b.lower && lower < b.higher) || (higher > b.lower && higher <= b.higher) ||
                (lower <= b.lower && higher >= b.higher)
        })
    }

    /// Builds the PartialFunction from the functions added using with
    pub fn build(mut self) -> PartialFunction {
        self.funcs.sort_by(|a, b| {
            a.lower
                .partial_cmp(&b.lower)
                .unwrap_or(a.higher.partial_cmp(&b.higher).unwrap_or(Ordering::Equal))
        });
        PartialFunction { funcs: self.funcs }
    }
}
