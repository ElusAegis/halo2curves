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
