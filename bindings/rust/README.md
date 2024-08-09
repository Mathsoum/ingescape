# Rust binding

## Try it out
The binding itself is auto-generated using bindgen from the ingescape C headers.  
Then the lib.rs file includes tests of the "unsafe" generated APIs.

As an exercise, the lib.rs file starts to contain "safe" wrappers of that binding in order to allow safe ingescape code in Rust.

For now, APIs are just unit tested and no real agent has been developed in Rust yet.