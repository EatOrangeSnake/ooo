pub trait DecAssign{
    fn dec_assign(&mut self);
}


impl DecAssign for u8{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC BYTE PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for u16{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for u32{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for u64{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for usize{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("DEC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!("DEC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!("DEC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for i8{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC BYTE PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for i16{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for i32{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for i64{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            std::arch::asm!("DEC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl DecAssign for isize{
    #[inline]
    fn dec_assign(&mut self) {
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("DEC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!("DEC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!("DEC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}
