# Sindex
**S**truct **I**ndex is a library for dynamically indexing structs with strings.

## Why
In Rust, or any programming language for the matter, how would you access a variable in a struct? Well you would probably write something like `foo.bar.my_field`. However, what if you wanted to change a variable dynamically, while the program is running or even based on a string? Well in that case you would have two options:
* Write a function with a large `match` statement that maps strings to fields.
* Or, serialize the entire object (to a dynamic format like JSON), then manipulate the serialized object, and then deserialize back.
Both methods have there downsides. The former can bet tedious to write if struct is very large and can even become error-prone. The latter requires very expensive data transformations `struct -> intermidiate format -> make changes -> struct`.
This library provides a third solution: automatically generated function with a match statement. This removes the drawbacks of the former option while keeping its performance and ease of use.

## Use Cases
Imaging you have a simulation and you would like to explore how it behaves under many different configurations. With this library, you could easily define which values you would like to test and their ranges.
```rust
// this is the base-line config
let config = FooConfig::new(...);

// these are the variables that I want 
let changes = [
    ("engine.horse_power", (180.0, 200.0)),
    ("wheels.diameter_cm", (30.0, 40.0)),
];

// make changes to the config and run
while ... {
    let mut config = config.clone();
    for (key, (min, max)) in &changes {
        let value = random(min, max);
        config.sindex_mut(key).unwrap().set_f64(value);
    }
    run_simulation(&config);
    ...
}
```

## Example
```rust
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

    println!("{:?}", foo.sindex("a")); // Some(1)
    println!("{:?}", foo.sindex("x")); // None
    println!("{:?}", foo.sindex("bar.d")); // Some(false)
    foo.sindex_mut("bar.d").unwrap().set_bool(false);
    println!("{:?}", foo.sindex("bar.d")); // Some(true)
}
```
