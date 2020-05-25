Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# Partial Function
A clean way to define function as a set of smaller functions where each has defined start and end bounds.

## Partial Function

Achieves the following:
```
f(x) = {
    x     if 0 <= x <   5
    x * 2 if 5 <= x <= 10
}
```
Expressed as:
```rust
let p = PartialFunction::new()
    .with(0.0, 5.0,  Box::new(|x| x    ))
    .with(5.0, 10.0, Box::new(|x| x * 2))
    .build();
assert_eq!(p.eval(5.0), Some(10.0));
```
or even as:
```rust
let p = partfn! {
    [0.0, 5.0]:  x -> x,
    [5.0, 10.0]: x -> x * 2,
};
assert_eq!(p.eval(5.0), Some(10.0));
```

## Lower Partial Function

Achieves the following:
```
f(x) = {
    x     if 0 <= x <   5
    x * 2 if 5 <= x
}
```
Expressed as:
```rust
let f = LowerPartialFunction::new()
    .with(0.0, Box::new(|x| x    ))
    .with(5.0, Box::new(|x| x * 2))
    .build();
assert_eq!(f.eval(5.0), Some(10.0));
```
or even as:
```rust
let f = lowpartfn! {
    [0.0]: x -> x,
    [5.0]: x -> x * 2,
};
assert_eq!(f.eval(5.0), Some(10.0));
```

## Adding To Your Project
Add the following to your Cargo.toml:
```
partial_function = "0.5.0"
```

