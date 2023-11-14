use crate::*;

pub trait Add<Rhs: Number>: Number {
    type Output: Number;
}

impl<Rhs: Number> Add<Rhs> for Zero {
    type Output = Rhs;
}

// basic positive addition
/* (0 + 1) + (0 + 1):
Successor<Zero> as
    Add<
        N = Zero as Add<N = Zero, Rhs = Successor<Zero>, Output = Successor<Zero>>,
        Rhs = Successor<Zero>,
        Output = Successor<T::O>
    >
*/
impl<N: Number, Rhs: Number, O: Number> Add<Rhs> for Successor<N>
where
    Rhs: IsPositive,
    N: Add<Rhs, Output = O>,
{
    type Output = Successor<O>;
}

// -a + b = -(a - b)
impl<N: Number, Rhs: Number, O: Number> Add<Rhs> for Negative<N>
where
    Rhs: IsPositive,
    N: Sub<Rhs, Output = O>,
    O: Neg,
{
    type Output = <O as Neg>::Output;
}

// a + -b = a - b
impl<N: Number, Rhs: Number, O: Number> Add<Negative<Rhs>> for Successor<N>
where
    Successor<N>: Sub<Rhs, Output = O>,
{
    type Output = O;
}
// -a + -b = -(a + b)
impl<N: Number, Rhs: Number> Add<Negative<Rhs>> for Negative<N>
where
    N: Add<Rhs>,
{
    type Output = Negative<<N as Add<Rhs>>::Output>;
}

pub trait AddOp {
    type Output: Number;
}

impl<Lhs, Rhs: Number> AddOp for Op<Lhs, Rhs>
where
    Lhs: Add<Rhs>,
{
    type Output = <Lhs as Add<Rhs>>::Output;
}
