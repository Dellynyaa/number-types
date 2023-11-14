# number-types

A primitive, albeit with slight improvements, recreation of the typenum crate I made
in order to get more familiar with the typesystem.

## Improvements over typenum

It uses its own traits and **only** uses types, never instances. Not a single instance 
is ever created. 

That's basically it, everything else is probably worse. This is just for fun.

## How to use it

There's a few macros for easier expression-crafting. They always run the operations 
in the same order: `type_expr!(<a> * <b> + <c> - <d>)` will do `(a * (b + (c - d)))`.
Weird, but not my choice. I just didn't want to spend THAT long on a macro.

There's types called Z0..Z10 and Z100, and negative counterparts for each (ZN...).
To craft operations yourself, use `<Op<Lhs, Rhs> as OpKindOp>::Output` (OpKind should be
Add, Mul, or Sub). To negate a number, use `<N as Neg>::Output`. 

You can assert type equality using assert_type_eq!(a, b).

## How does it work

All numbers are simply N wrapping layers of `Successor` around Zero. Negative numbers
also get a `Negative` wrapper at the top layer.

Addition is done essentially like this:
- If Lhs is Zero, Output = Rhs and return.
- Otherwise unwrap one layer, repeat from step one, wrap result in one more layer,
  then return it.

Subtraction is done like this:
- If Rhs is Zero, Output = Lhs.
- If Lhs is Zero, Output = Negative\<Rhs\>.
- Otherwise unwrap one layer on both sides and try again.

This includes some more cases to handle negative numbers (since they can turn the 
subtraction into what's basically an addition), but I won't explain those here.

Multiplication is repeated addition.

---

Did you know you can pattern match types?

```rs
impl<A: Constr, B: Constr, O> TraitA<B> for SomeWrapper(A)
where
  SomeOtherOrSameWrapper(A): TraitB<B, Output = O>
{
  type Output = O;
}
```

:3
