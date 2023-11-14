#![recursion_limit = "512"]

mod macros;
pub use macros::*;
mod to_num;
pub use to_num::*;

use std::marker::PhantomData;

pub trait Number {}

pub struct Zero;
pub struct Successor<N: Number>(PhantomData<N>);
pub struct Negative<N: Number>(PhantomData<N>);

// number impls:

impl Number for Zero {}
impl<N: Number> Number for Successor<N> {}
impl<N: Number> Number for Negative<N> {}

// numbers

pub type One = Successor<Zero>;

pub type Z0 = Zero;
pub type Z1 = Successor<Z0>;
pub type Z2 = Successor<Z1>;
pub type Z3 = Successor<Z2>;
pub type Z4 = Successor<Z3>;
pub type Z5 = Successor<Z4>;
pub type Z6 = Successor<Z5>;
pub type Z7 = Successor<Z6>;
pub type Z8 = Successor<Z7>;
pub type Z9 = Successor<Z8>;
pub type Z10 = Successor<Z9>;
pub type Z100 = type_expr!(<Z10> * <Z10>);
pub type ZN0 = Negative<Z0>;
pub type ZN1 = Negative<Z1>;
pub type ZN2 = Negative<Z2>;
pub type ZN3 = Negative<Z3>;
pub type ZN4 = Negative<Z4>;
pub type ZN5 = Negative<Z5>;
pub type ZN6 = Negative<Z6>;
pub type ZN7 = Negative<Z7>;
pub type ZN8 = Negative<Z8>;
pub type ZN9 = Negative<Z9>;
pub type ZN10 = Negative<Z10>;
pub type ZN100 = Negative<Z100>;

// ops:

pub struct Op<Lhs: Number, Rhs: Number>(PhantomData<Lhs>, PhantomData<Rhs>);

mod add;
mod mul;
mod neg;
mod sub;
pub use add::*;
pub use mul::*;
pub use neg::*;
pub use sub::*;

// tests:

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile() {
        // addition
        type TAOne = <Op<Zero, One> as AddOp>::Output;
        type TATwo = <Op<TAOne, TAOne> as AddOp>::Output;
        type TAThree1 = <Op<TAOne, TATwo> as AddOp>::Output;
        type TAThree2 = <Op<TATwo, TAOne> as AddOp>::Output;
        assert_types_eq!(TAOne, Z1);
        assert_types_eq!(TATwo, Z2);
        assert_types_eq!(TAThree1, TAThree2);
        assert_types_eq!(TAThree1, Z3);

        // subtraction, including negatives

        type TSOne1 = <Op<TATwo, TAOne> as SubOp>::Output;
        type TSOne2 = <Op<TAThree1, TATwo> as SubOp>::Output;
        type TSTwo1 = <Op<TATwo, Zero> as SubOp>::Output;
        type TSTwo2 = <Op<TAThree1, TAOne> as SubOp>::Output;
        type TSNegOne = <Op<TAOne, TATwo> as SubOp>::Output;
        type TSNegTwo = <Op<TAOne, TAThree1> as SubOp>::Output;
        type TSNegThree = <Op<Zero, TAThree1> as SubOp>::Output;
        assert_types_eq!(TSOne1, TSOne2);
        assert_types_eq!(TSOne1, Z1);
        assert_types_eq!(TSTwo1, TSTwo2);
        assert_types_eq!(TSTwo1, Z2);
        assert_types_eq!(TSNegOne, ZN1);
        assert_types_eq!(TSNegTwo, ZN2);
        assert_types_eq!(TSNegThree, ZN3);

        // subtraction, then addition

        assert_types_eq!(<Op<TSNegOne, TSTwo1> as AddOp>::Output, Z1);
        assert_types_eq!(<Op<TSOne1, TSNegOne> as AddOp>::Output, Z0); // 1 + -1 = 0

        // subtraction, then subtraction

        assert_types_eq!(<Op<TSNegOne, TSTwo1> as SubOp>::Output, ZN3);
        assert_types_eq!(<Op<TSOne1, TSNegOne> as SubOp>::Output, Z2);

        assert_types_eq!(<Op<ZN1, ZN2> as SubOp>::Output, Z1);
        assert_types_eq!(<Op<ZN2, ZN3> as SubOp>::Output, Z1);
        assert_types_eq!(<Op<ZN2, ZN4> as SubOp>::Output, Z2);

        assert_types_eq!(type_expr!(<Z5> + <Z5>), <Op<Z5, Z5> as AddOp>::Output);
    }

    #[test]
    fn test_run() {
        assert_eq!(
            <type_expr!(<Z5> + <Z8> + <Z8>) as ToNum<usize>>::OUTPUT,
            5 + 8 + 8
        );
        assert_eq!(<type_expr!(<Z100> * <Z5>) as ToNum<usize>>::OUTPUT, 500);
        assert_eq!(<type_expr!(<Z100> * <ZN5>) as ToNum<isize>>::OUTPUT, -500);
    }
}
