use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Specifier, Specifier)]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
