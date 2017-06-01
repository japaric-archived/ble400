set -ex

main() {
    local target=thumbv6m-none-eabi
    local version=0.25.3

    bindgen -V | grep $version

    set +x
    # NOTE(no-doc-comments) See rust-bindgen#426
    local output=$(bindgen bindings.h \
                           --ctypes-prefix=::cty \
                           --no-doc-comments \
                           --use-core \
                           -- \
                           -DBLE_STACK_SUPPORT_REQD \
                           -DBOARD_CUSTOM \
                           -DNRF51 \
                           -DNRF51822 \
                           -DNRF_SD_BLE_API_VERSION=2 \
                           -DS130 \
                           -DSOFTDEVICE_PRESENT \
                           -DSWI_DISABLE0 \
                           -I . \
                           -I components/ble/ble_advertising \
                           -I components/ble/ble_services/ble_nus \
                           -I components/ble/common \
                           -I components/boards \
                           -I components/device \
                           -I components/drivers_nrf/hal \
                           -I components/libraries/bsp \
                           -I components/libraries/button \
                           -I components/libraries/timer \
                           -I components/libraries/uart \
                           -I components/libraries/util \
                           -I components/softdevice/common/softdevice_handler \
                           -I components/softdevice/s130/headers \
                           -I components/toolchain \
                           -I components/toolchain/cmsis/include \
                           -fshort-enums \
                           -target $target)

    echo "$output" | sed -e '/automatically generated/,$d'
    echo "$output" | sed -n -e '/automatically generated/,$p' > src/ll.rs
}

main
