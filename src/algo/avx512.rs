
use std::arch::x86_64::*;

// Store 512-bit integer into memory without polluting cache.
// https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_stream_si512.html

// https://vgatherps.github.io/2018-09-02-nontemporal/
// https://en.algorithmica.org/hpc/cpu-cache/bandwidth/#bypassing-the-cache
