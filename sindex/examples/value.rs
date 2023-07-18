use sindex::{Value, ValueMut};

fn main() {
    let s = "abc";
    let s = Value::Str(s);
    println!("{:?}", s.get_i8());
    println!("{:?}", s.get_str());

    let mut a = 1;
    let mut a = ValueMut::I8(&mut a);
    println!("{:?}", a.get_str());
    println!("{:?}", a.get_i8());
    assert_eq!(1, a.set_i8(11).unwrap());
    println!("{:?}", a.get_i8());

    let mut a = 1.0;
    let mut a = ValueMut::F64(&mut a);
    println!("{:?}", a.get_str());
    println!("{:?}", a.get_f64());
    assert_eq!(1.0, a.set_f64(11.0).unwrap());
    println!("{:?}", a.get_f64());
}
