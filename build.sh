#!/usr/bin/env bash

cargo build --release
# cargo run --release -- -e -w -a ~/dev/rust/titta/
# cargo run --release -- help
cargo run --release -- tree 1 -w -a
# cargo run --release -- -i -w -a ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~/dev/rust/
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w -a ..

