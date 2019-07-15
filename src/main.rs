use std::{fmt::Debug, marker::PhantomData};

struct Wrapper<F, O, T> {
    f: F,
    state: T,
    pd: PhantomData<O>,
}

impl<F, O, T> Wrapper<F, O, T>
where
    T: 'static,
    F: FnOnce(&T) -> O,
    O: Debug,
{
    fn new(state: T, f: F) -> Self {
        Wrapper {
            state,
            f,
            pd: PhantomData,
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
