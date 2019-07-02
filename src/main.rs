use std::{fmt::Debug, marker::PhantomData};

struct Wrapper<F, O, T> {
    f: F,
    pd: PhantomData<(T, O)>,
}

impl<F, O, T> Wrapper<F, O, T>
where
    F: FnOnce(&T) -> O,
    O: Debug,
{
    fn new(f: F) -> Wrapper<F, O, T> {
        Wrapper { f, pd: PhantomData }
    }

    fn foo(self, t: &T) {
        let r = (self.f)(t);
        println!("result: {:?}", r);
    }
}

fn main() {
    Wrapper::new(|i: &String| i.as_str()).foo(&String::new());
}
