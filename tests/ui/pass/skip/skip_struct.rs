#![deny(dead_code)]

use modular_bitfield::prelude::*;

#[bitfield(skip(from_bytes, into_bytes))]
struct A {
    #[skip(setters)]
    f: u8,
}

#[bitfield(skip(new, into_bytes))]
struct B {
    #[skip(setters)]
    f: u8,
}

#[bitfield(skip(convert))]
#[derive(BitfieldSpecifier)]
struct C {
    #[skip(setters)]
    f: u8,
}

#[bitfield(skip(from_bytes, into_bytes))]
struct D {
    f: u8,
    _g: u8,
}

#[bitfield(skip(all))]
#[repr(u8)]
struct E {
    f: u8,
}

fn main() {
    let _ = A::new().f();
    let _ = B::from_bytes([0u8; 1]).f();
    let _ = C::new().f();
    let _ = D::new().f();
    let _ = E::from(0).f();
}
