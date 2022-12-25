
download DAY:
    aoc download -I --day {{DAY}} --input input/day{{DAY}}

debug DAY:
    RUST_LOG=trace cargo run --bin=day{{DAY}} < input/day{{DAY}}

run DAY:
    cargo run --release --bin=day{{DAY}} < input/day{{DAY}}

new DAY:
    cp template.rs src/bin/day{{DAY}}.rs

test:
    cargo test

install:
    cargo install aoc-cli
