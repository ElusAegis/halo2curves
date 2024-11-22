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

            #[inline(always)]
            pub fn neg(&self) -> Self {
                #impl_neg
            }

            #[inline(always)]
            pub fn mul(&self, rhs: &Self) -> Self{
                #impl_mul
            }

            #[inline(always)]
            pub fn square(&self) -> Self{
                #impl_square
            }

            #[inline(always)]
            pub(crate) fn montgomery_reduce(r: &[u64; #wide_num_limbs]) -> Self {
                #impl_mont
            }

            #[inline(always)]
            pub(crate) fn from_mont(&self) -> [u64; #num_limbs] {
                #impl_from_mont
            }


        }
    }
}
