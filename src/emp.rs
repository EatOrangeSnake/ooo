pub trait Empty{
    fn empty() -> Self;
}


impl Empty for u8{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!( 
                "", 
                out("al") out
            );
            out
        }
    }
}


impl Empty for u16{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("ax") out
            );
            out
        }
    }
}


impl Empty for u32{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("eax") out
            );
            out
        }
    }
}


impl Empty for u64{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("rax") out
            );
            out
        }
    }
}


impl Empty for usize{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "", 
                out("rax") out
            );
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "", 
                out("eax") out
            );
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "", 
                out("ax") out
            );
            out
        }
    }
}


impl Empty for i8{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("al") out
            );
            out
        }
    }
}


impl Empty for i16{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("ax") out
            );
            out
        }
    }
}


impl Empty for i32{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("eax") out
            );
            out
        }
    }
}


impl Empty for i64{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            std::arch::asm!(
                "", 
                out("rax") out
            );
            out
        }
    }
}


impl Empty for isize{
    #[inline]
    fn empty() -> Self {
        let out: Self;
        unsafe{
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!(
                "", 
                out("rax") out
            );
            #[cfg(target_arch = "x86_32")]
            std::arch::asm!(
                "", 
                out("eax") out
            );
            #[cfg(target_arch = "x8086")]
            std::arch::asm!(
                "", 
                out("ax") out
            );
            out
        }
    }
}
