# Goldilocks-NTT

## To do

* [ ] Transpose for {1, 3} × 2^k.
* Look into Rader FFT to turn prime sized FFTs into (p - 1) sized.

## Build run test bench

```sh
cargo fmt &&\
cargo clippy --all-features --all-targets &&\
cargo test --workspace --all-features --doc -- --nocapture &&\
cargo test --workspace --all-features --all-targets -- --nocapture &&\
cargo doc --workspace --all-features --no-deps
```

Run Criterion benchmarks

```sh
cargo bench --bench criterion --features="criterion"
```

Run NTT benchmarks

```sh
cargo bench --bench ntt --features="rand,rayon"
# cargo install inferno
inferno-flamegraph < tracing.folded  > tracing-flamegraph.svg
```

Check documentation coverage

```sh
RUSTDOCFLAGS="-Z unstable-options --show-coverage"  cargo doc --workspace --all-features --no-deps
```

```
cargo run --bin codegen > small.rs && mv small.rs src/ntt && cargo test ntt::small
```

## See also

* [plonky2]
* [winterfell]
* [risc0]

[plonky]: https://github.com/mir-protocol/plonky2/blob/d90a0559297366e1e2390cff9e3d1d5cf53875b7/field/src/goldilocks_field.rs
[winterfell]: https://github.com/novifinancial/winterfell/blob/21173bdf3e552ca7662c7aa2d34515b084ae21b0/math/src/field/f64/mod.rs
[risc0]: https://github.com/risc0/risc0/blob/main/risc0/zkp/src/field/goldilocks.rs


http://fftw.org/fftw-paper-ieee.pdf

https://netlib.org/utk/people/JackDongarra/CCDSC-2014/talk35.pdf
