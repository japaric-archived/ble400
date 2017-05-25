#![no_std]

#[macro_use]
extern crate ble400;

use ble400::ll;

const APP_TIMER_PRESCALER: u32 = 0;
const APP_TIMER_OP_QUEUE_SIZE: u8 = 4;

const RX_BUF_SIZE: u32 = 256;
const TX_BUF_SIZE: u32 = 256;

extern "C" fn uart_event_handle(_event: *mut ll::app_uart_evt_t) {
    ble400::bkpt();
}

unsafe fn uart_init() {
    let comm_params = ll::app_uart_comm_params_t {
        rx_pin_no: ll::RX_PIN_NUMBER,
        tx_pin_no: ll::TX_PIN_NUMBER,
        rts_pin_no: ll::RTS_PIN_NUMBER,
        cts_pin_no: ll::CTS_PIN_NUMBER,
        flow_control:
            ll::app_uart_flow_control_t::APP_UART_FLOW_CONTROL_DISABLED,
        use_parity: false,
        baud_rate: ll::UART_BAUDRATE_BAUDRATE_Baud115200,
    };

    let err_code = APP_UART_FIFO_INIT!(
        &comm_params,
        RX_BUF_SIZE,
        TX_BUF_SIZE,
        Some(uart_event_handle),
        APP_IRQ_PRIORITY_LOW
    );

    APP_ERROR_CHECK!(err_code);
}

fn main() {
    ble400::bkpt();

    unsafe {
        APP_TIMER_INIT!(APP_TIMER_PRESCALER, APP_TIMER_OP_QUEUE_SIZE, None);
    }

    unsafe { uart_init() }

    let ec = unsafe { ll::app_uart_put(b'H') };
    let ec = unsafe { ll::app_uart_put(b'e') };

    ble400::bkpt();

    // loop {}
}
