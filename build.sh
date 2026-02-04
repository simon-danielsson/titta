#!/usr/bin/env bash

cargo build --release
# cargo run --release -- -e -i -w -a ~/dev/rust/titta
cargo run --release -- help
# cargo run --release -- tree 2 -e -i -w ~/dev/rust/titta
# cargo run --release -- -i -w -a ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w ~/dev/rust/
# /Users/simondanielsson/dev/rust/titta/target/release/titta -i -w -a ..

