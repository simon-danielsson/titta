#!/usr/bin/env bash

cargo build --release
# cargo run --release -- -i -w -a ~/dev/rust
cargo run --release -- -i -w -a ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~/dev/rust/
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w -a ..

