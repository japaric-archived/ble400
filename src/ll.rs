extern "C" {
    pub fn bsp_board_led_invert(led_idx: u32);
    pub fn bsp_board_leds_init();

    pub fn _nrf_delay_ms(number_of_ms: u32);
}
