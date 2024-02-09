pub unsafe trait Pop{
    unsafe fn pop() -> Self;
}


unsafe impl Pop for u8{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {};
        ", out(reg_byte) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for u16{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:x};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for u32{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:e};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for u64{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:r};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for usize{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:r};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for i8{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {};
        ", out(reg_byte) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for i16{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:x};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for i32{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:e};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for i64{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:r};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}


unsafe impl Pop for isize{
    unsafe fn pop() -> Self {
        let out: Self;
        std::arch::asm!("
            POP {:r};
        ", out(reg) out, options(nomem, nostack, pure));
        out
    }
}
