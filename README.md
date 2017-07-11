Example
=======

```rust
#[macro_use]
extern crate derive_try_from_primitive;


#[derive(TryFromPrimitive)]
#[repr(u16)]
enum Foo {
    Bar,
    Baz = 100,
    Quix = 200,
}

fn main() {
  let bar = Foo::try_from(0);
  let baz = Foo::try_from(100);
  let quix = Foo::try_from(200);
  let bad = Foo::try_from(300);
  assert_eq!(bar.unwrap() as u16, 0);
  assert_eq!(baz.unwrap() as u16, 100);
  assert_eq!(quix.unwrap() as u16, 200);
  assert!(bad.is_none());
}
```
