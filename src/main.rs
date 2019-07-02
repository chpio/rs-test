use std::{fmt::Debug, marker::PhantomData};

struct Wrapper<F, O, T> {
    f: F,
    pd: PhantomData<(T, O)>,
}

impl<'a, F, O, T> Wrapper<F, O, T>
where
    T: 'static,
    F: FnOnce(&'a T) -> O,
    O: 'a + Debug,
{
    fn new(f: F) -> Wrapper<F, O, T> {
        Wrapper { f, pd: PhantomData }
    }

    fn foo(self, t: &'a T) {
        let r = (self.f)(t);
        println!("result: {:?}", r);
    }
}

fn main() {
    let s = String::new();
    Wrapper::new(|i: &String| i.as_str()).foo(&s);
}
