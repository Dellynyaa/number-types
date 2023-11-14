#[macro_export]
macro_rules! assert_types_eq {
    ($a:ty, $b:ty) => {{
        let _: ::std::marker::PhantomData<$a> = ::std::marker::PhantomData::<$b>;
    }};
}

#[macro_export]
macro_rules! type_expr {
    (<$a:ty> + <$b:ty>) => {
        <Op<$a, $b> as AddOp>::Output
    };
    (<$a:ty> * <$b:ty>) => {
        <Op<$a, $b> as MulOp>::Output
    };
    (<$a:ty> - <$b:ty>) => {
        <Op<$a, $b> as SubOp>::Output
    };
    (<$a:ty> + $($b:tt)+) => {
        <Op<$a, type_expr!($($b)+)> as AddOp>::Output
    };
    (<$a:ty> * $($b:tt)+) => {
        <Op<$a, type_expr!($($b)+)> as MulOp>::Output
    };
    (<$a:ty> - $($b:tt)+) => {
        <Op<$a, type_expr!($($b)+)> as SubOp>::Output
    };
}
