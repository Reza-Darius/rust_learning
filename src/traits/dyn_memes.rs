use std::ops::Deref;

trait Foo {}

struct Bar;

impl Foo for Bar {}

fn need_foo_dyn(f: &dyn Foo) {}

fn need_foo(f: &impl Foo) {}

#[test]
fn dyn_memes() {
    let bar = Bar;
    let dyn_foo: Box<dyn Foo> = Box::new(Bar);

    need_foo(&bar);
    // concrete type can get coerced to dynamic trait object
    need_foo_dyn(&bar);

    need_foo_dyn(dyn_foo.as_ref());
    need_foo_dyn(&*dyn_foo);
    need_foo_dyn(dyn_foo.deref());

    // none of these work:
    // need_foo(dyn_foo.as_ref());
    // need_foo(&*dyn_foo);
    // need_foo(dyn_foo.deref());
}
