#![feature(arbitrary_self_types)]

#[derive(Debug)]
struct Foo;

impl Foo {
    fn a(self: &Box<Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0106

    fn b(self: &Box<Foo>, f: &Foo) -> &Box<Foo> { self } //~ ERROR E0106

    fn c(this: &Box<Foo>, f: &Foo) -> &Foo { f } //~ ERROR E0106
}

fn main() {}
