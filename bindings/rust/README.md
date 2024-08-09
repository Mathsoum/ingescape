# Rust binding

## Building & testing
Building the binding can be done with `cargo build` directly in this directory.
To run the tests, simply run the script `run_tests.sh`.

**IMPORTANT NOTE**
Rust tests are run in parallel by default.
Doing so make sometimes the tests fails on `char*` values from C null or with the wrong value.
To avoid that behavior, run the tests with `--test-threads 1` like so:
```sh
cargo test -- --test-threads 1
```
The tests should pass when executed in sequence, not in parallel.

As a side note, tests can be run with `--nocapture` so printed text in the tests is not hidden, even when the tests pass.
```sh
cargo test -- --nocapture --test-threads 1
```

The script run_tests.sh simply run this command with those parameters.

## Try it out
The binding itself is auto-generated using bindgen from the ingescape C headers.
Then the lib.rs file includes tests of the "unsafe" generated APIs.

As an exercise, the lib.rs file starts to contain "safe" wrappers of that binding in order to allow safe ingescape code in Rust.

For now, APIs are just unit tested and no real agent has been developed in Rust yet.
