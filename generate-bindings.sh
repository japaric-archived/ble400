set -ex

main() {
    local target=thumbv6m-none-eabi
    local version=0.25.0

    bindgen -V | grep $version

    # NOTE(no-doc-comments) See rust-bindgen#426
    bindgen bindings.h \
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
            -I components/libraries/util \
            -I components/softdevice/s130/headers \
            -I components/toolchain \
            -I components/toolchain/cmsis/include \
            -Werror \
            -fshort-enums \
            -target $target \
            > src/ll.rs
}

main
