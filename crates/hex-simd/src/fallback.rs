use crate::Error;

use vsimd::ascii::AsciiCase;
use vsimd::hex::unhex;
use vsimd::tools::{read, slice_parts};

#[inline(always)]
pub fn check(data: &[u8]) -> Result<(), Error> {
    unsafe {
        let (mut src, mut len) = slice_parts(data);

        let end = src.add(len / 4 * 4);
        while src < end {
            let y1 = unhex(read(src, 0));
            let y2 = unhex(read(src, 1));
            let y3 = unhex(read(src, 2));
            let y4 = unhex(read(src, 3));
            ensure!((y1 | y2 | y3 | y4) != 0xff);
            src = src.add(4);
        }
        len %= 4;

        let mut flag = 0;
        let end = src.add(len);
        while src < end {
            flag |= unhex(read(src, 0));
            src = src.add(1);
        }
        ensure!(flag != 0xff);
    }

    Ok(())
}

#[inline]
const fn full_table(table: &[u8; 16]) -> [u16; 256] {
    let mut buf = [0; 256];
    let mut i = 0;
    while i < 256 {
        let hi = table[i >> 4];
        let lo = table[i & 0xf];
        buf[i] = u16::from_ne_bytes([hi, lo]);
        i += 1;
    }
    buf
}

const UPPER_TABLE: &[u8; 16] = b"0123456789ABCDEF";
const LOWER_TABLE: &[u8; 16] = b"0123456789abcdef";

pub const FULL_LOWER_TABLE: &[u16; 256] = &full_table(LOWER_TABLE);
pub const FULL_UPPER_TABLE: &[u16; 256] = &full_table(UPPER_TABLE);

#[inline(always)]
unsafe fn encode_bits(src: *const u8, dst: *mut u8, table: *const u16) {
    let x = src.read();
    let y = read(table, x as usize);
    dst.cast::<u16>().write_unaligned(y);
}

#[inline(always)]
pub unsafe fn encode(src: &[u8], mut dst: *mut u8, case: AsciiCase) {
    let table = match case {
        AsciiCase::Lower => FULL_LOWER_TABLE.as_ptr(),
        AsciiCase::Upper => FULL_UPPER_TABLE.as_ptr(),
    };
    let (mut src, len) = (src.as_ptr(), src.len());

    let end = src.add(len / 8 * 8);
    while src < end {
        let mut i = 0;
        while i < 8 {
            encode_bits(src, dst, table);
            src = src.add(1);
            dst = dst.add(2);
            i += 1;
        }
    }
    encode_short(src, len % 8, dst, table);
}

#[inline(always)]
pub unsafe fn encode_short(mut src: *const u8, len: usize, mut dst: *mut u8, table: *const u16) {
    let end = src.add(len);
    while src < end {
        encode_bits(src, dst, table);
        src = src.add(1);
        dst = dst.add(2);
    }
}

#[inline(always)]
fn shl4(x: u8) -> u8 {
    x.wrapping_shl(4)
}

#[inline(always)]
unsafe fn decode_bits(src: *const u8, dst: *mut u8) -> Result<(), Error> {
    let y1 = unhex(read(src, 0));
    let y2 = unhex(read(src, 1));
    ensure!((y1 | y2) != 0xff);
    let z = shl4(y1) | y2;
    dst.write(z);
    Ok(())
}

#[inline(always)]
pub unsafe fn decode(mut src: *const u8, len: usize, mut dst: *mut u8) -> Result<(), Error> {
    let end = src.add(len / 16 * 16);
    while src < end {
        let mut i = 0;
        while i < 8 {
            decode_bits(src, dst)?;
            src = src.add(2);
            dst = dst.add(1);
            i += 1;
        }
    }
    decode_short(src, len % 16, dst)
}

#[inline(always)]
pub unsafe fn decode_short(mut src: *const u8, len: usize, mut dst: *mut u8) -> Result<(), Error> {
    let end = src.add(len);
    while src < end {
        decode_bits(src, dst)?;
        src = src.add(2);
        dst = dst.add(1);
    }
    Ok(())
}
