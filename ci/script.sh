set -ex

main() {
    local e=
    for e in $(ls examples/*); do
        e=$(basename $e)
        e=${e%.*}
        xargo build --example $e
        xargo build --example $e --release
    done
}

main
