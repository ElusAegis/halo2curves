use proc_macro2::TokenStream;

mod aarch64;
mod x86_64;

/// TODO - more limbs make the operations quicker. The original version has 4 limbs. I assume that
/// the idea was to provide other versions with more limbs to speed up the operations.
/// So for the architecture should consider the number of limbs to use.

pub(crate) fn impl_arith(field: &syn::Ident, num_limbs: usize, inv: u64) -> TokenStream {
    let impl_add_asm_x86_64 = x86_64::impl_add_asm();
    let impl_add_asm_aarch64 = aarch64::impl_add_asm();

    let impl_sub_asm_x86_64 = x86_64::impl_sub_asm();
    let impl_sub_asm_aarch64 = aarch64::impl_sub_asm();

    let impl_neg_asm_x86_64 = x86_64::impl_neg_asm();
    let impl_neg_asm_aarch64 = aarch64::impl_neg_asm();

    let impl_double_asm_x86_64 = x86_64::impl_double_asm();
    let impl_double_asm_aarch64 = aarch64::impl_double_asm();

    let impl_from_mont_asm_x86_64 = x86_64::impl_from_mont_asm();
    let impl_from_mont_asm_aarch64 = aarch64::impl_from_mont_asm();

    let impl_mul_asm_x86_64 = x86_64::impl_mul_asm();
    let impl_mul_asm_aarch64 = aarch64::impl_mul_asm();

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        compile_error!("Unsupported architecture for optimized field arithmetic.");
        // TODO - consider using non-asm version as backup
    }

    let impl_add = crate::field::arith::impl_add(field, num_limbs);
    let impl_sub = crate::field::arith::impl_sub(field, num_limbs);
    let impl_neg = crate::field::arith::impl_neg(field, num_limbs);
    let impl_mont = crate::field::arith::impl_mont(field, num_limbs, inv);
    let impl_from_mont = crate::field::arith::impl_from_mont(field, num_limbs, inv);
    let impl_mul = crate::field::arith::impl_mul(field, num_limbs, false);
    let impl_square = crate::field::arith::impl_square(field, num_limbs);
    let wide_num_limbs = num_limbs * 2;

    quote::quote! {
        use std::arch::asm;
        impl #field {

            #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
            compile_error!("Unsupported architecture for optimized field arithmetic. Only `x86_64` and `aarch64` are supported.");

            /// Adds `rhs` to `self`, returning the result.
            #[inline]
            pub fn add(&self, rhs: &Self) -> #field {

                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let b_ptr: *const u64 = rhs.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_add_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_add_asm_aarch64
                }

                let res_asm = #field([r0, r1, r2, r3]);


                // TODO - remove this check in production
                let res_naive = {#impl_add};
                if res_asm != res_naive {
                    panic!("Bug in ASM: {:?} (asm) != {:?} (expected)", res_asm, res_naive);
                }

                res_asm
            }

            /// Subtracts `rhs` from `self`, returning the result.
            #[inline]
            pub fn sub(&self, rhs: &Self) -> #field {
                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let b_ptr: *const u64 = rhs.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_sub_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_sub_asm_aarch64
                }

                let res_asm = #field([r0, r1, r2, r3]);

                // TODO - remove this check in production
                let res_naive = {#impl_sub};
                if res_asm != res_naive {
                    panic!("Bug in ASM: {:?} (asm) != {:?} (expected)", res_asm, res_naive);
                }

                res_asm
            }

            /// Negates `self`.
            #[inline]
            pub fn neg(&self) -> #field {
                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_neg_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_neg_asm_aarch64
                }

                let res_asm = #field([r0, r1, r2, r3]);

                // TODO - remove this check in production
                let res_naive = {#impl_neg};
                if res_asm != res_naive {
                    panic!("Bug in ASM: {:?} (asm) != {:?} (expected)", res_asm, res_naive);
                }

                res_asm
            }

            /// Doubles this field element.
            #[inline]
            pub fn double(&self) -> #field {
                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_double_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_double_asm_aarch64
                }

                #field([r0, r1, r2, r3])
            }

            #[inline(always)]
            pub fn mul(&self, rhs: &Self) -> Self{
                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let b_ptr: *const u64 = rhs.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();
                let inv = #inv as u64;

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_mul_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_mul_asm_aarch64
                }

                let res_asm = #field([r0, r1, r2, r3]);

                // TODO - remove this check in production
                let res_naive = {#impl_mul};
                if res_asm != res_naive {
                    println!("Rust impl: {:?}", #field::montgomery_mul(&self.0, &rhs.0, &#field::MODULUS_LIMBS, #inv));
                    panic!("Bug in ASM: {:?} (asm) != {:?} (expected)\n\nRust Implementation Output: {:?}", res_asm, res_naive, #field::montgomery_mul(&self.0, &rhs.0, &#field::MODULUS_LIMBS, #inv));
                }

                res_asm
            }

            #[inline(always)]
            pub fn square(&self) -> Self{
                self.mul(self)
            }

            // TODO - remove in production
            #[inline(always)]
            pub(crate) fn montgomery_reduce(r: &[u64; #wide_num_limbs]) -> Self {
                #impl_mont
            }

            /// Converts this field element from Montgomery form back to standard representation.
            #[inline(always)]
            pub(crate) fn from_mont(&self) -> [u64; Self::NUM_LIMBS] {
                let mut r0: u64;
                let mut r1: u64;
                let mut r2: u64;
                let mut r3: u64;

                let a_ptr: *const u64 = self.0.as_ptr();
                let m_ptr: *const u64 = #field::MODULUS_LIMBS.as_ptr();
                let inv = #inv as u64;

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    #impl_from_mont_asm_x86_64

                    #[cfg(target_arch = "aarch64")]
                    #impl_from_mont_asm_aarch64
                }

                let res_asm =[r0, r1, r2, r3];

                // TODO - remove this check in production
                let res_naive = {#impl_from_mont};
                if res_asm != res_naive {
                    panic!("Bug in ASM: {:?} (asm) != {:?} (expected)", res_asm, res_naive);
                }

                res_asm
            }

            pub fn montgomery_mul(a: &[u64; 4], b: &[u64; 4], m: &[u64; 4], inv: u64) -> [u64; 4] {
                // Initialize the result array t[0..5]
                let mut t = [0u64; 5]; // Extra limb for carry handling

                // For each limb of b (i from 0 to 3)
                for i in 0..4 {
                    let b_i = b[i];
                    let mut A = 0u64;

                    // Multiplication step: t[j] += a[j] * b_i + A
                    for j in 0..4 {
                        let (mul_low, mul_high) = Self::mul_with_carry(a[j], b_i, A);
                        let (sum, carry) = Self::add_with_carry(t[j], mul_low, 0);
                        t[j] = sum;
                        A = mul_high + carry;
                    }
                    // t[4] holds the carry A
                    t[4] = A;

                    // Reduction step
                    // m_coeff = t[0] * inv mod 2^64
                    let m_coeff = t[0].wrapping_mul(inv);

                    // Compute t[0] + m_coeff * m[0]
                    let (mul_low, mul_high) = Self::mul_with_carry(m_coeff, m[0], 0);
                    let (sum, carry) = Self::add_with_carry(t[0], mul_low, 0);
                    // Discard t[0]; it will be overwritten in the next iteration
                    let mut C = mul_high + carry;

                    // For j from 1 to 3
                    for j in 1..4 {
                        let (mul_low, mul_high) = Self::mul_with_carry(m_coeff, m[j], 0);
                        let (sum, carry) = Self::add_with_carry(t[j], mul_low, C);
                        t[j - 1] = sum; // Shifted down by one position
                        C = mul_high + carry;
                    }

                    // t[3] = t[4] + C
                    let (sum, _) = Self::add_with_carry(t[4], C, 0);
                    t[3] = sum; // Final limb after reduction
                }

                // Final subtraction: if t >= m, subtract m
                let mut result = [0u64; 4];
                let mut borrow = 0u64;
                for i in 0..4 {
                    let (res, b) = Self::sub_with_borrow(t[i], m[i], borrow);
                    result[i] = res;
                    borrow = b;
                }

                // If borrow is zero, result = t - m
                // If borrow is non-zero, result = t[0..3]
                if borrow == 0 {
                    result
                } else {
                    // Result is t[0..3]
                    result.copy_from_slice(&t[0..4]);
                    result
                }
            }

            // Helper functions for arithmetic operations with carry and borrow
            fn mul_with_carry(a: u64, b: u64, carry_in: u64) -> (u64, u64) {
                let product = (a as u128) * (b as u128) + (carry_in as u128);
                let low = product as u64;
                let high = (product >> 64) as u64;
                (low, high)
            }

            fn add_with_carry(a: u64, b: u64, carry_in: u64) -> (u64, u64) {
                let sum = (a as u128) + (b as u128) + (carry_in as u128);
                let low = sum as u64;
                let carry_out = (sum >> 64) as u64;
                (low, carry_out)
            }

            fn sub_with_borrow(a: u64, b: u64, borrow_in: u64) -> (u64, u64) {
                let (res1, overflow1) = a.overflowing_sub(borrow_in);
                let (res2, overflow2) = res1.overflowing_sub(b);
                let borrow_out = (overflow1 as u64) + (overflow2 as u64);
                (res2, borrow_out)
            }


        }

    }
}
