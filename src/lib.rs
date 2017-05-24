#![feature(asm)]
#![feature(lang_items)]
#![no_std]

pub mod lang_items;
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
