use {core::convert::TryFrom, derive_try_from_primitive::TryFromPrimitive};

#[derive(TryFromPrimitive, Debug, PartialEq)]
#[repr(u16)]
enum Foo {
    A,       // first discriminant does not have to be specified
    B = 100, // specified discriminant following unspecified
    C = 200, // specified discriminant following specified
    D,       // unspecified discriminant following specified
    E,       // unspecified discriminant following unspecified
}

#[test]
fn generated_impl() {
    let a = Foo::try_from(0);
    let b = Foo::try_from(100);
    let c = Foo::try_from(200);
    let d = Foo::try_from(201);
    let e = Foo::try_from(202);
    let bad = Foo::try_from(300);
    assert_eq!(a.unwrap(), Foo::A);
    assert_eq!(b.unwrap(), Foo::B);
    assert_eq!(c.unwrap(), Foo::C);
    assert_eq!(d.unwrap(), Foo::D);
    assert_eq!(e.unwrap(), Foo::E);
    if let Err(value) = bad {
        assert_eq!(value, 300, "Input is returned for convenience");
    }
}
