use crate::*;

pub trait IsPositive: Number {}

impl IsPositive for Zero {}
impl<N: Number> IsPositive for Successor<N> {}

pub trait Neg: Number {
    type Output: Number + Neg;
}

impl Neg for Zero {
    type Output = Zero;
}

impl<N: Number> Neg for Successor<N> {
    type Output = Negative<Successor<N>>;
}

impl<N: Number + Neg> Neg for Negative<N> {
    type Output = N;
}
