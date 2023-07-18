use value_derive::value;

/// Allows indexing structs using strings
///
/// Example:
/// ```rust
/// use sindex::Sindex;
/// use sindex_derive::Sindex;
///
/// #[derive(Sindex)]
/// struct Foo {
///     foo: i64,
///     bar: Bar,
/// }
///
/// #[derive(Sindex)]
/// struct Bar {
///     bar: f32,
/// }
///
/// let mut foo = Foo {
///     foo: 1,
///     bar: Bar { bar: 2.0 },
/// };
///
/// // assert_eq!(foo.sindex("foo"), Some(&foo.foo));
/// // assert_eq!(foo.sindex("bar.bar"), Some(&foo.bar.bar));
/// // assert!(foo.sindex("x").is_none());
/// ```
pub trait Sindex {
    /// Returns a shared reference to a field (or a nested field).
    /// Use a `.` to specify nested fields, for example `foo.bar.c`.
    fn sindex<'a>(&'a self, key: &str) -> Option<Value<'a>>;

    /// Returns a mutable reference to a field (or a nested field).
    /// Use a `.` to specify nested fields, for example `foo.bar.c`.
    fn sindex_mut<'a>(&'a mut self, key: &str) -> Option<ValueMut<'a>>;
}

#[value(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool, str)]
#[derive(Debug, Clone, Copy)]
pub enum Value {
    // Generated fields
}
