use goldilocks_ntt::{
    divisors::{divisors, split},
    permute::transpose_copy,
    utils::{gcd, modexp, modinv},
    Field,
};

// OPT: Propagate sign through variable.

fn shift(var: &str, count: usize) -> String {
    let count = count % 192;
    if count == 0 {
        var.to_string()
    } else if count < 96 {
        format!("({var} << {count})")
    } else {
        format!("(-{})", shift(var, count - 96))
    }
}

fn mul_root_384(var: &str, exp: usize) -> String {
    assert!(exp < 384);
    if exp & 1 == 0 {
        let exp = exp / 2;
        shift(var, exp)
    } else {
        let exp = exp / 2;
        let (a, b) = ((24 + exp) % 192, (168 + exp) % 192);
        match (a < 96, b < 96) {
            (true, true) => format!("({var} << {}) + ({var} << {})", a, b),
            (true, false) => format!("({var} << {}) - ({var} << {})", a, b - 96),
            (false, true) => format!("({var} << {}) - ({var} << {})", b, a - 96),
            (false, false) => format!("-({var} << {}) - ({var} << {})", a - 96, b - 96),
        }
    }
}

fn ntt(vars: &mut [&str]) {
    match vars.len() {
        ..=1 => {}
        2 => naive_2(vars),
        3 => naive_3(vars),
        // 5 => winograd_5(vars),
        5 | 17 | 257 | 65537 => rader(vars),
        n => {
            let a = split(n);
            let b = n / a;
            assert!(a >= 2 && b >= 2);
            if gcd(a, b) == 1 {
                good_thomas(vars, (a, b));
            } else {
                cooley_tukey(vars, (a, b));
            }
        }
    }
}

pub fn naive_2(vars: &mut [&str]) {
    let [a, b] = vars else { panic!() };
    println!("        let ({a}, {b}) = ({a} + {b}, {a} - {b});");
}

/// Naive 3-point NTT.
///
/// For size 3 the Rader and Winograd algorithms are identical and result in
/// six additions and two multiplications. The naive algorithm has six additions
/// and four multiplications, but the multiplications can be done as bit-shifts.
pub fn naive_3(vars: &mut [&str]) {
    let [a, b, c] = vars else { panic!() };
    println!("        let ({a}, {b}, {c}) = ({a} + {b} + {c},");
    println!("            {a} + ({b} << 64) - ({c} << 32),");
    println!("            {a} - ({b} << 32) + ({c} << 64));");
}

pub fn winograd_5(vars: &mut [&str]) {
    let [a0, a1, a2, a3, a4] = vars else { panic!() };
    println!(
        "
        let s1 = {a1} + {a3};
        let s2 = {a2} + {a4};
        let s3 = s1 + s2;
        let t = {a0};
        let {a0} = {a0} + s3;
        let s4 = s1 - s2;
        let s5 = a1 - a3;
        let s6 = a4 - a2;
        let s7 = s5 - s6;
        let m1 = s3 * Field::new(??);
        let m2 = s4 * Field::new(??);
        let m3 = s5 * Field::new(??);
        let m4 = s6 * Field::new(??);
        let m5 = s7 * Field::new(??);
        let m1 = m1 + t;
        let t1 = m1 + m2;
        let t2 = m3 + m5;
        let t3 = m1 - m2;
        let t4 = m4 - m5;
        let {a1} = t1 + t2;
        let {a2} = t3 + t4;
        let {a3} = t1 - t2;
        let {a4} = t3 - t4;
    "
    );
}

fn rader(vars: &mut [&str]) {
    let n = vars.len();

    // Construct permutations.
    let (gi, gk) = match n {
        2 => (1, 1),
        3 => (2, 2),
        5 => (2, 3),
        17 => (3, 6),
        257 => (3, 86),
        65537 => (3, 21846),
        _ => panic!("Size {n} not supported by Rader NTT"),
    };
    let pi = |i| modexp(gi, i, n);
    let pk = |i| modexp(gk, i, n);

    // Permute input
    let mut buffer = vec![""; n - 1];
    for i in 0..n - 1 {
        buffer[i] = vars[pk(i)];
    }

    // Transform using n-1 sized transform
    ntt(&mut buffer);

    // Add `values[1..].sum()` to `value[0]`. `buffer[0]` conveniently contains
    // this sum after the NTT.
    println!("        let t = {};", vars[0]);
    println!("        let {} = {} + {};", vars[0], vars[0], buffer[0]);

    // Apply twiddles
    let twiddles = {
        let mut twiddles = vec![Field::new(0); n - 1];
        let root = Field::root(n as u64).unwrap();
        let scale = Field::from((n - 1) as u64).inv();
        for i in 0..n - 1 {
            let pi = modexp(gi, i, n);
            twiddles[i] = root.pow(pi as u64) * scale;
        }
        goldilocks_ntt::ntt::ntt(twiddles.as_mut_slice());
        twiddles
    };

    for (i, t) in twiddles.iter().enumerate() {
        let v = buffer[i];
        println!("        let {v} = {v} * Field::new({t});");
    }

    // Add x0 to all `values[1..]` terms by adding to the constant term before INTT.
    println!("        let {} = {} + t;", buffer[0], buffer[0]);

    // Transform back
    buffer[1..].reverse();
    ntt(&mut buffer);

    // Permute into results
    for i in 0..n - 1 {
        vars[pi(i)] = buffer[i];
    }
}

