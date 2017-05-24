set -ex

main() {
    ( cd .. && cargo install xargo || true )
    rustup component add rust-src
}

main
