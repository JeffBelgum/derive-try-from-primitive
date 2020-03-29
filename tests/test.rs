use {
    core::convert::TryFrom,
    derive_try_from_primitive::TryFromPrimitive,
};

#[derive(TryFromPrimitive, Debug, PartialEq)]
#[repr(u16)]
enum Foo {
    Bar,
    Baz = 100,
    Quix = 200,
}

#[test]
fn generated_impl() {
    let bar = Foo::try_from(0);
    let baz = Foo::try_from(100);
    let quix = Foo::try_from(200);
    let bad = Foo::try_from(300);
    assert_eq!(bar.unwrap(), Foo::Bar);
    assert_eq!(baz.unwrap(), Foo::Baz);
    assert_eq!(quix.unwrap(), Foo::Quix);
    if let Err(value) = bad {
        assert_eq!(value, 300, "Input is returned for convenience");
    }
}