fn cooley_tukey(vars: &mut [&str], (a, b): (usize, usize)) {
    let n = vars.len();
    assert_eq!(a * b, n);
    assert!(a >= 2 && b >= 2);
    // Interpret vars as a 2D array of size a x b
    transpose_copy(vars, (a, b));
    // Now vars is a 2D array of size b x a
    vars.chunks_exact_mut(a).for_each(ntt);
    for i in 1..b {
        for j in 1..a {
            let var = vars[i * a + j];
            let exp = i * j;
            let order = n;
            let g = gcd(exp, order);
            let (exp, order) = (exp / g, order / g);
            if 384 % order == 0 {
                let exp = exp * 384 / order;
                println!("        let {var} = {};", mul_root_384(var, exp));
            } else {
                let omega: u64 = Field::root(order as u64).unwrap().pow(exp as u64).into();
                println!("        let {var} = {var} * Field::new({omega});")
            }
        }
    }
    transpose_copy(vars, (b, a));
    // Now vars is a 2D array of size a x b
    vars.chunks_exact_mut(b).for_each(ntt);
    transpose_copy(vars, (a, b));
}

fn good_thomas(vars: &mut [&str], (a, b): (usize, usize)) {
    let n = a * b;
    debug_assert_eq!(vars.len(), n);
    assert!(a >= 2 && b >= 2);

    // Find parameters
    // See C.S. Burrus (2018) eq. (10.5).
    let (k1, k2) = (b, a);
    let (k3, k4) = (modinv(b, a) * b, modinv(a, b) * a);
    let permute_i = |i| {
        let (i1, i2) = (i / b, i % b);
        (i1 * k1 + i2 * k2) % n
    };
    let permute_k = |i| {
        let (i1, i2) = (i % a, i / a);
        (i1 * k3 + i2 * k4) % n
    };

    // Input permutation.
    let mut buffer = vec![""; n];
    for (i, b) in buffer.iter_mut().enumerate() {
        *b = vars[permute_i(i)];
    }

    buffer.chunks_exact_mut(b).for_each(ntt);
    transpose_copy(&mut buffer, (a, b));
    buffer.chunks_exact_mut(a).for_each(ntt);

    // Output permutation.
    for (i, v) in buffer.iter().enumerate() {
        vars[permute_k(i)] = *v;
    }
}

pub fn generate(size: usize) {
    println!(
        r#"/// Size {size} NTT.
///
/// # Panics
/// 
/// Panics if `values.len() != {size}`.
pub fn ntt_{size}(values: &mut [Field]) {{
    debug_assert_eq!(values.len() % {size}, 0);
    for values in values.chunks_exact_mut({size}) {{"#
    );

    let var_strings = (0..size).map(|i| format!("a{i}")).collect::<Vec<_>>();
    let mut vars = var_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    // Generate reads
    for (i, a) in vars.iter().enumerate() {
        println!("        let {a} = values[{i}];");
    }

    ntt(&mut vars);

    // Generate writes
    for (i, a) in vars.iter().enumerate() {
        println!("        values[{i}] = {a};");
    }
    println!("    }}\n}}\n");
}

fn main() {
    let sizes = divisors()
        .iter()
        .map(|n| *n as usize)
        .filter(|&s| (2..=128).contains(&s))
        .collect::<Vec<_>>();

    // Generate header and dispatch function
    println!(
        "//! Generated using `cargo run --bin codegen`
#![allow(
    unused_parens,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::too_many_lines
)]
use super::{{Ntt, NttFn}};
use crate::Field;
use std::sync::Arc;

pub fn ntt(size: usize) -> Option<Arc<dyn Ntt>> {{
    let f = match size {{
        0 | 1 => ntt_01,"
    );
    for s in &sizes {
        println!("        {s} => ntt_{s},");
    }
    println!(
        "        _ => return None,
    }};
    Some(Arc::new(NttFn::new(size, f)))
}}

pub fn ntt_01(_values: &mut [Field]) {{}}

"
    );

    // Generate the NTTs
    for &s in &sizes {
        generate(s);
    }

    // Generate tests
    println!(
        "#[cfg(test)]
mod tests {{
    use super::{{super::tests::test_ntt, *}};
    "
    );

    for s in &sizes {
        println!(
            r#"
    #[test]
    fn test_ntt_{s}() {{
        test_ntt(ntt({s}).unwrap());
    }}"#
        );
    }

    println!(
        r#"}}
    
#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {{
    use super::{{super::bench::bench_ntt, *}};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {{"#
    );
    for s in &sizes {
        println!("        bench_ntt(criterion, \"small\", ntt({s}).unwrap());");
    }
    println!("    }}");
    println!("}}");
}
