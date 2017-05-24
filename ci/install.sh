set -ex

main() {
    local url=https://developer.arm.com/-/media/Files/downloads/gnu-rm/6_1-2017q1/gcc-arm-none-eabi-6-2017-q1-update-linux.tar.bz2

    curl -L $url | tar -C $GCC_DIR --strip-components 1 -xj

    ( cd .. && cargo install xargo || true )
    rustup component add rust-src
}

main
