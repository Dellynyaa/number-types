# number-types

A primitive, albeit with slight improvements, recreation of the typenum crate I made
in order to get more familiar with the typesystem.

## How to use it

Don't. I will publish it to crates.io when I feel like it's stable enough for that.
Not now.

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
