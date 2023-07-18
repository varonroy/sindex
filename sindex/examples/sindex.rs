use sindex::Sindex;
use sindex_derive::Sindex;

#[derive(Debug, Clone, Copy, Sindex)]
struct Foo {
    a: i64,
    bar: Bar,
}

#[derive(Debug, Clone, Copy, Sindex)]
struct Bar {
    c: f64,
    d: bool,
}

fn main() {
    let mut foo = Foo {
        a: 1,
        bar: Bar { c: 2.0, d: true },
    };

    println!("{:?}", foo.sindex("a"));
    println!("{:?}", foo.sindex("no_a_variable"));
    println!("{:?}", foo.sindex("bar.c"));
    foo.sindex_mut("bar.c").unwrap().set_bool(false);
    println!("{:?}", foo.sindex("bar.c"));
}
