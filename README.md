Example
=======

```rust
use core::convert::TryFrom;
use derive_try_from_primitive::TryFromPrimitive;


#[derive(TryFromPrimitive)]
#[repr(u16)]
enum Foo {
    Bar,
    Baz = 100,
    Quix = 200,
}

// Generated Code:
impl core::convert::TryFrom<u16> for Foo {
    type Error = u16;

    fn try_from(n: 16) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Foo::Bar),
            100 => Ok(Foo::Baz),
            200 => Ok(Foo::Quix),
            _ => Err(n),
        }
    }
}

fn main() {
    let bar = Foo::try_from(0);
    let baz = Foo::try_from(100);
    let quix = Foo::try_from(200);
    let bad = Foo::try_from(300);
    assert_eq!(bar.unwrap() as u16, 0);
    assert_eq!(baz.unwrap() as u16, 100);
    assert_eq!(quix.unwrap() as u16, 200);
    if let Err(value) = bad {
        assert_eq!(value, 300, "Input is returned for convenience");
    }
}
```
