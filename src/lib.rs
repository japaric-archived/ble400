#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(untagged_unions)]
#![no_std]

extern crate aligned;
extern crate cty;
#[macro_use]
extern crate cortex_m_semihosting;

pub mod lang_items;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod ll;

use core::mem;

use cty::c_void;

#[doc(hidden)]
pub use aligned::Aligned;

#[macro_export]
macro_rules! APP_TIMER_INIT {
    ($PRESCALER:expr, $OP_QUEUE_SIZE:expr, $SCHEDULER_FUNC:expr) => {
        {
            use $crate::Aligned;

            const __PRESCALER: u32 = $PRESCALER;
            const __OP_QUEUE_SIZE: u8 = $OP_QUEUE_SIZE;
            const SZ: usize = ll::APP_TIMER_USER_OP_SIZE as usize *
                (__OP_QUEUE_SIZE as usize + 1);
            static mut APP_TIMER_BUF: Aligned<u32, [u8; SZ]> = Aligned([0; SZ]);

            $crate::check(ll::app_timer_init(
                __PRESCALER,
                __OP_QUEUE_SIZE + 1,
                &mut APP_TIMER_BUF as *mut _ as *mut _,
                $SCHEDULER_FUNC,
            ))
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

            $crate::check(ll::app_uart_init(
                $P_COMM_PARAMS,
                &mut buffers,
                $EVT_HANDLER,
                ll::app_irq_priority_t::$IRQ_PRIO,
            ))
        }
    }
}

#[allow(non_snake_case)]
pub unsafe fn SOFTDEVICE_HANDLER_INIT(
    CLOCK_SOURCE: *mut ll::nrf_clock_lf_cfg_t,
    EVT_HANDLER: Option<unsafe extern "C" fn() -> u32>,
) -> Result<(), Error> {
    // FIXME(hardcoded) this is `mem::size_of::<ble_evt_t>()`
    const BLE_EVT_T_SZ: usize = 48;
    const SZ: usize = BLE_EVT_T_SZ + ll::NRF_BLE_GATT_MAX_MTU_SIZE as usize;
    static mut BLE_EVT_BUFFER: Aligned<u32, [u8; SZ]> = Aligned([0; SZ]);

    assert_eq!(mem::size_of::<ll::ble_evt_t>(), BLE_EVT_T_SZ);

    check(ll::softdevice_handler_init(
        CLOCK_SOURCE,
        BLE_EVT_BUFFER.array.as_mut_ptr() as *mut c_void,
        SZ as u16,
        EVT_HANDLER,
    ))
}

#[allow(non_snake_case)]
pub unsafe fn BLE_GAP_CONN_SEC_MODE_SET_OPEN(
    sec_mode: *mut ll::ble_gap_conn_sec_mode_t,
) {
    (*sec_mode).set_sm(1);
    (*sec_mode).set_lv(1);
}

#[inline(always)]
pub fn bkpt() {
    unsafe { asm!("bkpt" :::: "volatile") }
}

/// Checks an error code
pub fn check(ec: u32) -> Result<(), Error> {
    match ec {
        ll::NRF_SUCCESS => Ok(()),
        ll::NRF_ERROR_SVC_HANDLER_MISSING => Err(Error::SvcHandlerMissing),
        ll::NRF_ERROR_SOFTDEVICE_NOT_ENABLED => Err(
            Error::SoftdeviceNotEnabled,
        ),
        ll::NRF_ERROR_INTERNAL => Err(Error::Internal),
        ll::NRF_ERROR_NO_MEM => Err(Error::NoMem),
        ll::NRF_ERROR_NOT_FOUND => Err(Error::NotFound),
        ll::NRF_ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
        ll::NRF_ERROR_INVALID_PARAM => Err(Error::InvalidParam),
        ll::NRF_ERROR_INVALID_STATE => Err(Error::InvalidState),
        ll::NRF_ERROR_INVALID_LENGTH => Err(Error::InvalidLength),
        ll::NRF_ERROR_INVALID_FLAGS => Err(Error::InvalidFlags),
        ll::NRF_ERROR_INVALID_DATA => Err(Error::InvalidData),
        ll::NRF_ERROR_DATA_SIZE => Err(Error::DataSize),
        ll::NRF_ERROR_TIMEOUT => Err(Error::Timeout),
        ll::NRF_ERROR_NULL => Err(Error::Null),
        ll::NRF_ERROR_FORBIDDEN => Err(Error::Forbidden),
        ll::NRF_ERROR_INVALID_ADDR => Err(Error::InvalidAddr),
        ll::NRF_ERROR_BUSY => Err(Error::Busy),
        ll::NRF_ERROR_CONN_COUNT => Err(Error::ConnCount),
        ll::NRF_ERROR_RESOURCES => Err(Error::Resources),
        ll::NRF_ERROR_MODULE_NOT_INITIALZED => Err(Error::ModuleNotInitialzed),
        ll::NRF_ERROR_MUTEX_INIT_FAILED => Err(Error::MutexInitFailed),
        ll::NRF_ERROR_MUTEX_LOCK_FAILED => Err(Error::MutexLockFailed),
        ll::NRF_ERROR_MUTEX_UNLOCK_FAILED => Err(Error::MutexUnlockFailed),
        ll::NRF_ERROR_MUTEX_COND_INIT_FAILED => Err(Error::MutexCondInitFailed),
        ll::NRF_ERROR_MODULE_ALREADY_INITIALIZED => {
            Err(Error::ModuleAlreadyInitialized)
        }
        ll::NRF_ERROR_STORAGE_FULL => Err(Error::StorageFull),
        ll::NRF_ERROR_API_NOT_IMPLEMENTED => Err(Error::ApiNotImplemented),
        ll::NRF_ERROR_FEATURE_NOT_ENABLED => Err(Error::FeatureNotEnabled),
        ll::NRF_ERROR_DRV_TWI_ERR_OVERRUN => Err(Error::DrvTwiErrOverrun),
        ll::NRF_ERROR_DRV_TWI_ERR_ANACK => Err(Error::DrvTwiErrAnack),
        ll::NRF_ERROR_DRV_TWI_ERR_DNACK => Err(Error::DrvTwiErrDnack),
        _ => Err(Error::Unknown(ec)),
    }
}

