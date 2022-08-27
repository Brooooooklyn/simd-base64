use crate::isa::{InstructionSet, SimdLoad, SIMD128, SIMD256, SIMD512};

#[cfg(target_arch = "arm")]
use core::arch::arm::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "arm")]
define_isa!(NEON, "neon");

#[cfg(target_arch = "aarch64")]
define_isa!(NEON, "neon");

#[cfg(target_arch = "aarch64")]
define_isa!(CRC32, "crc");

unsafe impl SIMD128 for NEON {
    type V128 = uint8x16_t;

    #[inline(always)]
    fn v128_to_bytes(self, a: Self::V128) -> [u8; 16] {
        unsafe { core::mem::transmute(a) }
    }

    #[inline(always)]
    unsafe fn v128_load(self, addr: *const u8) -> Self::V128 {
        debug_assert_ptr_align!(addr, 16);
        vld1q_u8(addr)
    }

    #[inline(always)]
    unsafe fn v128_load_unaligned(self, addr: *const u8) -> Self::V128 {
        vld1q_u8(addr)
    }

    #[inline(always)]
    unsafe fn v128_store(self, addr: *mut u8, a: Self::V128) {
        debug_assert_ptr_align!(addr, 16);
        vst1q_u8(addr, a)
    }

    #[inline(always)]
    unsafe fn v128_store_unaligned(self, addr: *mut u8, a: Self::V128) {
        vst1q_u8(addr, a)
    }

    #[inline(always)]
    fn v128_create_zero(self) -> Self::V128 {
        unsafe { vdupq_n_u8(0) }
    }

    #[inline(always)]
    fn v128_not(self, a: Self::V128) -> Self::V128 {
        unsafe { vmvnq_u8(a) }
    }

