# Goldilocks NTT

Fast number theoretic transforms over the 64-bit Goldilocks prime field $\mathbb{F}_p$ with

$$
p = 2^{64} - 2^{32} + 1
$$

## To do

* [ ] Optimize large transpose.
* [ ] Winograd NTT.
* [ ] <https://doi.org/10.1147/rd.222.0134>
* [ ] <https://eccc.weizmann.ac.il/report/2021/103>

## Build run test bench

```sh
cargo fmt &&\
cargo clippy --all-features --all-targets &&\
cargo test --workspace --all-features --doc -- --nocapture &&\
cargo test --workspace --all-features --all-targets -- --nocapture &&\
cargo doc --workspace --all-features --no-deps
```

```
cargo test --all-features -- --test-threads 1 --nocapture
```

Run Criterion benchmarks

```sh
cargo bench --all-features --bench criterion --package goldilocks-ntt

cargo bench --all-features --bench criterion --package goldilocks-pcs
```

Create a baseline

```
cargo bench --all-features --bench criterion -- --save-baseline main
```

```
cargo bench --all-features --bench criterion -- --baseline-lenient main
```


Run NTT benchmarks

```sh
cargo bench --all-features --bench ntt -- ntt 32
```

Check documentation coverage

```sh
RUSTDOCFLAGS="-Z unstable-options --show-coverage"  cargo doc --workspace --all-features --no-deps
```

```
cargo +stable run --bin codegen > small.rs && mv small.rs src/ntt && cargo +stable test ntt::small
```


Inspect assembly using [cargo-show-asm]

[cargo-show-asm]: https://crates.io/crates/cargo-show-asm

```
cargo asm --lib -p goldilocks-ntt "ntt::small::ntt_2" 0
```

```
RUSTFLAGS="-Awarnings" cargo asm --lib --all-features -p goldilocks-ntt --rust "field::algo::generic::mul_redc_2"
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

https://users.ece.cmu.edu/~franzf/papers/fft-enc11.pdf

https://github.com/ejmahler/RustFFT

https://gitlab.com/hatsya/open-source/cpads/-/blob/master/include/cpads/algebra/ff64.hpp

https://github.com/mir-protocol/plonky2/issues/1

https://github.com/mir-protocol/plonky2/issues/8