#[derive(Debug)]
pub enum Error {
    SvcHandlerMissing,
    SoftdeviceNotEnabled,
    Internal,
    NoMem,
    NotFound,
    NotSupported,
    InvalidParam,
    InvalidState,
    InvalidLength,
    InvalidFlags,
    InvalidData,
    DataSize,
    Timeout,
    Null,
    Forbidden,
    InvalidAddr,
    Busy,
    ConnCount,
    Resources,
    ModuleNotInitialzed,
    MutexInitFailed,
    MutexLockFailed,
    MutexUnlockFailed,
    MutexCondInitFailed,
    ModuleAlreadyInitialized,
    StorageFull,
    ApiNotImplemented,
    FeatureNotEnabled,
    DrvTwiErrOverrun,
    DrvTwiErrAnack,
    DrvTwiErrDnack,
    Unknown(u32),
}

#[macro_export]
macro_rules! APP_TIMER_TICKS {
    ($MS:expr, $PRESCALER:expr) => {
        $MS * $crate::ll::APP_TIMER_CLOCK_FREQ / (($PRESCALER + 1) * 1_000)
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}

// SVCALL
macro_rules! svcall {
    ($SET:ident::$SVC:ident) => {
        {
            const N: u8 = ll::$SET::$SVC as u8;

            let ret;
            asm!("svc $1"
                 : "={r0}"(ret)
                 : "I"(N)
                 : "r0"
                 : "volatile");

            ret
        }
    };
    ($SET:ident::$SVC:ident, $r0:ident) => {
        {
            const N: u8 = ll::$SET::$SVC as u8;

            let ret;
            asm!("svc $1"
                 : "={r0}"(ret)
                 : "I"(N) "{r0}"($r0)
                 : "r0"
                 : "volatile");

            ret
        }
    };
    ($SET:ident::$SVC:ident, $r0:ident, $r1:ident, $r2:ident) => {
        {
            const N: u8 = ll::$SET::$SVC as u8;

            let ret;
            asm!("svc $1"
                 : "={r0}"(ret)
                 : "I"(N) "{r0}"($r0) "{r1}"($r1) "{r2}"($r2)
                 : "r0"
                 : "volatile");

            ret
        }
    };
}

#[inline(always)]
pub unsafe extern "C" fn sd_ble_gap_device_name_set(
    p_write_perm: *const ll::ble_gap_conn_sec_mode_t,
    p_dev_name: *const u8,
    len: u16,
) -> Result<(), Error> {
    let ret = svcall!(
        BLE_GAP_SVCS::SD_BLE_GAP_DEVICE_NAME_SET,
        p_write_perm,
        p_dev_name,
        len
    );
    check(ret)
}

#[inline(always)]
pub unsafe extern "C" fn sd_ble_gap_ppcp_set(
    p_conn_params: *const ll::ble_gap_conn_params_t,
) -> Result<(), Error> {
    let ret = svcall!(BLE_GAP_SVCS::SD_BLE_GAP_PPCP_SET, p_conn_params);
    check(ret)
}

#[inline(always)]
pub unsafe extern "C" fn sd_app_evt_wait() -> Result<(), Error> {
    let ret = svcall!(NRF_SOC_SVCS::SD_APP_EVT_WAIT);
    check(ret)
}
