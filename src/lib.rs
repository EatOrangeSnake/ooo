//!the extensions for operators of the Rust.
//!until now, this crate provide four operators, such as 
//!DEC(decrease,same as substruct 1), INC(increase,same as add 1).
//!these operators are same as some assmbly instructions.
//!so you are not worth worrying workpiece.
//!for exsample:
//! ```
//! use ode::inc::*; //use the trait that increase.
//! use ode::ina::*; //use the trait that increase assign.
//! let mut x: usize = 300usize; //assign "300usize" to x.
//! assert_eq!(x, 300usize); //no problem.
//! x.inc_assign(); //add one to x when use trait "IncAssign".
//! assert_eq!(x, 301usize); //if you do it,should has not worings.
//! assert_eq!(x.inc(), 302usize); //add one again.it will be "302usize" when use trait "Inc".
//! assert_eq!(x, 301usize); //but you can know the variable "x" was not affected.
//! //because "inc" method return a new increased value and not affect self, 
//! //"inc_assign" method affect self and do not return anything.
//! x.dec_assign(); //substruct one to x when use trait "DecrefAssign"
//! assert_eq!(x, 301usize); //no ploblem.
//! assert_eq!(x.dec(), 300usize); //Allow method "decrease" when use trait "Dec"
//! assert_eq!(x, 301usize); //But also do not affect self.
//! ```
//! if you have some suggest, welcome to tell me.my email is "ora_in_py@hotmail.com".
pub mod ina;
pub mod dea;
pub mod inc;
pub mod dec;
#[cfg(not(target_arch = "x86_64"))]
pub mod pop;
#[cfg(not(target_arch = "x86_64"))]
pub mod push;
