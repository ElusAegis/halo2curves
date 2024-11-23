use proc_macro2::TokenStream;

pub(super) fn impl_add_asm() -> TokenStream {
    quote::quote! {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // add a array and b array with carry
            "add r8, qword ptr [{b_ptr} + 0]",
            "adc r9, qword ptr [{b_ptr} + 8]",
            "adc r10, qword ptr [{b_ptr} + 16]",
            "adc r11, qword ptr [{b_ptr} + 24]",

            // copy result array to latter registers
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod reduction
            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            // if carry copy former registers to out areas
            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            b_ptr = in(reg) b_ptr,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
}

pub(super) fn impl_sub_asm() -> TokenStream {
    quote::quote! {
        asm!(
            // init modulus area
            "mov r12, qword ptr [{m_ptr} + 0]",
            "mov r13, qword ptr [{m_ptr} + 8]",
            "mov r14, qword ptr [{m_ptr} + 16]",
            "mov r15, qword ptr [{m_ptr} + 24]",

            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // sub a array and b array with borrow
            "sub r8, qword ptr [{b_ptr} + 0]",
            "sbb r9, qword ptr [{b_ptr} + 8]",
            "sbb r10, qword ptr [{b_ptr} + 16]",
            "sbb r11, qword ptr [{b_ptr} + 24]",

            // Mask: rax contains 0xFFFF if < m or 0x0000 otherwise
            "sbb rax, rax",

            // Zero-out the modulus if a-b < m or leave as-is otherwise
            "and r12, rax",
            "and r13, rax",
            "and r14, rax",
            "and r15, rax",

            // Add zero if a-b < m or a-b+m otherwise
            "add  r12, r8",
            "adc  r13, r9",
            "adc  r14, r10",
            "adc  r15, r11",

            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            b_ptr = in(reg) b_ptr,
            out("rax") _,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_neg_asm() -> TokenStream {
    quote::quote! {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{m_ptr} + 0]",
            "mov r9, qword ptr [{m_ptr} + 8]",
            "mov r10, qword ptr [{m_ptr} + 16]",
            "mov r11, qword ptr [{m_ptr} + 24]",

            "sub r8, qword ptr [{a_ptr} + 0]",
            "sbb r9, qword ptr [{a_ptr} + 8]",
            "sbb r10, qword ptr [{a_ptr} + 16]",
            "sbb r11, qword ptr [{a_ptr} + 24]",

            "mov r12, qword ptr [{a_ptr} + 0]",
            "mov r13, qword ptr [{a_ptr} + 8]",
            "mov r14, qword ptr [{a_ptr} + 16]",
            "mov r15, qword ptr [{a_ptr} + 24]",

            "or r12, r13",
            "or r14, r15",
            "or r12, r14",

            "mov r13, 0xffffffffffffffff",
            "cmp r12, 0x0000000000000000",
            "cmove r13, r12",

            "and r8, r13",
            "and r9, r13",
            "and r10, r13",
            "and r11, r13",

            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            out("r8") r0,
            out("r9") r1,
            out("r10") r2,
            out("r11") r3,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_double_asm() -> TokenStream {
    quote::quote! {
        asm!(
            // load a array to former registers
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // add a array and b array with carry
            "add r8, r8",
            "adc r9, r9",
            "adc r10, r10",
            "adc r11, r11",

            // copy result array to latter registers
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod reduction
            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            // if carry copy former registers to out areas
            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
}

pub(crate) fn impl_from_mont_asm() -> TokenStream {
    quote::quote! {
        asm!(
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",
            "mov r15, {inv}",
            "xor r12, r12",

            // i0
            "mov rdx, r8",
            "mulx rcx, rdx, r15",

            // j0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "adox r8, rax",
            "adcx r9, rcx",
            // j1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "adox r9, rax",
            "adcx r10, rcx",
            // j2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "adox r10, rax",
            "adcx r11, rcx",
            // j3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "adox r11, rax",
            "adcx r8, rcx",
            "adox r8, r12",

            // i1
            "mov rdx, r9",
            "mulx rcx, rdx, r15",

            // j0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "adox r9, rax",
            "adcx r10, rcx",

            // j1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "adox r10, rax",
            "adcx r11, rcx",
            // j2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "adox r11, rax",
            "adcx r8, rcx",
            // j3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "adox r8, rax",
            "adcx r9, rcx",
            "adox r9, r12",

            // i2
            "mov rdx, r10",
            "mulx rcx, rdx, r15",

            // j0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "adox r10, rax",
            "adcx r11, rcx",

            // j1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "adox r11, rax",
            "adcx r8, rcx",

            // j2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "adox r8, rax",
            "adcx r9, rcx",

            // j3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "adox r9, rax",
            "adcx r10, rcx",
            "adox r10, r12",

            // i3
            "mov rdx, r11",
            "mulx rcx, rdx, r15",
            // j0
            "mulx rcx, rax, qword ptr [{m_ptr} + 0]",
            "adox r11, rax",
            "adcx r8, rcx",
            // j1
            "mulx rcx, rax, qword ptr [{m_ptr} + 8]",
            "adox r8, rax",
            "adcx r9, rcx",
            // j2
            "mulx rcx, rax, qword ptr [{m_ptr} + 16]",
            "adox r9, rax",
            "adcx r10, rcx",
            // j3
            "mulx rcx, rax, qword ptr [{m_ptr} + 24]",
            "adox r10, rax",
            "adcx r11, rcx",
            "adox r11, r12",

            // modular reduction is not required since:
            // high(inv * p3) + 2 < p3

            m_ptr = in(reg) m_ptr,
            a_ptr = in(reg) a_ptr,
            inv = in(reg) inv,

            out("rax") _,
            out("rcx") _,
            out("rdx") _,
            out("r8") r0,
            out("r9") r1,
            out("r10") r2,
            out("r11") r3,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            options(pure, readonly, nostack)
        );
    }
}
