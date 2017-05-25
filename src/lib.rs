#![feature(asm)]
#![feature(lang_items)]
#![feature(untagged_unions)]
#![no_std]

extern crate cty;

pub mod lang_items;
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod ll;

pub fn bkpt() {
    unsafe { asm!("bkpt" :::: "volatile") }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}
