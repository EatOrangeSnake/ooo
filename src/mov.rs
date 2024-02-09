///Input a mutable refence and a immutable refence move to the first argument.
///```
/// use ooo::mov::*;
/// let mut x: usize = 3usize;
/// let y: usize = 3usize;
/// x.mov(&y); //set x as y.
///```
///Maybe you think this is unuseful, but if you want to move bigger size variable
/// as smaller size variable.
///You can do it by this method.
///```
/// use ooo::mov::*;
/// let mut x: u8 = 3;
/// let y: u16 = 3;
/// //Watch out! the size of u8 is not same as the size of u16. But We have a good solution.
/// x.mov(&y); //will move the low byte of y to x.
///```


pub trait Mov<T>{
    fn mov(&mut self, rhs: &T);
}


impl<T> Mov<T> for u8{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {}, BYTE PTR [{}];
                "
            , out(reg_byte) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV BYTE PTR [{}], {};
                "
            , in(reg) self
            , in(reg_byte) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for u16{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:x}, WORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV WORD PTR [{}], {:x};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for u32{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:e}, DWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV DWORD PTR [{}], {:e};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for u64{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:r}, QWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV QWORD PTR [{}], {:r};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for usize{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        let temp: Self;
        #[cfg(target_arch = "x86_64")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:r}, QWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV QWORD PTR [{}], {:r};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
        #[cfg(target_arch = "x86_32")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:e}, DWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV DWORD PTR [{}], {:e};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
        #[cfg(target_arch = "x8086")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:x}, WORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV WORD PTR [{}], {:x};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for i8{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {}, BYTE PTR [{}];
                "
            , out(reg_byte) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV BYTE PTR [{}], {};
                "
            , in(reg) self
            , in(reg_byte) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for i16{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:x}, WORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV WORD PTR [{}], {:x};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for i32{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:e}, DWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV DWORD PTR [{}], {:e};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for i64{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        unsafe{
            let temp: Self;
            std::arch::asm!(
                "
                    MOV {:r}, QWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV QWORD PTR [{}], {:r};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}


impl<T> Mov<T> for isize{
    #[inline]
    fn mov(&mut self, rhs: &T) {
        let temp: Self;
        #[cfg(target_arch = "x86_64")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:r}, QWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV QWORD PTR [{}], {:r};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
        #[cfg(target_arch = "x86_32")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:e}, DWORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV DWORD PTR [{}], {:e};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
        #[cfg(target_arch = "x8086")]
        unsafe{
            std::arch::asm!(
                "
                    MOV {:x}, WORD PTR [{}];
                "
            , out(reg) temp
            , in(reg) rhs
            , options(nostack, pure, readonly));
            std::arch::asm!(
                "
                    MOV WORD PTR [{}], {:x};
                "
            , in(reg) self
            , in(reg) temp
            , options(nostack));
        };
    }
}
