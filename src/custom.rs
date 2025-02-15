pub trait Mutated: Sized {
    // fn mutated(mut self, mut func: impl FnMut(&mut Self)) -> Self {
    //     func(&mut self);
    //     self
    // }
    fn mutated(mut self, mut func: impl FnOnce(&mut Self)) -> Self {
        func(&mut self);
        self
    }
}

impl<T> Mutated for T {}

pub trait OptionBuilder: Sized {
    fn try_build<P>(&mut self, param: Option<P>, func: impl Fn(P) -> Self);
}

// impl<T> OptionBuilder for Option<T>
/**
```rust
embed.try_build(objective, |objective| {
    embed.field("Objective", objective, false)
});
```
*/
impl<T> OptionBuilder for T {
    fn try_build<P>(&mut self, param: Option<P>, func: impl Fn(P) -> Self) {
        if let Some(param) = param {
            *self = func(param);
        }
    }
}