    #[inline(always)]
    fn v128_and(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vandq_u8(a, b) }
    }

    #[inline(always)]
    fn v128_or(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vorrq_u8(a, b) }
    }

    #[inline(always)]
    fn v128_xor(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { veorq_u8(a, b) }
    }

    #[inline(always)]
    fn v128_andnot(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        self.v128_and(a, self.v128_not(b))
    }

    #[inline(always)]
    fn v128_all_zero(self, a: Self::V128) -> bool {
        #[cfg(target_arch = "arm")]
        unsafe {
            let a0 = vreinterpretq_u64_u8(a);
            let a1 = vorr_u64(vget_low_u64(a0), vget_high_u64(a0));
            vget_lane_u64::<0>(a1) == 0
        }
        #[cfg(target_arch = "aarch64")]
        unsafe {
            vmaxvq_u8(a) == 0
        }
    }

    #[inline(always)]
    fn u8x16_splat(self, x: u8) -> Self::V128 {
        unsafe { vdupq_n_u8(x) }
    }

    #[inline(always)]
    fn u16x8_splat(self, x: u16) -> Self::V128 {
        unsafe { vreinterpretq_u8_u16(vdupq_n_u16(x)) }
    }

    #[inline(always)]
    fn u32x4_splat(self, x: u32) -> Self::V128 {
        unsafe { vreinterpretq_u8_u32(vdupq_n_u32(x)) }
    }

    #[inline(always)]
    fn u64x2_splat(self, x: u64) -> Self::V128 {
        unsafe { vreinterpretq_u8_u64(vdupq_n_u64(x)) }
    }

    #[inline(always)]
    fn i8x16_splat(self, x: i8) -> Self::V128 {
        unsafe { vreinterpretq_u8_s8(vdupq_n_s8(x)) }
    }

    #[inline(always)]
    fn i16x8_splat(self, x: i16) -> Self::V128 {
        unsafe { vreinterpretq_u8_s16(vdupq_n_s16(x)) }
    }

    #[inline(always)]
    fn i32x4_splat(self, x: i32) -> Self::V128 {
        unsafe { vreinterpretq_u8_s32(vdupq_n_s32(x)) }
    }

    #[inline(always)]
    fn i64x2_splat(self, x: i64) -> Self::V128 {
        unsafe { vreinterpretq_u8_s64(vdupq_n_s64(x)) }
    }

    #[inline(always)]
    fn u8x16_add(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vaddq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_add(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vaddq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_add(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vaddq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u64x2_add(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u64_u8;
            let g = vreinterpretq_u8_u64;
            g(vaddq_u64(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u8x16_sub(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vsubq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_sub(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vsubq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_sub(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vsubq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u64x2_sub(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u64_u8;
            let g = vreinterpretq_u8_u64;
            g(vsubq_u64(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u8x16_sub_sat(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vqsubq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_sub_sat(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vqsubq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i8x16_sub_sat(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s8_u8;
            let g = vreinterpretq_u8_s8;
            g(vqsubq_s8(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i16x8_sub_sat(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s16_u8;
            let g = vreinterpretq_u8_s16;
            g(vqsubq_s16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i16x8_mul_lo(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s16_u8;
            let g = vreinterpretq_u8_s16;
            g(vmulq_s16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i32x4_mul_lo(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s32_u8;
            let g = vreinterpretq_u8_s32;
            g(vmulq_s32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u16x8_shl<const IMM8: i32>(self, a: Self::V128) -> Self::V128 {
        unsafe { vreinterpretq_u8_u16(vshlq_n_u16::<IMM8>(vreinterpretq_u16_u8(a))) }
    }

    #[inline(always)]
    fn u32x4_shl<const IMM8: i32>(self, a: Self::V128) -> Self::V128 {
        unsafe { vreinterpretq_u8_u32(vshlq_n_u32::<IMM8>(vreinterpretq_u32_u8(a))) }
    }

    #[inline(always)]
    fn u16x8_shr<const IMM8: i32>(self, a: Self::V128) -> Self::V128 {
        unsafe { vreinterpretq_u8_u16(vshrq_n_u16::<IMM8>(vreinterpretq_u16_u8(a))) }
    }

    #[inline(always)]
    fn u32x4_shr<const IMM8: i32>(self, a: Self::V128) -> Self::V128 {
        unsafe { vreinterpretq_u8_u32(vshrq_n_u32::<IMM8>(vreinterpretq_u32_u8(a))) }
    }

    #[inline(always)]
    fn u8x16_eq(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vceqq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_eq(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vceqq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_eq(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vceqq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u8x16_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vcltq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vcltq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vcltq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i8x16_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s8_u8;
            vcltq_s8(f(a), f(b))
        }
    }

    #[inline(always)]
    fn i16x8_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s16_u8;
            let g = vreinterpretq_u8_u16;
            g(vcltq_s16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i32x4_lt(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s32_u8;
            let g = vreinterpretq_u8_u32;
            g(vcltq_s32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u8x16_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vmaxq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vmaxq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vmaxq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i8x16_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s8_u8;
            let g = vreinterpretq_u8_s8;
            g(vmaxq_s8(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i16x8_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s16_u8;
            let g = vreinterpretq_u8_s16;
            g(vmaxq_s16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i32x4_max(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s32_u8;
            let g = vreinterpretq_u8_s32;
            g(vmaxq_s32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u8x16_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe { vminq_u8(a, b) }
    }

    #[inline(always)]
    fn u16x8_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vminq_u16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u32x4_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vminq_u32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i8x16_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s8_u8;
            let g = vreinterpretq_u8_s8;
            g(vminq_s8(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i16x8_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s16_u8;
            let g = vreinterpretq_u8_s16;
            g(vminq_s16(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn i32x4_min(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_s32_u8;
            let g = vreinterpretq_u8_s32;
            g(vminq_s32(f(a), f(b)))
        }
    }

    #[inline(always)]
    fn u16x8_bswap(self, a: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u16_u8;
            let g = vreinterpretq_u8_u16;
            g(vrev64q_u16(f(a)))
        }
    }

    #[inline(always)]
    fn u32x4_bswap(self, a: Self::V128) -> Self::V128 {
        unsafe {
            let f = vreinterpretq_u32_u8;
            let g = vreinterpretq_u8_u32;
            g(vrev64q_u32(f(a)))
        }
    }

    #[inline(always)]
    fn u64x2_bswap(self, a: Self::V128) -> Self::V128 {
        self.u8x16_swizzle(a, self.load(crate::common::bswap::SHUFFLE_U64X2))
    }

    #[inline(always)]
    fn u8x16_swizzle(self, a: Self::V128, b: Self::V128) -> Self::V128 {
        #[cfg(target_arch = "arm")]
        unsafe {
            let a = uint8x8x2_t(vget_low_u8(a), vget_high_u8(a));
            let (b0, b1) = (vget_low_u8(b), vget_high_u8(b));
            let c0 = vget_lane_u64::<0>(vreinterpret_u64_u8(vtbl2_u8(a, b0)));
            let c1 = vget_lane_u64::<0>(vreinterpret_u64_u8(vtbl2_u8(a, b1)));
            vreinterpretq_u8_u64(vsetq_lane_u64::<1>(c1, vdupq_n_u64(c0)))
        }
        #[cfg(target_arch = "aarch64")]
        unsafe {
            vqtbl1q_u8(a, b)
        }
    }

    #[inline(always)]
    fn u8x16_any_zero(self, a: Self::V128) -> bool {
        #[cfg(target_arch = "arm")]
        {
            let zero = self.v128_create_zero();
            let cmp = self.u8x16_eq(a, zero);
            !self.v128_all_zero(cmp)
        }
        #[cfg(target_arch = "aarch64")]
        unsafe {
            vminvq_u8(a) == 0
        }
    }
}

unsafe impl SIMD256 for NEON {
    type V256 = uint8x16x2_t;

    #[inline(always)]
    fn v256_from_v128x2(self, a: Self::V128, b: Self::V128) -> Self::V256 {
        uint8x16x2_t(a, b)
    }

    #[inline(always)]
    fn v256_to_v128x2(self, a: Self::V256) -> (Self::V128, Self::V128) {
        (a.0, a.1)
    }

    #[inline(always)]
    fn v256_to_bytes(self, a: Self::V256) -> [u8; 32] {
        unsafe { core::mem::transmute([a.0, a.1]) }
    }

    #[inline(always)]
    unsafe fn v256_load(self, addr: *const u8) -> Self::V256 {
        debug_assert_ptr_align!(addr, 32);
        vld1q_u8_x2(addr)
    }

    #[inline(always)]
    unsafe fn v256_load_unaligned(self, addr: *const u8) -> Self::V256 {
        vld1q_u8_x2(addr)
    }

    #[inline(always)]
    unsafe fn v256_store(self, addr: *mut u8, a: Self::V256) {
        debug_assert_ptr_align!(addr, 32);
        vst1q_u8_x2(addr, a)
    }

    #[inline(always)]
    unsafe fn v256_store_unaligned(self, addr: *mut u8, a: Self::V256) {
        vst1q_u8_x2(addr, a)
    }
}

unsafe impl SIMD512 for NEON {
    type V512 = uint8x16x4_t;

    #[inline(always)]
    fn v512_from_v256x2(self, a: Self::V256, b: Self::V256) -> Self::V512 {
        uint8x16x4_t(a.0, a.1, b.0, b.1)
    }

    #[inline(always)]
    fn v512_to_v256x2(self, a: Self::V512) -> (Self::V256, Self::V256) {
        (uint8x16x2_t(a.0, a.1), uint8x16x2_t(a.2, a.3))
    }

    #[inline(always)]
    fn v512_to_bytes(self, a: Self::V512) -> [u8; 64] {
        unsafe { core::mem::transmute([a.0, a.1, a.2, a.3]) }
    }

    #[inline(always)]
    unsafe fn v512_load(self, addr: *const u8) -> Self::V512 {
        debug_assert_ptr_align!(addr, 32);
        vld1q_u8_x4(addr)
    }

    #[inline(always)]
    unsafe fn v512_load_unaligned(self, addr: *const u8) -> Self::V512 {
        vld1q_u8_x4(addr)
    }

    #[inline(always)]
    unsafe fn v512_store(self, addr: *mut u8, a: Self::V512) {
        debug_assert_ptr_align!(addr, 32);
        vst1q_u8_x4(addr, a)
    }

    #[inline(always)]
    unsafe fn v512_store_unaligned(self, addr: *mut u8, a: Self::V512) {
        vst1q_u8_x4(addr, a)
    }
}

impl NEON {
    #[inline(always)]
    pub fn v128_bsl(self, a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
        unsafe { vbslq_u8(a, b, c) }
    }

    #[inline(always)]
    pub fn v256_bsl(self, a: uint8x16x2_t, b: uint8x16x2_t, c: uint8x16x2_t) -> uint8x16x2_t {
        let d0 = self.v128_bsl(a.0, b.0, c.0);
        let d1 = self.v128_bsl(a.1, b.1, c.1);
        uint8x16x2_t(d0, d1)
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub fn u8x32_swizzle(self, a: uint8x16x2_t, b: uint8x16x2_t) -> uint8x16x2_t {
        unsafe {
            let c0 = vqtbl2q_u8(a, b.0);
            let c1 = vqtbl2q_u8(a, b.1);
            uint8x16x2_t(c0, c1)
        }
    }
}
