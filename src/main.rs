#![feature(generic_associated_types)]
use std::{fmt::Debug, marker::PhantomData};

struct Wrapper<F, T> {
    f: F,
    state: T,
    // pd: PhantomData<O>,
}


trait WrapperTrait {
    type OUTPUT<'a>: 'a + Debug;
}

impl<O, F, T> WrapperTrait for Wrapper<F, T> {
    type OUTPUT<'a> = O where O: 'a + Debug;
}

impl<F, T> Wrapper<F, T>
where
    Self: WrapperTrait,
    T: 'static,
    F: FnOnce(&T) -> <Self as WrapperTrait>::OUTPUT,
{
    fn new(state: T, f: F) -> Self {
        Wrapper {
            state,
            f,
            // pd: PhantomData,
        }
    }

    fn foo(self) {
        let r = (self.f)(&self.state);
        println!("result: {:?}", r);
    }
}

fn main() {
    Wrapper::new(String::new(), |i: &String| i.as_str()).foo();
}
