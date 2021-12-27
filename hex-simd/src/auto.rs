use crate::{AsciiCase, Error, OutBuf};

macro_rules! try_simd {
    ($f:ident($($args:tt)*)) => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use simd_abstraction::traits::InstructionSet;
            use simd_abstraction::arch::x86::*;
            if AVX2::detect().is_some() {
                return unsafe { $crate::arch::x86::avx2::$f($($args)*) };
            }
            if SSE41::detect().is_some() {
                return unsafe { $crate::arch::x86::sse41::$f($($args)*) };
            }
        }
        #[cfg(all(
            feature="unstable",
            any(target_arch = "arm", target_arch="aarch64")
        ))]
        {
            use simd_abstraction::traits::InstructionSet;

            #[cfg(target_arch="arm")]
            use simd_abstraction::arch::arm::*;

            #[cfg(target_arch="aarch64")]
            use simd_abstraction::arch::aarch64::*;

            if NEON::detect().is_some() {
                return unsafe { $crate::arch::arm::neon::$f($($args)*) };
            }
        }
    };
}

#[inline]
pub fn check(src: &[u8]) -> bool {
    try_simd!(check(src));
    crate::fallback::check(src)
}

#[inline]
pub fn encode<'s, 'd>(
    src: &'s [u8],
    dst: OutBuf<'d, u8>,
    case: AsciiCase,
) -> Result<&'d mut [u8], Error> {
    try_simd!(encode(src, dst, case));
    crate::fallback::encode(src, dst, case)
}

#[inline]
pub fn decode<'s, 'd>(src: &'s [u8], dst: OutBuf<'d, u8>) -> Result<&'d mut [u8], Error> {
    try_simd!(decode(src, dst));
    crate::fallback::decode(src, dst)
}

#[inline]
pub fn decode_inplace(buf: &mut [u8]) -> Result<&mut [u8], Error> {
    try_simd!(decode_inplace(buf));
    crate::fallback::decode_inplace(buf)
}
