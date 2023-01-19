use goldilocks_ntt::{
    divisors::{divisors, split},
    permute::transpose_copy,
    utils::{gcd, modexp},
    Field,
};

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
    if exp & 1 == 0 {
        shift(var, exp / 2)
    } else {
        format!(
            "{} + {}",
            shift(var, 24 + (exp / 2)),
            shift(var, 168 + (exp / 2))
        )
    }
}

fn ntt(vars: &mut [&str]) {
    match vars.len() {
        ..=1 => {}
        2 => naive_2(vars),
        3 => naive_3(vars),
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
    println!("    let ({a}, {b}) = ({a} + {b}, {a} - {b});");
}

pub fn naive_3(vars: &mut [&str]) {
    let [a, b, c] = vars else { panic!() };
    println!("    let ({a}, {b}, {c}) = ({a} + {b} + {c},");
    println!("        {a} + ({b} << 64) - ({c} << 32),");
    println!("        {a} - ({b} << 32) + ({c} << 64));");
}

fn rader(vars: &mut [&str]) {
    let n = vars.len();

    // Construct permutations.
    let (gi, gk) = goldilocks_ntt::ntt::rader::parameters(n);
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
    println!("    let t = {};", vars[0]);
    println!("    let {} = {} + {};", vars[0], vars[0], buffer[0]);

    // Apply twiddles
    let twiddles = goldilocks_ntt::ntt::rader::twiddles(n, pi);
    for (i, t) in twiddles.iter().enumerate() {
        let v = buffer[i];
        println!("    let {v} = {v} * Field::new({t});");
    }

    // Add x0 to all `values[1..]` terms by adding to the constant term before INTT.
    println!("    let {} = {} + t;", buffer[0], buffer[0]);

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
                println!("    let {var} = {};", mul_root_384(var, exp));
            } else {
                let omega: u64 = Field::root(order as u64).unwrap().pow(exp as u64).into();
                println!("    let {var} = {var} * Field::new({omega});")
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

    let (k1, k2, k3, k4) = goldilocks_ntt::ntt::good_thomas::parameters(a, b);
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
    assert_eq!(values.len(), {size});"#
    );

    let var_strings = (0..size).map(|i| format!("a{i}")).collect::<Vec<_>>();
    let mut vars = var_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    // Generate reads
    for (i, a) in vars.iter().enumerate() {
        println!("    let {a} = values[{i}];");
    }

    ntt(&mut vars);

    // Generate writes
    for (i, a) in vars.iter().enumerate() {
        println!("    values[{i}] = {a};");
    }
    println!("}}\n");
}

fn main() {
    let sizes = divisors()
        .iter()
        .map(|n| *n as usize)
        .filter(|&s| (2..=128).contains(&s))
        .filter(|&s| s != 102) // This one crashes rustc
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
use crate::Field;

/// Apply a small NTT to `values`, or return `false` if the size is not supported.
pub fn ntt(values: &mut [Field]) -> bool {{
    match values.len() {{
        ..=1 => return true,"
    );
    for s in &sizes {
        println!("        {s} => ntt_{s}(values),");
    }
    println!(
        "        _ => return false,
    }}
    true
}}
"
    );

    // Generate the NTTs
    for &s in &sizes {
        generate(s);
    }

    // Generate tests
    let size_list = sizes
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "#[cfg(test)]
mod tests {{
    use super::{{super::tests::test_ntt_fn, *}};

    #[test]
    fn test_small_ntt() {{
        for size in [0, 1, {size_list}] {{
            test_ntt_fn(|values| assert!(ntt(values)), size);
        }}
    }}"
    );

    for s in &sizes {
        println!(
            r#"
    #[test]
    fn test_ntt_{s}() {{
        test_ntt_fn(ntt_{s}, {s});
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
        println!("        bench_ntt(criterion, \"small\", ntt_{s}, {s});");
    }
    println!("    }}");
    println!("}}");
}
