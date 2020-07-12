# antic-rs

This is a wrapper for the
[antic library for algebraic number theory](https://github.com/wbhart/antic).

Great library in terms of functionality, but the API is terrible
because it's written in C! No borrow checking, no lifetime management,
and worst of all, no RAII, so you have to remember to initialise and
free objects yourself.

All of this can be solved by providing a safe Rust wrapper library.

This is mainly implemented for benchmarks in the code for
[our cyclotomic field library](https://github.com/CyclotomicFields/cyclotomic).

To compile this you need to have antic and flint installed.

## bindgen invocation

The bindgen-generated code is included since it must be manually
edited after generation to compile. The process I use to do this is:

* Invoke bindgen:
  ```
  $ bindgen /usr/local/include/antic/nf_elem.h > src/bindings.rs
  ```

* Fix all of the compilation errors manually until `cargo test` runs
  successfully.
