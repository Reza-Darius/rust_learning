use std::ops::Deref;

trait Foo {}

struct Bar;

impl Foo for Bar {}

fn dyn_dispatch(f: &dyn Foo) {}
fn static_dispatch<T: Foo + ?Sized>(f: &T) {}

#[test]
fn dyn_memes() {
    let bar = Bar;
    let dyn_foo: Box<dyn Foo> = Box::new(Bar);

    static_dispatch(&bar);
    // concrete type can get coerced to dynamic trait object
    dyn_dispatch(&bar);

    dyn_dispatch(dyn_foo.as_ref());
    dyn_dispatch(&*dyn_foo);
    dyn_dispatch(dyn_foo.deref());

    // because T: Trait has an implicit Sized bound
    // we need to relax it
    static_dispatch(dyn_foo.as_ref());
    static_dispatch(&*dyn_foo);
    static_dispatch(dyn_foo.deref());
}
