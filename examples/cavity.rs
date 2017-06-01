//! A BLE application full of holes

#![feature(const_fn)]
#![no_std]

#[macro_use]
extern crate ble400;
#[macro_use]
extern crate cortex_m_semihosting;

use core::{mem, ptr};

use ble400::ll;

const APP_TIMER_PRESCALER: u32 = 0;
const APP_TIMER_OP_QUEUE_SIZE: u8 = 4;

const RX_BUF_SIZE: u32 = 256;
const TX_BUF_SIZE: u32 = 256;

const CENTRAL_LINK_COUNT: u8 = 0;
const PERIPHERAL_LINK_COUNT: u8 = 1;

const DEVICE_NAME: &str = "Ferris";

const INTERVAL_UNIT: f64 = 1.25e-3;
const MIN_CONN_INTERVAL: u16 = (20e-3 / INTERVAL_UNIT) as u16;
const MAX_CONN_INTERVAL: u16 = (75e-3 / INTERVAL_UNIT) as u16;
const SLAVE_LATENCY: u16 = 0;
const CONN_SUP_TIMEOUT: u16 = (4_000e-3 / 10e-3) as u16;

const APP_ADV_INTERVAL: u32 = (40e-3 / 0.625e-3) as u32;
const APP_ADV_TIMEOUT_IN_SECONDS: u32 = 180;

const FIRST_CONN_PARAMS_UPDATE_DELAY: u32 =
    APP_TIMER_TICKS!(5_000, APP_TIMER_PRESCALER);

const NEXT_CONN_PARAMS_UPDATE_DELAY: u32 =
    APP_TIMER_TICKS!(30_000, APP_TIMER_PRESCALER);

const MAX_CONN_PARAMS_UPDATE_COUNT: u8 = 3;

unsafe extern "C" fn uart_event_handle(_event: *mut ll::app_uart_evt_t) {
    ll::bsp_board_led_invert(2);
    // ble400::bkpt();
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

    APP_UART_FIFO_INIT!(
        &comm_params,
        RX_BUF_SIZE,
        TX_BUF_SIZE,
        Some(uart_event_handle),
        APP_IRQ_PRIORITY_LOW
    )
        .unwrap()
}

unsafe fn ble_stack_init() {
    let mut clock_lf_cfg: ll::nrf_clock_lf_cfg_t = ll::_NRF_CLOCK_LFCLKSRC();

    ble400::SOFTDEVICE_HANDLER_INIT(&mut clock_lf_cfg, None).unwrap();

    let mut ble_enable_params: ll::ble_enable_params_t = mem::zeroed();
    ble400::check(ll::softdevice_enable_get_default_config(
        CENTRAL_LINK_COUNT,
        PERIPHERAL_LINK_COUNT,
        &mut ble_enable_params,
    )).unwrap();

    ble400::check(ll::softdevice_enable(&mut ble_enable_params)).unwrap();

    ble400::check(ll::softdevice_ble_evt_handler_set(Some(ble_evt_dispatch)))
        .unwrap();
}

unsafe extern "C" fn ble_evt_dispatch(p_ble_evt: *mut ll::ble_evt_t) {
    // NOTE hit when discovered?
    ble400::bkpt()
}

unsafe fn gap_params_init() {
    let mut sec_mode: ll::ble_gap_conn_sec_mode_t = mem::zeroed();

    ble400::BLE_GAP_CONN_SEC_MODE_SET_OPEN(&mut sec_mode);
    ble400::sd_ble_gap_device_name_set(
        &sec_mode,
        DEVICE_NAME.as_ptr(),
        DEVICE_NAME.len() as u16,
    ).unwrap();

    let mut gap_conn_params: ll::ble_gap_conn_params_t = mem::zeroed();

    gap_conn_params.min_conn_interval = MIN_CONN_INTERVAL;
    gap_conn_params.max_conn_interval = MAX_CONN_INTERVAL;
    gap_conn_params.slave_latency = SLAVE_LATENCY;
    gap_conn_params.conn_sup_timeout = CONN_SUP_TIMEOUT;

    ble400::sd_ble_gap_ppcp_set(&gap_conn_params).unwrap();
}

extern "C" fn nus_data_handler(
    _p_nus: *mut ll::ble_nus_s,
    _p_data: *mut u8,
    _length: u16,
) {
    ble400::bkpt();
}

