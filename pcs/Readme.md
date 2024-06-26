# Goldilocks Polynomial Commitment Scheme

Commit to polynomials in $\mathbb{F}_p[X]$ with

$$
p = 2^{64} - 2^{32} + 1
$$




The commitment scheme is 

* The scheme is **not zero-knowledge**, zero-knowledge 
* The scheme is **not additive**, i.e. given two commitments $C_P$ and $C_Q$ there is **not** an efficient way to compute $C_{P+Q}$.



## Build, run, test and bench

```
cargo bench --all-features --bench criterion --package goldilocks-pcs
```
