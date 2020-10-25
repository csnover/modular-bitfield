use super::{
    Bits,
    FromBits,
    IntoBits,
};
use crate::{
    error::{
        InvalidBitPattern,
        OutOfBounds,
    },
    Specifier,
};

impl Specifier for bool {
    const BITS: usize = 1;
    type Bytes = u8;
    type InOut = bool;

    #[inline]
    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
        Ok(input as u8)
    }

    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
        match bytes {
            0 => Ok(false),
            1 => Ok(true),
            invalid_bytes => Err(InvalidBitPattern { invalid_bytes }),
        }
    }
}

macro_rules! impl_specifier_for_primitive {
    ( $( ($prim:ty: $bits:literal) ),* $(,)? ) => {
        $(
            impl Specifier for $prim {
                const BITS: usize = $bits;
                type Bytes = $prim;
                type InOut = $prim;

                #[inline]
                fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                    Ok(input)
                }

                #[inline]
                fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                    Ok(bytes)
                }
            }
        )*
    };
}
impl_specifier_for_primitive!(
    (u8: 8),
    (u16: 16),
    (u32: 32),
    (u64: 64),
    (u128: 128),
);

impl FromBits<u8> for bool {
    #[inline(always)]
    fn from_bits(bits: Bits<u8>) -> Self {
        bits.into_raw() != 0
    }
}

impl IntoBits<u8> for bool {
    #[inline(always)]
    fn into_bits(self) -> Bits<u8> {
        Bits(self as u8)
    }
}

macro_rules! impl_wrapper_from_naive {
    ( $($type:ty),* ) => {
        $(
            impl IntoBits<$type> for $type {
                #[inline(always)]
                fn into_bits(self) -> Bits<$type> {
                    Bits(self)
                }
            }

            impl FromBits<$type> for $type {
                #[inline(always)]
                fn from_bits(bits: Bits<$type>) -> Self {
                    bits.into_raw()
                }
            }
        )*
    }
}
impl_wrapper_from_naive!(bool, u8, u16, u32, u64, u128);
