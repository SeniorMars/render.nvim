set positional-arguments
alias t := test
alias tt := test_trace
alias b := build

default:
  just --list

install:
  ./install.sh

lint:
  cargo clippy --all-targets --all-features -- -D warnings

build:
  cargo build

test TEST: build
  cargo test --package render --lib -- $1 --exact --nocapture

# will print a stack trace if it crashes
test_trace TEST: build
  env RUST_BACKTRACE=1 cargo test --package render --lib -- $1 --exact --nocapture

clean:
  cargo clean
