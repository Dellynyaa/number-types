use crate::*;

pub trait Mul<Rhs: Number>: Number {
    type Output: Number;
}

impl<Rhs: Number> Mul<Rhs> for Zero {
    type Output = Zero;
}

// multiplies by repeated addition
/* (0 + 1) * (0 + 1 + 1)
Mul<
    N = Zero,
    Rhs = Successor<Successor<Zero>>,
    Output = Rhs + Zero * Rhs,
>
*/
impl<N: Number, Rhs: Number> Mul<Rhs> for Successor<N>
where
    N: Mul<Rhs>,
    Rhs: Add<<N as Mul<Rhs>>::Output>,
{
    type Output = <Rhs as Add<<N as Mul<Rhs>>::Output>>::Output;
}

// multiplication of something negative:
// -a * b = -(a * b)
impl<N: Number, Rhs: Number> Mul<Rhs> for Negative<N>
where
    N: Mul<Rhs>,
    <N as Mul<Rhs>>::Output: Neg,
{
    type Output = <<N as Mul<Rhs>>::Output as Neg>::Output;
}

pub trait MulOp {
    type Output: Number;
}

impl<Lhs: Number, Rhs: Number> MulOp for Op<Lhs, Rhs>
where
    Lhs: Mul<Rhs>,
{
    type Output = <Lhs as Mul<Rhs>>::Output;
}
