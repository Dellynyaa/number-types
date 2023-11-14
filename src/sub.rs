use crate::*;

pub trait Sub<Rhs: Number>: Number {
    type Output: Number;
}

impl<Rhs: Number> Sub<Successor<Rhs>> for Zero {
    type Output = <Successor<Rhs> as Neg>::Output;
}

impl<Rhs: Number> Sub<Negative<Successor<Rhs>>> for Zero {
    type Output = Successor<Rhs>;
}

impl<N: Number> Sub<Zero> for N {
    type Output = N;
}

/* (0 + 1) - (0 + 1):
Sub<
    N = Zero as Sub<N = Zero, Rhs = Zero, Output = N>,
    Rhs = Zero,
    Output = N::Output,
*/
/* (0 + 1 + 1) - (0 + 1):
Sub<
    N = Successor<Zero> as Sub<N = Successor<Zero>, Rhs = Zero, Output = N>,
    Rhs = Zero,
    Output = N::Output,
*/
impl<N: Number, Rhs: Number, O: Number> Sub<Successor<Rhs>> for Successor<N>
where
    N: Sub<Rhs, Output = O>,
{
    type Output = O;
}

// -a - -b = -a + b
impl<N: Number, Rhs: Number, O: Number> Sub<Negative<Rhs>> for Negative<N>
where
    Negative<N>: Add<Rhs, Output = O>,
{
    type Output = O;
}

// a - -b = a + b
impl<N: Number, Rhs: Number, O: Number> Sub<Negative<Rhs>> for Successor<N>
where
    N: Add<Rhs, Output = O>,
{
    type Output = Successor<O>;
}

// -a - b = -(a + b)
impl<N: Number, Rhs: Number, O: Number + Neg> Sub<Successor<Rhs>> for Negative<N>
where
    N: Add<Successor<Rhs>, Output = O>,
{
    type Output = <O as Neg>::Output;
}

pub trait SubOp {
    type Output: Number;
}

impl<Lhs: Number, Rhs: Number> SubOp for Op<Lhs, Rhs>
where
    Lhs: Sub<Rhs>,
{
    type Output = <Lhs as Sub<Rhs>>::Output;
}
