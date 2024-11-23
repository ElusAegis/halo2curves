use proc_macro2::TokenStream;
// TODO - optimise with lateout
// TODO - try to optimise register allocation and see if it improves performance

pub(super) fn impl_add_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(
            // Load 'a' array into registers
            "ldr {a0}, [{a_ptr}, #0]",
            "ldr {a1}, [{a_ptr}, #8]",
            "ldr {a2}, [{a_ptr}, #16]",
            "ldr {a3}, [{a_ptr}, #24]",

            // Load 'b' array into registers of 'r'
            "ldr {r0}, [{b_ptr}, #0]",
            "ldr {r1}, [{b_ptr}, #8]",
            "ldr {r2}, [{b_ptr}, #16]",
            "ldr {r3}, [{b_ptr}, #24]",

            // Add 'a' and 'b' with carry propagation
            "adds {a0}, {a0}, {r0}",   // a0 = a0 + b0, sets flags
            "adcs {a1}, {a1}, {r1}",   // a1 = a1 + b1 + carry
            "adcs {a2}, {a2}, {r2}",
            "adcs {a3}, {a3}, {r3}",

            // Load 'm' array into registers for modular reduction
            "ldr {m0}, [{m_ptr}, #0]",
            "ldr {m1}, [{m_ptr}, #8]",
            "ldr {m2}, [{m_ptr}, #16]",
            "ldr {m3}, [{m_ptr}, #24]",

            // Subtract 'm' from the result with borrow propagation
            "subs {r0}, {a0}, {m0}",   // r0 = r0 - m0, sets flags
            "sbcs {r1}, {a1}, {m1}",   // r1 = r1 - m1 - borrow
            "sbcs {r2}, {a2}, {m2}",
            "sbcs {r3}, {a3}, {m3}",

            // Conditional select: if borrow occurred, use original values
            "csel {r0}, {a0}, {r0}, cc",  // cc = carry clear (borrow occurred)
            "csel {r1}, {a1}, {r1}, cc",
            "csel {r2}, {a2}, {r2}, cc",
            "csel {r3}, {a3}, {r3}, cc",

            // Outputs
            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            b_ptr = in(reg) b_ptr,
            // Output operands
            r0 = out(reg) r0,
            r1 = out(reg) r1,
            r2 = out(reg) r2,
            r3 = out(reg) r3,
            // Temporary (clobbered) registers
            a0 = out(reg) _,
            a1 = out(reg) _,
            a2 = out(reg) _,
            a3 = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}

