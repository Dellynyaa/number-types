use crate::*;

pub trait ToNum<T>: Number {
    const OUTPUT: T;
}

impl<N: Number + ToNum<isize>> ToNum<isize> for Negative<N> {
    const OUTPUT: isize = -N::OUTPUT;
}

impl<N: Number + ToNum<isize>> ToNum<isize> for Successor<N> {
    const OUTPUT: isize = N::OUTPUT + 1;
}

impl ToNum<isize> for Zero {
    const OUTPUT: isize = 0;
}

impl<N: Number + ToNum<usize>> ToNum<usize> for Successor<N> {
    const OUTPUT: usize = N::OUTPUT + 1;
}

impl ToNum<usize> for Zero {
    const OUTPUT: usize = 0;
}
