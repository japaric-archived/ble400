# `ble400`

> [BLE400] + nRF51822 + [S130] + Rust

[BLE400]: http://www.waveshare.com/wiki/BLE400
[S130]: http://developer.nordicsemi.com/nRF5_SDK/nRF5_SDK_v12.x.x/nRF5_SDK_12.3.0_d7731ad.zip

## How to use

Your nRF51 device must be flashed with version S130 of the SoftDevice firmware.
(See next section for instructions)

``` console
$ xargo build --example blinky

$ openocd -f interface/stlink-v2.cfg -f target/nrf51.cfg &

$ arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/examples/blinky

> continue
```

## Flashing SoftDevice

``` console
$ openocd -f (..) &

$ telnet 4444

> nrf51 mass_erase

> flash banks
#0 : nrf51.flash (nrf51) at 0x00000000, size 0x00040000, buswidth 1, chipwidth 1
#1 : nrf51.uicr (nrf51) at 0x10001000, size 0x00000000, buswidth 1, chipwidth 1

> halt
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x000001c2 msp: 0x20003ff0

> flash write_image erase s130_nrf51_2.0.1_softdevice.hex
auto erase enabled
Padding image section 0 with 2112 bytes
using fast async flash loader. This is currently supported
only with ST-Link and CMSIS-DAP. If you have issues, add
"set WORKAREASIZE 0" before sourcing nrf51.cfg to disable it
target halted due to breakpoint, current mode: Thread
xPSR: 0x61000000 pc: 0x2000001e msp: 0x20003ff0
wrote 110592 bytes from file s130_nrf51_2.0.1_softdevice.hex in 4.953488s (21.803 KiB/s)
```

## Customization

You can customize some aspects of the SDK through the `custom_board.h` and
`sdk_config.h` files.

# License

The Rust code and the `shims.c` file in repository are licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

All the other C source and header files come from the nRF5 SDK and preserve
their licensing terms. Check the header of each file for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
