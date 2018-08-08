use std::cmp::Ordering;

/// A regular function that is only defined between lower and higher.
/// If two functions intersect their higher and lower bounds respectively.
/// The second will take precedence where f(lower).
pub struct BoundedFunction<B, O>
where
    B: PartialOrd,
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
    B: PartialOrd,
{
    funcs: Vec<BoundedFunction<B, O>>,
}

impl<B, O> PartialFunction<B, O>
where
    B: PartialOrd,
{
    /// Creates a new PartialFunctionBuilder
    pub fn new() -> PartialFunctionBuilder<B, O> {
        PartialFunctionBuilder::new()
    }

    /// Evaluates the partial function.
    /// Returns NAN if no function is defined.
    pub fn eval(&self, x: B) -> Option<O> {
        let iter = self.funcs.iter().enumerate();
        for (i, bounded) in iter {
            let next = self.funcs.get(i + 1);
            if (x >= bounded.lower && x < bounded.higher) ||
                (next.is_none() && x == bounded.higher) ||
                (next.is_some() && next.unwrap().lower != bounded.higher)
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
    B: PartialOrd,
{
    funcs: Vec<BoundedFunction<B, O>>,
}

impl<B, O> PartialFunctionBuilder<B, O>
where
    B: PartialOrd,
{
    /// Creates a new PartialFunctionBuilder.
    pub fn new() -> Self {
        PartialFunctionBuilder { funcs: vec![] }
    }

    /// Adds a bounded function bounded between [lower,higher[ of function func.
    pub fn with(mut self, lower: B, higher: B, func: fn(B) -> O) -> Self {
        debug_assert!(self.can_insert(&lower, &higher));
        let f = BoundedFunction {
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
            (lower >= &b.lower && lower < &b.higher) ||
                (higher > &b.lower && higher <= &b.higher) ||
                (lower <= &b.lower && higher >= &b.higher)
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


struct LowerBoundedFunction<B, O>
where
    B: PartialOrd,
{
    /// The stored function f(x) = ???
    pub func: fn(B) -> O,
    /// The lower bound of the function.
    pub lower: B,
}

pub struct LowerPartialFunction<B, O>
where
    B: PartialOrd,
{
    funcs: Vec<LowerBoundedFunction<B, O>>,
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
    pub fn with(mut self, lower: B, func: fn(B) -> O) -> Self {
        debug_assert!(self.can_insert(&lower));
        let f = LowerBoundedFunction {
            func,
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
        self.funcs.sort_by(|a, b| {
            a.lower
                .partial_cmp(&b.lower)
                .unwrap_or(Ordering::Equal)
        });
        LowerPartialFunction { funcs: self.funcs }
    }
}
