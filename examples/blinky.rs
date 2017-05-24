#![no_std]

extern crate ble400;

use ble400::ll;

fn main() {
    unsafe { ll::bsp_board_leds_init() }

    loop {
        for i in 0..5 {
            unsafe { ll::bsp_board_led_invert(i) }
            unsafe { ll::_nrf_delay_ms(100) }
        }
    }
}
