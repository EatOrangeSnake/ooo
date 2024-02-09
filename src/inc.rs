///like as IncAssign, but do not affect self.
///```
/// use ooo::inc::*;
/// let x: usize = 3usize;
/// assert_eq!(x.inc(), 4usize); //return a increased value.
/// assert_eq!(x, 3usize); //but do not affect self.
///```


pub trait Inc{
    fn inc(&self) -> Self;
}


impl Inc for u8{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1}, BYTE PTR [{0}];
                    INC {1};
                "
            , in(reg) self, lateout(reg_byte) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for u16{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    INC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for u32{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    INC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for u64{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    INC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for usize{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    INC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    INC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    INC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for i8{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1}, BYTE PTR [{0}];
                    INC {1};
                "
            , in(reg) self, lateout(reg_byte) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for i16{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    INC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for i32{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    INC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for i64{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    INC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}


impl Inc for isize{
    #[inline]
    fn inc(&self) -> Self {
        unsafe{
            let out: Self;
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "
                    MOV {1:r}, QWORD PTR [{0}];
                    INC {1:r};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "
                    MOV {1:e}, DWORD PTR [{0}];
                    INC {1:e};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "
                    MOV {1:x}, WORD PTR [{0}];
                    INC {1:x};
                "
            , in(reg) self, lateout(reg) out, options(nostack, pure, readonly));
            out
        }
    }
}
