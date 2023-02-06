# Matrix Match
This crate provides a macro to easily match on two values at the same time.

The macro takes a matrix of possible results and patterns for the rows and
columns. The expression at the intersection of the matching patterns gets executed and possibly returned.

This crate can be used in no-std contexts.

## Example

```rust
use matrix_match::matrix_match;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Light {
    Red,
    Orange,
    Green,
}

fn next(light: Light, car_waiting: bool) -> Light {
    use Light::*;
    matrix_match!(
        (light, car_waiting) ; true  , false =>
        Red                 => Green , Red    ;
        Orange              => Red   , Red    ;
        Green               => Green , Orange ;
    )
}

fn main() {
    assert_eq!(next(Light::Red,    true ), Light::Green);
    assert_eq!(next(Light::Red,    false), Light::Red);

    assert_eq!(next(Light::Orange, true ), Light::Red);
    assert_eq!(next(Light::Orange, false), Light::Red);

    assert_eq!(next(Light::Green,  true ), Light::Green);
    assert_eq!(next(Light::Green,  false), Light::Orange);
}
```

## Implementation details
The macro first creates a match for the row patterns and then for every row creates a match for the column patterns.
There exists a implementation in the [single-match](tree/single-match) branch that only creates a single match. 
After some benchmarking (also in that branch) it was determined that both implementation are just as fast at run time and so the simpler one was chosen.
