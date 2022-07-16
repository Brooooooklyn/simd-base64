use crate::{sa_ascii, Base64, Error, OutBuf};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use simd_abstraction::tools::alloc_uninit_bytes;

impl Base64 {
    /// Encodes `src` and returns [`Box<str>`]
    ///
    /// # Panics
    /// This function panics if:
    ///
    /// + The encoded length of `src` is greater than `isize::MAX`
    ///
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn encode_to_boxed_str(&self, src: &[u8]) -> Box<str> {
        use core::{slice, str};

        if src.is_empty() {
            return Box::from("");
        }

        unsafe {
            let m = Self::encoded_length_unchecked(src.len(), self.padding);
            assert!(m <= (isize::MAX as usize));
            let mut uninit_buf = alloc_uninit_bytes(m);
            Self::encode(self, src, OutBuf::uninit(&mut uninit_buf)).unwrap();

            let ptr = Box::into_raw(uninit_buf).cast::<u8>();
            let buf = slice::from_raw_parts_mut(ptr, m);
            Box::from_raw(str::from_utf8_unchecked_mut(buf))
        }
    }

    /// Decodes `src` and returns [`Box<[u8]>`](Box)
    ///
    /// # Errors
    /// This function returns `Err` if:
    ///
    /// + The content of `src` is invalid.
    ///
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_to_boxed_bytes(&self, src: &[u8]) -> Result<Box<[u8]>, Error> {
        use core::slice;

        if src.is_empty() {
            return Ok(Box::from([]));
        }
        unsafe {
            let (_, m) = Self::decoded_length_unchecked(src, self.padding)?;

            // safety: 0 < m < isize::MAX
            let mut uninit_buf = alloc_uninit_bytes(m);
            Self::decode(self, src, OutBuf::uninit(&mut uninit_buf))?;

            let ptr = Box::into_raw(uninit_buf).cast::<u8>();
            let buf = slice::from_raw_parts_mut(ptr, m);
            Ok(Box::from_raw(buf))
        }
    }

    /// Forgiving decodes `buf` and writes inplace.
    ///
    /// See <https://infra.spec.whatwg.org/#forgiving-base64>
    #[inline]
    pub fn forgiving_decode_inplace(buf: &mut [u8]) -> Result<&mut [u8], Error> {
        let buf = forgiving_fix_data(buf);
        Self::STANDARD_NO_PAD.decode_inplace(buf)
    }
}

#[inline(always)]
fn find_non_ascii_whitespace(data: &[u8]) -> usize {
    sa_ascii::multiversion::find_non_ascii_whitespace::auto_indirect(data)
}

#[inline(always)]
fn remove_ascii_whitespace(data: &mut [u8]) -> &mut [u8] {
    let non_aw_pos = find_non_ascii_whitespace(data);
    if non_aw_pos >= data.len() {
        return data;
    }

    unsafe {
        let dirty_len = data.len() - non_aw_pos;
        let dirty_data = data.as_mut_ptr().add(non_aw_pos);

        let clean_len = sa_ascii::remove_ascii_whitespace_raw_fallback(dirty_data, dirty_len);

        data.get_unchecked_mut(..(non_aw_pos + clean_len))
    }
}

const fn forgiving_discard_table(mask: u8) -> [u8; 256] {
    let charset = crate::fallback::STANDARD_CHARSET;
    let mut table = [0; 256];

    let mut i = 0;
    loop {
        table[i as usize] = i;
        if i == 255 {
            break;
        }
        i += 1;
    }

    let mut i = 0;
    while i < 64 {
        table[charset[i] as usize] = charset[i & mask as usize];
        i += 1;
    }
    table
}

#[inline(always)]
fn forgiving_discard4(ch: &mut u8) {
    const TABLE: &[u8; 256] = &forgiving_discard_table(0xf0);
    unsafe { *ch = *TABLE.get_unchecked(*ch as usize) }
}

#[inline(always)]
fn forgiving_discard2(ch: &mut u8) {
    const TABLE: &[u8; 256] = &forgiving_discard_table(0xfc);
    unsafe { *ch = *TABLE.get_unchecked(*ch as usize) }
}

#[inline(always)]
fn forgiving_fix_data(buf: &mut [u8]) -> &mut [u8] {
    let buf = remove_ascii_whitespace(buf);

    if buf.is_empty() {
        return buf;
    }

    unsafe {
        let len = buf.len();
        match len % 4 {
            0 => {
                let x1 = *buf.get_unchecked(len - 1);
                let x2 = *buf.get_unchecked(len - 2);
                if x1 == b'=' {
                    if x2 == b'=' {
                        let last3 = buf.get_unchecked_mut(len - 3);
                        forgiving_discard4(last3);
                        buf.get_unchecked_mut(..len - 2)
                    } else {
                        let last2 = buf.get_unchecked_mut(len - 2);
                        forgiving_discard2(last2);
                        buf.get_unchecked_mut(..len - 1)
                    }
                } else {
                    buf
                }
            }
            1 => buf,
            2 => {
                let last1 = buf.get_unchecked_mut(len - 1);
                forgiving_discard4(last1);
                buf
            }
            3 => {
                let last1 = buf.get_unchecked_mut(len - 1);
                forgiving_discard2(last1);
                buf
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

#[test]
fn test_forgiving() {
    let inputs = ["ab", "abc", "abcd"];
    let outputs: &[&[u8]] = &[&[105], &[105, 183], &[105, 183, 29]];

    for i in 0..inputs.len() {
        let (src, expected) = (inputs[i], outputs[i]);
        let mut buf = src.to_owned().into_bytes();
        let ans = Base64::forgiving_decode_inplace(&mut buf).unwrap();
        assert_eq!(ans, expected, "src = {:?}, expected = {:?}", src, expected);
    }
}

#[test]
fn test_remove_ascii_whitespace() {
    let cases = [
        "abcd",
        "ab\tcd",
        "ab\ncd",
        "ab\x0Ccd",
        "ab\rcd",
        "ab cd",
        "ab\t\n\x0C\r cd",
        "ab\t\n\x0C\r =\t\n\x0C\r =\t\n\x0C\r ",
    ];
    for case in cases {
        let mut buf = case.to_owned().into_bytes();
        let expected = {
            let mut v = buf.clone();
            v.retain(|c| !c.is_ascii_whitespace());
            v
        };
        let ans = remove_ascii_whitespace(&mut buf);
        assert_eq!(ans, &*expected, "case = {:?}", case);
    }
}
