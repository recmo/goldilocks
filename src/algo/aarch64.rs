use core::arch::asm;

// See <https://courses.cs.washington.edu/courses/cse469/19wi/arm64.pdf>
// See <https://dougallj.github.io/applecpu/firestorm.html>
// See <https://7-cpu.com/cpu/Apple_M1.html>
// See <https://gist.github.com/recmo/389339a2e71825eadad1e56106b2e24b>

// TODO: Prefetching

// TODO: Associativity issues? https://en.algorithmica.org/hpc/cpu-cache/associativity/

/// Transpose swap two 4x4 matrices in memory.
///
///
/// [  0  1  2  3]         [ 0  4  8 12]
/// [  4  5  6  7]   /==\  [ 1  5  9 13]
/// [  8  9 10 11]   \==/  [ 2  6 10 14]
/// [ 12 13 14 15]         [ 3  7 11 15]
pub unsafe fn transpose_swap_4(a: *mut u64, b: *mut u64, stride: usize) {
    asm!(
        r#"
        ; Load a pair of registers with non-temporal hint. This loads 256 bits
        ; or 4 64-bit integers. Therefore we swap a 4x4 matrix at a time.

        dmb nshld ; memory barrier to make sure addresses are loaded correctly
                  ; see <https://developer.arm.com/documentation/den0024/a/Memory-Ordering/Barriers/Non-temporal-load-and-store-pair>
        
        ; Prefetch A matrix reads
        prfm pldl1strm, [{a0}, #960]
        prfm pldl1strm, [{a1}, #960]
        prfm pldl1strm, [{a2}, #960]
        prfm pldl1strm, [{a3}, #960]

        ; Prefetch B matrix reads
        add {temp}, {b0}, {b_prefetch}, lsl #3
        prfm pldl1strm, [{temp}]

        add {temp}, {b1}, {b_prefetch}, lsl #3
        prfm pldl1strm, [{temp}]

        add {temp}, {b2}, {b_prefetch}, lsl #3
        prfm pldl1strm, [{temp}]

        add {temp}, {b3}, {b_prefetch}, lsl #3
        prfm pldl1strm, [{temp}]

        ; Read the whole first matrix
        ldnp q0, q1, [{a0}]  ; v0 = A[ 0  1]  v1 = A[ 2  3]
        ldnp q2, q3, [{a1}]  ; v2 = A[ 4  5]  v3 = A[ 6  7]
        ldnp q4, q5, [{a2}]  ; v4 = A[ 8  9]  v5 = A[10 11]
        ldnp q6, q7, [{a3}]  ; v6 = A[12 13]  v7 = A[14 15]

        ; Read most of the second matrix
        ldnp  q8,  q9, [{b0}]   ; v8  = B[ 0  4]   v9 = B[ 8 12]
        ldnp q10, q11, [{b1}]   ; v10 = B[ 1  5]  v11 = B[ 9 13]
        ldnp q12, q13, [{b2}]   ; v12 = B[ 2  6]  v12 = B[10 14]

        ; Write the first two rows of the second matrix
        trn1.2d v14, v0, v2    ; v14 = A[ 0  4]
        trn1.2d v15, v4, v6    ; v15 = A[ 8 12]
        stnp q14, q15, [{b0}]   ;

        trn2.2d v14, v0, v2    ; v14 = A[ 1  5]
        trn2.2d v15, v4, v6    ; v15 = A[ 9 13]
        stnp q14, q15, [{b1}]   ;

        ; v0, v2, v4, v6 are now free. Load the rest of B

        ; Read the rest of B
        ldnp q14, q15, [{b3}]   ; v14 = B[ 3  7]  v15 = B[11 15]

        ; Write the rest of B
        trn1.2d v0, v1, v3    ; v0 = A[ 2  6]
        trn1.2d v2, v5, v7    ; v2 = A[10 14]
        stnp q0, q2, [{b2}]

        trn2.2d v0, v1, v3    ; v0 = A[ 3  7]
        trn2.2d v2, v5, v7    ; v2 = A[11 15]
        stnp q0, q2, [{b3}]   ;

        trn1.2d v4,  v8, v10    ; v4 = B[ 0  1]
        trn1.2d v6, v12, v14    ; v6 = B[ 3  3]
        stnp q4, q6, [{a0}]

        trn2.2d v4,  v8, v10    ; v4 = B[ 4  5]
        trn2.2d v6, v12, v14    ; v6 = B[ 6  7]
        stnp q4, q6, [{a1}]

        trn1.2d v4,  v9, v11    ; v4 = B[ 8  9]
        trn1.2d v6, v13, v15    ; v6 = B[10 11]
        stnp q4, q6, [{a2}]

        trn2.2d v4,  v9, v11    ; v4 = B[12 13]
        trn2.2d v6, v13, v15    ; v6 = B[14 15]
        stnp q4, q6, [{a3}]
    "#,
        a0 = in(reg) a,
        a1 = in(reg) a.add(stride),
        a2 = in(reg) a.add(stride * 2),
        a3 = in(reg) a.add(stride * 3),
        b0 = in(reg) b,
        b1 = in(reg) b.add(stride),
        b2 = in(reg) b.add(stride * 2),
        b3 = in(reg) b.add(stride * 3),
        b_prefetch = in(reg) 30 * 4 * stride,
        temp = out(reg) _,
        options(nostack)
    )
}

pub fn transpose(a: &mut [u64], size: usize) {
    assert_eq!(a.len(), size * size);
    assert_eq!(size % 4, 0);

    for i in (0..size).step_by(4) {
        for j in (i..size).step_by(4).skip(1) {
            let (a, b) = a.split_at_mut(size * size);
            unsafe {
                transpose_swap_4(
                    a.as_mut_ptr().add(i * size + j),
                    a.as_mut_ptr().add(j * size + i),
                    size,
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::generic, *};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_transpose_swap_4() {
        let mut a = [0u64; 16];
        let mut b = [0u64; 16];
        for i in 0..16 {
            a[i] = i as u64;
            b[i] = (i + 100) as u64;
        }
        for i in 0..4 {
            for j in 0..4 {
                print!("{:3} ", a[i * 4 + j]);
                // assert_eq!(a[i * 4 + j], (i + 100) as u64);
                // assert_eq!(b[i * 4 + j], j as u64);
            }
            println!("");
        }
        println!("");
        for i in 0..4 {
            for j in 0..4 {
                print!("{:3} ", b[i * 4 + j]);
                // assert_eq!(a[i * 4 + j], (i + 100) as u64);
                // assert_eq!(b[i * 4 + j], j as u64);
            }
            println!("");
        }
        println!("");

        unsafe {
            transpose_swap_4(a.as_mut_ptr(), b.as_mut_ptr(), 4);
        }

        for i in 0..4 {
            for j in 0..4 {
                print!("{:3} ", a[i * 4 + j]);
                // assert_eq!(a[i * 4 + j], (i + 100) as u64);
                // assert_eq!(b[i * 4 + j], j as u64);
            }
            println!("");
        }
        println!("");
        for i in 0..4 {
            for j in 0..4 {
                print!("{:3} ", b[i * 4 + j]);
                // assert_eq!(a[i * 4 + j], (i + 100) as u64);
                // assert_eq!(b[i * 4 + j], j as u64);
            }
            println!("");
        }
        todo!()
    }
}

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use core::hint::black_box;
    use criterion::{BatchSize, Criterion, Throughput};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        bench_transpose_4(criterion);
        bench_transpose(criterion);
    }

    pub fn bench_transpose_4(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/aarch64/transpose_swap_4");
        let size = 4;
        group.throughput(Throughput::Elements(4));
        group.bench_function(format!("4x4"), |b| {
            b.iter_batched_ref(
                || {
                    (
                        (0..16).map(|_| thread_rng().gen()).collect::<Vec<u64>>(),
                        (0..16).map(|_| thread_rng().gen()).collect::<Vec<u64>>(),
                    )
                },
                |(a, b)| unsafe { transpose_swap_4(a.as_mut_ptr(), b.as_mut_ptr(), 4) },
                BatchSize::LargeInput,
            )
        });
        group.finish();
    }

    pub fn bench_transpose(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/aarch64");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || {
                        (0..size * size)
                            .map(|_| thread_rng().gen())
                            .collect::<Vec<u64>>()
                    },
                    |m| transpose(m.as_mut_slice(), size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }
}