unsafe fn services_init() {
    static mut m_nus: Option<ll::ble_nus_t> = None;

    let mut nus_init: ll::ble_nus_init_t = mem::zeroed();

    nus_init.data_handler = Some(nus_data_handler);

    m_nus = Some(mem::zeroed());
    ble400::check(ll::ble_nus_init(m_nus.as_mut().unwrap(), &nus_init))
        .unwrap();
}

const NUS_SERVICE_UUID_TYPE: u8 = ll::BLE_UUID_TYPE_VENDOR_BEGIN as u8;

static mut m_adv_uuids: [ll::ble_uuid_t; 1] = [
    ll::ble_uuid_t {
        type_: NUS_SERVICE_UUID_TYPE,
        uuid: ll::BLE_UUID_NUS_SERVICE as u16,
    },
];

unsafe extern "C" fn on_adv_evt(evt: ll::ble_adv_evt_t) {
    ll::bsp_board_led_invert(1);

    // hprintln!("{:?}", evt);

    match evt {
        ll::ble_adv_evt_t::BLE_ADV_EVT_FAST => {
            ble400::check(ll::bsp_indication_set(
                ll::bsp_indication_t::BSP_INDICATE_ADVERTISING,
            )).unwrap();
        }
        ll::ble_adv_evt_t::BLE_ADV_EVT_IDLE => {
            ble400::bkpt()
        }
        _ => {}
    }
}

unsafe fn advertising_init() {
    let mut advdata: ll::ble_advdata_t = mem::zeroed();
    let mut scanrsp: ll::ble_advdata_t = mem::zeroed();
    let mut options: ll::ble_adv_modes_config_t = mem::zeroed();

    advdata.name_type = ll::ble_advdata_name_type_t::BLE_ADVDATA_FULL_NAME;
    advdata.include_appearance = false;
    advdata.flags = ll::BLE_GAP_ADV_FLAGS_LE_ONLY_LIMITED_DISC_MODE as u8;

    scanrsp.uuids_complete.uuid_cnt = m_adv_uuids.len() as u16;
    scanrsp.uuids_complete.p_uuids = m_adv_uuids.as_mut_ptr();

    options.ble_adv_fast_enabled = true;
    options.ble_adv_fast_interval = APP_ADV_INTERVAL;
    options.ble_adv_fast_timeout = APP_ADV_TIMEOUT_IN_SECONDS;

    ble400::check(ll::ble_advertising_init(
        &mut advdata,
        &mut scanrsp,
        &mut options,
        Some(on_adv_evt),
        None,
    )).unwrap();
}

extern "C" fn on_conn_params_evt(evt: *mut ll::ble_conn_params_evt_t) {
    ble400::bkpt();
}

extern "C" fn conn_params_error_handler(nrf_error: u32) {
    ble400::bkpt();
}

unsafe fn conn_params_init() {
    let mut cp_init: ll::ble_conn_params_init_t = mem::zeroed();

    cp_init.p_conn_params = ptr::null_mut();
    cp_init.first_conn_params_update_delay = FIRST_CONN_PARAMS_UPDATE_DELAY;
    cp_init.next_conn_params_update_delay = NEXT_CONN_PARAMS_UPDATE_DELAY;
    cp_init.max_conn_params_update_count = MAX_CONN_PARAMS_UPDATE_COUNT;
    cp_init.start_on_notify_cccd_handle = ll::BLE_GATT_HANDLE_INVALID as u16;
    cp_init.disconnect_on_fail = false;
    cp_init.evt_handler = Some(on_conn_params_evt);
    cp_init.error_handler = Some(conn_params_error_handler);

    ble400::check(ll::ble_conn_params_init(&mut cp_init)).unwrap();
}

unsafe fn power_manage() {
    ble400::sd_app_evt_wait().unwrap();
}

fn main() {
    unsafe {
        ll::bsp_board_leds_init();
        ll::bsp_board_led_invert(0);

        ble400::bkpt();

        APP_TIMER_INIT!(APP_TIMER_PRESCALER, APP_TIMER_OP_QUEUE_SIZE, None)
            .unwrap();

        uart_init();
        ble_stack_init();
        gap_params_init();
        services_init();
        advertising_init();
        conn_params_init();

        ble400::check(ll::ble_advertising_start(
            ll::ble_adv_mode_t::BLE_ADV_MODE_FAST,
        )).unwrap();

        ble400::check(ll::app_uart_put(b'H')).unwrap();
        ble400::check(ll::app_uart_put(b'e')).unwrap();

        loop {
            ll::bsp_board_led_invert(0);
            power_manage()
        }
    }
}
