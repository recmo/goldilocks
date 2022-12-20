# Goldilocks-NTT


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
cargo bench --bench criterion --features="bench"
```

Run NTT benchmarks

```sh
cargo bench --bench ntt
```

Check documentation coverage

```sh
RUSTDOCFLAGS="-Z unstable-options --show-coverage"  cargo doc --workspace --all-features --no-deps
```

## See also

* [plonky2]
* [winterfell]

[plonky]: https://github.com/mir-protocol/plonky2/blob/d90a0559297366e1e2390cff9e3d1d5cf53875b7/field/src/goldilocks_field.rs
[winterfell]: https://github.com/novifinancial/winterfell/blob/21173bdf3e552ca7662c7aa2d34515b084ae21b0/math/src/field/f64/mod.rs
