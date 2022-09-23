use crate::Error;

use vsimd::hex::unhex;
use vsimd::tools::slice;
use vsimd::SIMD256;

#[inline]
pub fn check_fallback(data: &[u8]) -> Result<(), Error> {
    let mut iter = data.chunks_exact(4);
    for chunk in &mut iter {
        let y1 = unhex(chunk[0]);
        let y2 = unhex(chunk[1]);
        let y3 = unhex(chunk[2]);
        let y4 = unhex(chunk[3]);
        ensure!((y1 | y2 | y3 | y4) != 0xff);
    }
    let flag = iter.remainder().iter().fold(0, |acc, &x| acc | unhex(x));
    ensure!(flag != 0xff);
    Ok(())
}

#[inline]
pub fn check_simd<S: SIMD256>(s: S, data: &[u8]) -> Result<(), Error> {
    unsafe {
        let (mut src, mut len) = (data.as_ptr(), data.len());

        if len == 16 {
            let x = s.v128_load_unaligned(src);
            ensure!(vsimd::hex::check_ascii_xn(s, x));
            return Ok(());
        }

        if len == 32 {
            let x = s.v256_load_unaligned(src);
            ensure!(vsimd::hex::check_ascii_xn(s, x));
            return Ok(());
        }

        let end = src.add(len / 32 * 32);
        while src < end {
            let x = s.v256_load_unaligned(src);
            ensure!(vsimd::hex::check_ascii_xn(s, x));
            src = src.add(32);
        }
        len %= 32;

        if len == 0 {
            return Ok(());
        }

        if len >= 16 {
            let x = s.v128_load_unaligned(src);
            ensure!(vsimd::hex::check_ascii_xn(s, x));
            len -= 16;
            src = src.add(16);
        }

        check_fallback(slice(src, len))
    }
}
