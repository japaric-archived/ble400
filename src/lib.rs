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

#[macro_export]
macro_rules! APP_ERROR_CHECK {
    ($ERR_CODE:expr) => {
        {
            let local_err_code: u32 = $ERR_CODE;
            if local_err_code != $crate::ll::NRF_SUCCESS {
                ll::app_error_handler_bare(local_err_code);
            }
        }
    }
}

#[macro_export]
macro_rules! APP_TIMER_INIT {
    ($PRESCALER:expr, $OP_QUEUE_SIZE:expr, $SCHEDULER_FUNC:expr) => {
        {
            const __PRESCALER: u32 = $PRESCALER;
            const __OP_QUEUE_SIZE: u8 = $OP_QUEUE_SIZE;
            const SZ: usize = ll::APP_TIMER_USER_OP_SIZE as usize *
                (__OP_QUEUE_SIZE as usize + 1);
            static mut APP_TIMER_BUF: [u8; SZ] = [0; SZ];

            let err_code = ll::app_timer_init(
                __PRESCALER,
                __OP_QUEUE_SIZE + 1,
                &mut APP_TIMER_BUF as *mut _ as *mut _,
                $SCHEDULER_FUNC,
            );
            APP_ERROR_CHECK!(err_code);
        }
    }
}

#[macro_export]
macro_rules! APP_UART_FIFO_INIT {
    ($P_COMM_PARAMS:expr,
     $RX_BUF_SIZE:expr,
     $TX_BUF_SIZE:expr,
     $EVT_HANDLER:expr,
     $IRQ_PRIO:ident) => {
        {
            const __RX_BUF_SIZE: u32 = $RX_BUF_SIZE;
            const __TX_BUF_SIZE: u32 = $TX_BUF_SIZE;

            static mut RX_BUF: [u8; __RX_BUF_SIZE as usize] =
                [0; RX_BUF_SIZE as usize];
            static mut TX_BUF: [u8; __TX_BUF_SIZE as usize] =
                [0; TX_BUF_SIZE as usize];
            let mut buffers = ll::app_uart_buffers_t {
                rx_buf: RX_BUF.as_mut_ptr(),
                tx_buf: TX_BUF.as_mut_ptr(),
                tx_buf_size: __TX_BUF_SIZE,
                rx_buf_size: __RX_BUF_SIZE,
            };

            ll::app_uart_init(
                $P_COMM_PARAMS,
                &mut buffers,
                $EVT_HANDLER,
                ll::app_irq_priority_t::$IRQ_PRIO,
            )
        }
    }
}

pub fn bkpt() {
    unsafe { asm!("bkpt" :::: "volatile") }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}