pub(super) fn impl_sub_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(

            // Load 'a' array into temporary registers
            "ldr {a0}, [{a_ptr}, #0]",
            "ldr {a1}, [{a_ptr}, #8]",
            "ldr {a2}, [{a_ptr}, #16]",
            "ldr {a3}, [{a_ptr}, #24]",

            // Subtract 'b' array from 'a' array with borrow propagation
            "ldr {b0}, [{b_ptr}, #0]",
            "ldr {b1}, [{b_ptr}, #8]",
            "ldr {b2}, [{b_ptr}, #16]",
            "ldr {b3}, [{b_ptr}, #24]",

            "subs {a0}, {a0}, {b0}",  // a0 = a0 - b0, sets flags
            "sbcs {a1}, {a1}, {b1}",  // a1 = a1 - b1 - borrow
            "sbcs {a2}, {a2}, {b2}",
            "sbcs {a3}, {a3}, {b3}",

            // Load 'm' array into registers (modulus)
            "ldr {m0}, [{m_ptr}, #0]",
            "ldr {m1}, [{m_ptr}, #8]",
            "ldr {m2}, [{m_ptr}, #16]",
            "ldr {m3}, [{m_ptr}, #24]",

            // Mask: Use conditional selection to zero out modulus if borrow occurred
            "csel {m0}, xzr, {m0}, cs",  // cs = carry set (no borrow)
            "csel {m1}, xzr, {m1}, cs",
            "csel {m2}, xzr, {m2}, cs",
            "csel {m3}, xzr, {m3}, cs",

            // Add modulus to result if borrow occurred
            "adds {r0}, {a0}, {m0}",
            "adcs {r1}, {a1}, {m1}",
            "adcs {r2}, {a2}, {m2}",
            "adcs {r3}, {a3}, {m3}",

            // Outputs
            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            b_ptr = in(reg) b_ptr,
            // Output operands
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            // Temporary (clobbered) registers
            a0 = out(reg) _,
            a1 = out(reg) _,
            a2 = out(reg) _,
            a3 = out(reg) _,
            b0 = out(reg) _,
            b1 = out(reg) _,
            b2 = out(reg) _,
            b3 = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_neg_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(
            // Load 'm' array into registers
            "ldr {m0}, [{m_ptr}, #0]",
            "ldr {m1}, [{m_ptr}, #8]",
            "ldr {m2}, [{m_ptr}, #16]",
            "ldr {m3}, [{m_ptr}, #24]",

            // Subtract 'a' array from 'm' array with borrow propagation
            "ldr {a0}, [{a_ptr}, #0]",
            "ldr {a1}, [{a_ptr}, #8]",
            "ldr {a2}, [{a_ptr}, #16]",
            "ldr {a3}, [{a_ptr}, #24]",
            "subs {r0}, {m0}, {a0}",  // r0 = m0 - a0, sets flags
            "sbcs {r1}, {m1}, {a1}",  // r1 = m1 - a1 - borrow
            "sbcs {r2}, {m2}, {a2}",
            "sbcs {r3}, {m3}, {a3}",

            // Load 'a' array into temporary registers
            "ldr {t0}, [{a_ptr}, #0]",
            "ldr {t1}, [{a_ptr}, #8]",
            "ldr {t2}, [{a_ptr}, #16]",
            "ldr {t3}, [{a_ptr}, #24]",

            // Perform OR operations across the 'a' array
            "orr {t0}, {t0}, {t1}",
            "orr {t2}, {t2}, {t3}",
            "orr {t0}, {t0}, {t2}",

            // Check if result is zero and conditionally mask
            "mov {mask}, -1",            // mask = 0xffffffffffffffff
            "cmp {t0}, #0",              // Compare OR result with zero
            "csel {mask}, xzr, {mask}, eq",  // mask = 0 if zero, -1 otherwise

            // Apply mask to the result
            "and {r0}, {r0}, {mask}",
            "and {r1}, {r1}, {mask}",
            "and {r2}, {r2}, {mask}",
            "and {r3}, {r3}, {mask}",

            // Outputs
            a_ptr = in(reg) a_ptr,
            m_ptr = in(reg) m_ptr,
            r0 = out(reg) r0,
            r1 = out(reg) r1,
            r2 = out(reg) r2,
            r3 = out(reg) r3,
            // Temporary registers
            a0 = out(reg) _,
            a1 = out(reg) _,
            a2 = out(reg) _,
            a3 = out(reg) _,
            t0 = out(reg) _,
            t1 = out(reg) _,
            t2 = out(reg) _,
            t3 = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            mask = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_double_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(
            // Load 'a' array into registers
            "ldr {a0}, [{a_ptr}, #0]",
            "ldr {a1}, [{a_ptr}, #8]",
            "ldr {a2}, [{a_ptr}, #16]",
            "ldr {a3}, [{a_ptr}, #24]",

            // Double 'a' values with carry propagation
            "adds {a0}, {a0}, {a0}",   // a0 = a0 + a0, sets flags
            "adcs {a1}, {a1}, {a1}",   // a1 = a1 + a1 + carry
            "adcs {a2}, {a2}, {a2}",
            "adcs {a3}, {a3}, {a3}",

            // Load 'm' array into registers for modular reduction
            "ldr {m0}, [{m_ptr}, #0]",
            "ldr {m1}, [{m_ptr}, #8]",
            "ldr {m2}, [{m_ptr}, #16]",
            "ldr {m3}, [{m_ptr}, #24]",

            // Subtract 'm' from the result with borrow propagation
            "subs {r0}, {a0}, {m0}",   // r0 = a0 - m0, sets flags
            "sbcs {r1}, {a1}, {m1}",   // r1 = a1 - m1 - borrow
            "sbcs {r2}, {a2}, {m2}",
            "sbcs {r3}, {a3}, {m3}",

            // Conditional select: if borrow occurred, use original values
            "csel {r0}, {a0}, {r0}, cc",  // cc = carry clear (borrow occurred)
            "csel {r1}, {a1}, {r1}, cc",
            "csel {r2}, {a2}, {r2}, cc",
            "csel {r3}, {a3}, {r3}, cc",

            // Outputs
            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            // Output operands
            r0 = out(reg) r0,
            r1 = out(reg) r1,
            r2 = out(reg) r2,
            r3 = out(reg) r3,
            // Temporary (clobbered) registers
            a0 = out(reg) _,
            a1 = out(reg) _,
            a2 = out(reg) _,
            a3 = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}

// TODO - check the high part carry situation
pub(crate) fn impl_from_mont_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(
            // Load 'a' limbs into registers r0-r3
            "ldr {r0}, [{a_ptr}, #0]",     // r0 = a[0]
            "ldr {r1}, [{a_ptr}, #8]",     // r1 = a[1]
            "ldr {r2}, [{a_ptr}, #16]",    // r2 = a[2]
            "ldr {r3}, [{a_ptr}, #24]",    // r3 = a[3]

            // Load 'm' limbs into registers m0-m3
            "ldr {m0}, [{m_ptr}, #0]",     // m0 = m[0]
            "ldr {m1}, [{m_ptr}, #8]",     // m1 = m[1]
            "ldr {m2}, [{m_ptr}, #16]",    // m2 = m[2]
            "ldr {m3}, [{m_ptr}, #24]",    // m3 = m[3]

            // Load 'inv' (Montgomery constant)
            "mov {inv_reg}, {inv}",        // inv_reg = inv

            // ========================
            // Begin Montgomery Reduction Loop
            // ========================

            // Loop for i = 0 to 3
            // i = 0
            // Compute m = (r0 * inv) mod R (we only need the low part)
            "mul {m}, {r0}, {inv_reg}",    // m = r0 * inv (low 64 bits)

            // Multiply and accumulate with modulus limbs
            // As on Aarch64 we have no 128-bit multiply,
            // as well as no way to do addition in two carry chains,
            // we split the operation into low and high parts

            // Low part first:
            // j = 0
            "mul {l}, {m}, {m0}",         // t0 = m * m[0] (low)
            "adds {r0}, {r0}, {l}",       // r0 = r0 + t0, set flags
            // j = 1
            "mul {l}, {m}, {m1}",         // t1 = m * m[1] (low)
            "adcs {r1}, {r1}, {l}",       // r1 = r1 + t1, set flags
            // j = 2
            "mul {l}, {m}, {m2}",         // t2 = m * m[2] (low)
            "adcs {r2}, {r2}, {l}",       // r2 = r2 + t2, set flags
            // j = 3
            "mul {l}, {m}, {m3}",         // t3 = m * m[3] (low)
            "adcs {r3}, {r3}, {l}",       // r3 = r3 + t3, set flags

            // Low part carry
            "adc {r0}, {r0}, xzr",

            // High part next:
            // j = 0
            "umulh {h}, {m}, {m0}",         // t0 = m * m[0] (high)
            "adds {r1}, {r1}, {h}",       // r1 = r1 + t0 + carry
            // j = 1
            "umulh {h}, {m}, {m1}",         // t1 = m * m[1] (high)
            "adcs {r2}, {r2}, {h}",       // r2 = r2 + t1 + carry
            // j = 2
            "umulh {h}, {m}, {m2}",         // t2 = m * m[2] (high)
            "adcs {r3}, {r3}, {h}",       // r3 = r3 + t2 + carry
            // j = 3
            "umulh {h}, {m}, {m3}",         // t3 = m * m[3] (high)
            "adc {r0}, {r0}, {h}",       // carry = carry + t3 + carry

            // Repeat for i = 1, 2, 3 with adjusted indexing
            // i = 1
            "mul {m}, {r1}, {inv_reg}",
            // Low part
            "mul {l}, {m}, {m0}",
            "adds {r1}, {r1}, {l}",
            "mul {l}, {m}, {m1}",
            "adcs {r2}, {r2}, {l}",
            "mul {l}, {m}, {m2}",
            "adcs {r3}, {r3}, {l}",
            "mul {l}, {m}, {m3}",
            "adcs {r0}, {r0}, {l}",
            // Low part carry
            "adc {r1}, {r1}, xzr",
            // High part
            "umulh {h}, {m}, {m0}",
            "adds {r2}, {r2}, {h}",
            "umulh {h}, {m}, {m1}",
            "adcs {r3}, {r3}, {h}",
            "umulh {h}, {m}, {m2}",
            "adcs {r0}, {r0}, {h}",
            "umulh {h}, {m}, {m3}",
            "adcs {r1}, {r1}, {h}",

            // i = 2
            "mul {m}, {r2}, {inv_reg}",
            // Low part
            "mul {l}, {m}, {m0}",
            "adds {r2}, {r2}, {l}",
            "mul {l}, {m}, {m1}",
            "adcs {r3}, {r3}, {l}",
            "mul {l}, {m}, {m2}",
            "adcs {r0}, {r0}, {l}",
            "mul {l}, {m}, {m3}",
            "adcs {r1}, {r1}, {l}",
            // Low part carry
            "adc {r2}, {r2}, xzr",
            // High part
            "umulh {h}, {m}, {m0}",
            "adds {r3}, {r3}, {h}",
            "umulh {h}, {m}, {m1}",
            "adcs {r0}, {r0}, {h}",
            "umulh {h}, {m}, {m2}",
            "adcs {r1}, {r1}, {h}",
            "umulh {h}, {m}, {m3}",
            "adc {r2}, {r2}, {h}",

            // i = 3
            "mul {m}, {r3}, {inv_reg}",    // m = r3 * inv
            // Low part
            "mul {l}, {m}, {m0}",
            "adds {r3}, {r3}, {l}",
            "mul {l}, {m}, {m1}",
            "adcs {r0}, {r0}, {l}",
            "mul {l}, {m}, {m2}",
            "adcs {r1}, {r1}, {l}",
            "mul {l}, {m}, {m3}",
            "adcs {r2}, {r2}, {l}",
            // Low part carry
            "adc {r3}, {r3}, xzr",
            // High part
            "umulh {h}, {m}, {m0}",
            "adds {r0}, {r0}, {h}",
            "umulh {h}, {m}, {m1}",
            "adcs {r1}, {r1}, {h}",
            "umulh {h}, {m}, {m2}",
            "adcs {r2}, {r2}, {h}",
            "umulh {h}, {m}, {m3}",
            "adc {r3}, {r3}, {h}",

            // ========================
            // End of Montgomery Reduction Loop
            // ========================

            // The result is in r0 (we can output r0 as the final result)

            // Outputs
            a_ptr = in(reg) a_ptr,
            m_ptr = in(reg) m_ptr,
            inv = in(reg) inv,
            // Output operands
            r0 = out(reg) r0,
            r1 = out(reg) r1,
            r2 = out(reg) r2,
            r3 = out(reg) r3,
            // Temporary (clobbered) registers
            m = out(reg) _,
            l = out(reg) _,
            h = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            inv_reg = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_mul_asm() -> TokenStream {
    quote::quote! {
        std::arch::asm!(
            // Constants and pointers
            // m_ptr: pointer to modulus array
            // a_ptr: pointer to operand a array
            // b_ptr: pointer to operand b array
            // inv: Montgomery constant (-m^-1 mod 2^64)

            // Load modulus limbs into registers m0-m3
            "ldr {m0}, [{m_ptr}, #0]",     // m0 = m[0]
            "ldr {m1}, [{m_ptr}, #8]",     // m1 = m[1]
            "ldr {m2}, [{m_ptr}, #16]",    // m2 = m[2]
            "ldr {m3}, [{m_ptr}, #24]",    // m3 = m[3]

            // Initialize accumulators and pointers
            "mov {t0}, xzr",               // t0 = 0
            "mov {t1}, xzr",               // t1 = 0
            "mov {t2}, xzr",               // t2 = 0
            "mov {t3}, xzr",               // t3 = 0
            "mov {t4}, xzr",               // t4 = 0 (extra limb for carry)

            // Load Montgomery constant
            "mov {inv_reg}, {inv}",        // inv_reg = inv

            // Outer loop: i from 0 to 3
            // Unrolled loop for i = 0 to 3

            // ========================
            // i = 0
            // ========================

            // Load b[0]
            "ldr {b_i}, [{b_ptr}, #0]",    // b_i = b[0]
            // Initialize carry A
            "mov {A}, xzr",

            // Multiplication step
            // For j = 0 to 3
            // Multiply a[j] * b_i and add to t[j] with carry A

            // j = 0
            "ldr {a_j}, [{a_ptr}, #0]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t0}, {t0}, {mul_lo}",   // t0 += a[0]*b_i (lo), set flags
            "adc {A}, {mul_hi}, xzr",      // A = mul_hi + carry

            // j = 1
            "ldr {a_j}, [{a_ptr}, #8]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t1}, {t1}, {mul_lo}",   // t1 += a[1]*b_i + A (lo)
            "adc {A}, {A}, {mul_hi}",      // A = A + mul_hi + carry

            // j = 2
            "ldr {a_j}, [{a_ptr}, #16]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t2}, {t2}, {mul_lo}",   // t2 += a[2]*b_i + A (lo)
            "adc {A}, {A}, {mul_hi}",      // A = A + mul_hi + carry

            // j = 3
            "ldr {a_j}, [{a_ptr}, #24]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t3}, {t3}, {mul_lo}",   // t3 += a[3]*b_i + A (lo)
            "adc {A}, {A}, {mul_hi}",      // A = A + mul_hi + carry

            // Store carry in t4
            "adds {t4}, {t4}, {A}",        // t4 += A

            // Reduction step
            // Compute m = t0 * inv mod 2^64
            "mul {m}, {t0}, {inv_reg}",    // m = t0 * inv mod 2^64

            // Multiply m by modulus and add to t[0..3]
            // Also shift t[1..4] to t[0..3] for next iteration

            // m * m[0]
            "umulh {mul_hi}, {m}, {m0}",
            "mul {mul_lo}, {m}, {m0}",
            "adds {t0}, {t0}, {mul_lo}",   // t0 += m * m0
            "adc {C}, {mul_hi}, xzr",      // C = mul_hi + carry

            // m * m[1]
            "umulh {mul_hi}, {m}, {m1}",
            "mul {mul_lo}, {m}, {m1}",
            "adds {t1}, {t1}, {mul_lo}",   // t1 += m * m1 + C
            "adc {C}, {C}, {mul_hi}",      // C = C + mul_hi + carry

            // m * m[2]
            "umulh {mul_hi}, {m}, {m2}",
            "mul {mul_lo}, {m}, {m2}",
            "adds {t2}, {t2}, {mul_lo}",   // t2 += m * m2 + C
            "adc {C}, {C}, {mul_hi}",      // C = C + mul_hi + carry

            // m * m[3]
            "umulh {mul_hi}, {m}, {m3}",
            "mul {mul_lo}, {m}, {m3}",
            "adds {t3}, {t3}, {mul_lo}",   // t3 += m * m3 + C
            "adc {C}, {C}, {mul_hi}",      // C = C + mul_hi + carry

            // Add C to t4
            "adds {t4}, {t4}, {C}",        // t4 += C

            // Shift t[1..4] to t[0..3] for next iteration
            "mov {t0}, {t1}",
            "mov {t1}, {t2}",
            "mov {t2}, {t3}",
            "mov {t3}, {t4}",
            "mov {t4}, xzr",               // Reset t4

            // Repeat for i = 1 to 3
            // ========================
            // i = 1
            // ========================

            // Load b[1]
            "ldr {b_i}, [{b_ptr}, #8]",    // b_i = b[1]
            // Initialize carry A
            "mov {A}, xzr",

            // Multiplication step
            // j = 0
            "ldr {a_j}, [{a_ptr}, #0]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t0}, {t0}, {mul_lo}",   // t0 += a[0]*b_i
            "adc {A}, {mul_hi}, xzr",

            // j = 1
            "ldr {a_j}, [{a_ptr}, #8]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t1}, {t1}, {mul_lo}",   // t1 += a[1]*b_i + A
            "adc {A}, {A}, {mul_hi}",

            // j = 2
            "ldr {a_j}, [{a_ptr}, #16]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // j = 3
            "ldr {a_j}, [{a_ptr}, #24]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // Store carry in t4
            "adds {t4}, {t4}, {A}",

            // Reduction step
            // Compute m = t0 * inv mod 2^64
            "mul {m}, {t0}, {inv_reg}",

            // m * m[0]
            "umulh {mul_hi}, {m}, {m0}",
            "mul {mul_lo}, {m}, {m0}",
            "adds {t0}, {t0}, {mul_lo}",
            "adc {C}, {mul_hi}, xzr",

            // m * m[1]
            "umulh {mul_hi}, {m}, {m1}",
            "mul {mul_lo}, {m}, {m1}",
            "adds {t1}, {t1}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[2]
            "umulh {mul_hi}, {m}, {m2}",
            "mul {mul_lo}, {m}, {m2}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[3]
            "umulh {mul_hi}, {m}, {m3}",
            "mul {mul_lo}, {m}, {m3}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // Add C to t4
            "adds {t4}, {t4}, {C}",

            // Shift t[1..4] to t[0..3]
            "mov {t0}, {t1}",
            "mov {t1}, {t2}",
            "mov {t2}, {t3}",
            "mov {t3}, {t4}",
            "mov {t4}, xzr",

            // ========================
            // i = 2
            // ========================

            // Load b[2]
            "ldr {b_i}, [{b_ptr}, #16]",    // b_i = b[2]
            // Initialize carry A
            "mov {A}, xzr",

            // Multiplication step
            // j = 0
            "ldr {a_j}, [{a_ptr}, #0]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t0}, {t0}, {mul_lo}",
            "adc {A}, {mul_hi}, xzr",

            // j = 1
            "ldr {a_j}, [{a_ptr}, #8]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t1}, {t1}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // j = 2
            "ldr {a_j}, [{a_ptr}, #16]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // j = 3
            "ldr {a_j}, [{a_ptr}, #24]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // Store carry in t4
            "adds {t4}, {t4}, {A}",

            // Reduction step
            // Compute m = t0 * inv mod 2^64
            "mul {m}, {t0}, {inv_reg}",

            // m * m[0]
            "umulh {mul_hi}, {m}, {m0}",
            "mul {mul_lo}, {m}, {m0}",
            "adds {t0}, {t0}, {mul_lo}",
            "adc {C}, {mul_hi}, xzr",

            // m * m[1]
            "umulh {mul_hi}, {m}, {m1}",
            "mul {mul_lo}, {m}, {m1}",
            "adds {t1}, {t1}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[2]
            "umulh {mul_hi}, {m}, {m2}",
            "mul {mul_lo}, {m}, {m2}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[3]
            "umulh {mul_hi}, {m}, {m3}",
            "mul {mul_lo}, {m}, {m3}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // Add C to t4
            "adds {t4}, {t4}, {C}",

            // Shift t[1..4] to t[0..3]
            "mov {t0}, {t1}",
            "mov {t1}, {t2}",
            "mov {t2}, {t3}",
            "mov {t3}, {t4}",
            "mov {t4}, xzr",

            // ========================
            // i = 3
            // ========================

            // Load b[3]
            "ldr {b_i}, [{b_ptr}, #24]",    // b_i = b[3]
            // Initialize carry A
            "mov {A}, xzr",

            // Multiplication step
            // j = 0
            "ldr {a_j}, [{a_ptr}, #0]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t0}, {t0}, {mul_lo}",
            "adc {A}, {mul_hi}, xzr",

            // j = 1
            "ldr {a_j}, [{a_ptr}, #8]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t1}, {t1}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // j = 2
            "ldr {a_j}, [{a_ptr}, #16]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // j = 3
            "ldr {a_j}, [{a_ptr}, #24]",
            "umulh {mul_hi}, {a_j}, {b_i}",
            "mul {mul_lo}, {a_j}, {b_i}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {A}, {A}, {mul_hi}",

            // Store carry in t4
            "adds {t4}, {t4}, {A}",

            // Reduction step
            // Compute m = t0 * inv mod 2^64
            "mul {m}, {t0}, {inv_reg}",

            // m * m[0]
            "umulh {mul_hi}, {m}, {m0}",
            "mul {mul_lo}, {m}, {m0}",
            "adds {t0}, {t0}, {mul_lo}",
            "adc {C}, {mul_hi}, xzr",

            // m * m[1]
            "umulh {mul_hi}, {m}, {m1}",
            "mul {mul_lo}, {m}, {m1}",
            "adds {t1}, {t1}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[2]
            "umulh {mul_hi}, {m}, {m2}",
            "mul {mul_lo}, {m}, {m2}",
            "adds {t2}, {t2}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // m * m[3]
            "umulh {mul_hi}, {m}, {m3}",
            "mul {mul_lo}, {m}, {m3}",
            "adds {t3}, {t3}, {mul_lo}",
            "adc {C}, {C}, {mul_hi}",

            // Add C to t4
            "adds {t4}, {t4}, {C}",

            // Final result is in t0..t3
            // Need to subtract modulus if t >= m

            // Subtract modulus m from t
            "subs {r0}, {t0}, {m0}",       // r0 = t0 - m0
            "sbcs {r1}, {t1}, {m1}",       // r1 = t1 - m1 - borrow
            "sbcs {r2}, {t2}, {m2}",
            "sbcs {r3}, {t3}, {m3}",
            "sbc {borrow}, xzr, xzr",      // borrow = final borrow

            // Conditional select: if borrow is 0, result is r0..r3
            // Otherwise, result is t0..t3

            "csel {r0}, {r0}, {t0}, lo",   // If borrow == 0, keep r0; else t0
            "csel {r1}, {r1}, {t1}, lo",
            "csel {r2}, {r2}, {t2}, lo",
            "csel {r3}, {r3}, {t3}, lo",

            // Outputs
            a_ptr = in(reg) a_ptr,
            b_ptr = in(reg) b_ptr,
            m_ptr = in(reg) m_ptr,
            inv = in(reg) inv,

            // Output operands
            r0 = out(reg) r0,
            r1 = out(reg) r1,
            r2 = out(reg) r2,
            r3 = out(reg) r3,

            // Temporary registers
            t0 = out(reg) _,
            t1 = out(reg) _,
            t2 = out(reg) _,
            t3 = out(reg) _,
            t4 = out(reg) _,
            a_j = out(reg) _,
            b_i = out(reg) _,
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            m = out(reg) _,
            A = out(reg) _,
            C = out(reg) _,
            mul_hi = out(reg) _,
            mul_lo = out(reg) _,
            inv_reg = out(reg) _,
            borrow = out(reg) _,
            options(pure, readonly, nostack)
        );
    }
}
