///```
/// use ooo::ina::*;
/// let x: usize = 3usize;
/// x.inc_assign(); //increace self(add one to self)
/// assert_eq!(x, 4usize) //x was increaced and affected;
///```


pub trait IncAssign{
    fn inc_assign(&mut self);
}


impl IncAssign for u8{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC BYTE PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for u16{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for u32{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for u64{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for usize{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("INC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!("INC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!("INC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for i8{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC BYTE PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for i16{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for i32{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for i64{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            std::arch::asm!("INC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}


impl IncAssign for isize{
    #[inline]
    fn inc_assign(&mut self) {
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("INC QWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!("INC DWORD PTR [{}];", in(reg) self, options(nostack, nomem));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!("INC WORD PTR [{}];", in(reg) self, options(nostack, nomem));
        };
    }
}
