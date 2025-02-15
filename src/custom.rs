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

pub trait CollectIntoOptionalVec<T>: Iterator {
    fn collect_some(self) -> Option<Vec<T>>;
}
impl<I, T> CollectIntoOptionalVec<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_some(self) -> Option<Vec<T>>
    where
        Self: Sized,
    {
        let v: Vec<T> = self.collect();
        (!v.is_empty()).then_some(v)
    }
}

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
