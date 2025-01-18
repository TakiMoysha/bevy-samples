# run across all workspace memebers
check:
  cargo check --workspace

run-cars:
  cargo run --bin cars

lib-tests:
  cargo test --lib
