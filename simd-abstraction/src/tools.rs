use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use crate::traits::{SIMD128, SIMD256};

pub struct OutBuf<'a, T> {
    base: *mut T,
    len: usize,
    _marker: PhantomData<&'a mut [MaybeUninit<T>]>,
}

unsafe impl<'a, T: Send> Send for OutBuf<'a, T> {}
unsafe impl<'a, T: Sync> Sync for OutBuf<'a, T> {}

impl<'a, T> OutBuf<'a, T> {
    #[inline]
    pub unsafe fn new(base: *mut T, len: usize) -> Self {
        Self {
            base,
            len,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_slice_mut(slice: &'a mut [T]) -> Self {
        let (base, len) = (slice.as_mut_ptr(), slice.len());
        unsafe { Self::new(base, len) }
    }
    #[inline]
    pub fn from_uninit_mut(slice: &'a mut [MaybeUninit<T>]) -> Self {
        let (base, len) = (slice.as_mut_ptr(), slice.len());
        unsafe { Self::new(base.cast(), len) }
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.base
    }
}

impl<T> fmt::Debug for OutBuf<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutBuf")
            .field("base", &self.base)
            .field("len", &self.len)
            .finish()
    }
}

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Bytes16(pub [u8; 16]);

#[derive(Debug)]
#[repr(C, align(32))]
pub struct Bytes32(pub [u8; 32]);

pub trait Load<T> {
    type Output;

    fn load(self, src: T) -> Self::Output;
}

impl<S: SIMD128> Load<&'_ Bytes16> for S {
    type Output = S::V128;

    #[inline(always)]
    fn load(self, src: &'_ Bytes16) -> Self::Output {
        unsafe { self.v128_load(src.0.as_ptr()) }
    }
}

impl<S: SIMD256> Load<&'_ Bytes32> for S {
    type Output = S::V256;

    #[inline(always)]
    fn load(self, src: &'_ Bytes32) -> Self::Output {
        unsafe { self.v256_load(src.0.as_ptr()) }
    }
}

#[allow(unused_macros)]
macro_rules! debug_assert_ptr_align {
    ($ptr:expr, $align:literal) => {{
        let align: usize = $align;
        let ptr = $ptr as *const _ as *const ();
        let addr = ptr as usize;
        debug_assert!(addr % align == 0)
    }};
}
