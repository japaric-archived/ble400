set -e

main() {
    local target=thumbv6m-none-eabi
    local version=0.25.0

    bindgen -V | grep $version

    # NOTE(no-doc-comments) See rust-bindgen#426
    local output=$(bindgen bindings.h \
                           --ctypes-prefix=::cty \
                           --no-doc-comments \
                           --use-core \
                           -- \
                           -DBOARD_CUSTOM \
                           -DNRF51 \
                           -DNRF51822 \
                           -DS130 \
                           -DSOFTDEVICE_PRESENT \
                           -I . \
                           -I components/boards \
                           -I components/device \
                           -I components/drivers_nrf/hal \
                           -I components/libraries/timer \
                           -I components/libraries/uart \
                           -I components/libraries/util \
                           -I components/softdevice/s130/headers \
                           -I components/toolchain \
                           -I components/toolchain/cmsis/include \
                           -fshort-enums \
                           -target $target)

    echo "$output" | sed -e '/automatically generated/,$d'
    echo "$output" | sed -n -e '/automatically generated/,$p' > src/ll.rs
}

main
