extern crate gcc;

use gcc::Config;

fn main() {
    let mut config = Config::new();

    config.flag("-std=c99");
    config.flag("-mcpu=cortex-m0");
    config.flag("-mthumb");
    config.flag("-mabi=aapcs");
    config.flag("-mfloat-abi=soft");
    config.flag("-ffunction-sections");
    config.flag("-fdata-sections");
    config.flag("-fno-strict-aliasing");
    config.flag("-fno-builtin");
    config.flag("--short-enums");

    config.define("BLE_STACK_SUPPORT_REQD", None);
    config.define("BOARD_CUSTOM", None);
    config.define("NRF51", None);
    config.define("NRF51822", None);
    config.define("NRF_SD_BLE_API_VERSION", Some("2"));
    config.define("S130", None);
    config.define("SOFTDEVICE_PRESENT", None);
    config.define("SWI_DISABLE0", None);

    config.file("shims.c");

    config.file("components/libraries/log/src/nrf_log_backend_serial.c");
    config.file("components/libraries/log/src/nrf_log_frontend.c");
    config.file("components/libraries/util/app_error.c");
    config.file("components/libraries/util/app_error_weak.c");
    config.file("components/libraries/fifo/app_fifo.c");
    config.file("components/libraries/timer/app_timer.c");
    config.file("components/libraries/uart/app_uart_fifo.c");
    config.file("components/libraries/util/app_util_platform.c");
    config.file("components/libraries/fstorage/fstorage.c");
    config.file("components/libraries/hardfault/hardfault_implementation.c");
    config.file("components/boards/boards.c");
    config.file("components/drivers_nrf/common/nrf_drv_common.c");
    config.file("components/drivers_nrf/clock/nrf_drv_clock.c");
    config.file("components/drivers_nrf/uart/nrf_drv_uart.c");
    config.file("components/libraries/bsp/bsp.c");
    config.file("components/ble/common/ble_advdata.c");
    config.file("components/ble/ble_advertising/ble_advertising.c");
    config.file("components/ble/common/ble_conn_params.c");
    config.file("components/toolchain/gcc/gcc_startup_nrf51.S");
    config.file("components/toolchain/system_nrf51.c");
    config.file("components/ble/ble_services/ble_nus/ble_nus.c");
    config.file(
        "components/softdevice/common/softdevice_handler/softdevice_handler.c",
    );

    // FIXME sdk_config.h shouldn't be hardcoded
    config.include("."); // sdk_config.h
    config.include("components/ble/common");
    config.include("components/boards");
    config.include("components/device");
    config.include("components/drivers_nrf/clock");
    config.include("components/drivers_nrf/common");
    config.include("components/drivers_nrf/delay");
    config.include("components/drivers_nrf/hal");
    config.include("components/drivers_nrf/uart");
    config.include("components/libraries/bsp");
    config.include("components/libraries/button");
    config.include("components/libraries/experimental_section_vars");
    config.include("components/libraries/fifo");
    config.include("components/libraries/fstorage");
    config.include("components/libraries/log");
    config.include("components/libraries/log/src");
    config.include("components/libraries/timer");
    config.include("components/libraries/util");
    config.include("components/softdevice/common/softdevice_handler");
    config.include("components/softdevice/s130/headers");
    config.include("components/toolchain");
    config.include("components/toolchain/cmsis/include");

    config.compile("libsoftdevice.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=custom_board.h");
    println!("cargo:rerun-if-changed=sdk_config.h");
    println!("cargo:rerun-if-changed=shims.c");
    println!("cargo:rerun-if-changed=shims.h");
}
