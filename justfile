test-lib-bsa *args="":
  cargo test -p bsa {{args}}

run-lib-bsa-bin name="unpack":
  cargo run -p bsa --bin {{name}}

