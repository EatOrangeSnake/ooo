///like as DecAssign, but it will return new value and not affect self.
///```
/// use ooo::dec;
/// let x: usize = 3usize;
/// assert_eq!(x.dec(), 2usize); //return decreased value.
/// assert_eq!(x, 3usize); //but do not affect self.
///```


pub trait Dec{
    fn dec(&self) -> Self;
}


impl Dec for u8{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1}, BYTE PTR [{0}];
                    DEC {1};
                "
            , in(reg) self, lateout(reg_byte) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for u16{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    DEC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for u32{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    DEC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for u64{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for usize{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    DEC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for i8{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1}, BYTE PTR [{0}];
                    DEC {1};
                "
            , in(reg) self, lateout(reg_byte) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for i16{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    DEC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for i32{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    DEC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for i64{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Dec for isize{
    #[inline]
    fn dec(&self) -> Self {
        unsafe{
            let out: Self;
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    DEC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    DEC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}
