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
