# Rust FFI With `lean.h`

Based on [LurkLab's Rust.FFI](https://github.com/lurk-lab/RustFFI.lean).

Added a minimal working example of a rust function which operates on two lean `Nat`s using the Lean API.

I still need to deal with getting functions declared `static inline` in `lean.h` accessible in Rust, since many of these are useful.

## Running

Build/run `Main.lean` with `lake exe ffi`
