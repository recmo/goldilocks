use goldilocks_ntt::{Field, divisors::{divisors, split, is_smooth}, permute::transpose_copy, utils::gcd};

fn shift(var: &str, count: usize) -> String {
    let count = count % 192;
    if count == 0 {
        format!("{var}")
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
        format!("{} + {}", shift(var, 24 + (exp / 2)), shift(var, 168 + (exp / 2)))
    }
}

pub fn generate(size: usize) {
    println!(
        r#"/// Size {size} NTT.
fn ntt_{size}(values: &mut [Field]) {{
    debug_assert_eq!(values.len(), {size});"#
    );

    let var_strings = (0..size).map(|i| format!("a{i}")).collect::<Vec<_>>();
    let mut vars = var_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    // Generate reads
    for (i, a) in vars.iter().enumerate() {
        println!("    let {a} = values[{i}];");
    }

    fn recurse(vars: &mut [&str]) {
        let n = vars.len();
        if n == 2 {
            let [a, b] = vars else { unreachable!() };
            println!("    let ({a}, {b}) = ({a} + {b}, {a} - {b});");
        } else if n == 3 {
            let [a, b, c] = vars else { unreachable!() };
            println!("    let ({a}, {b}, {c}) = ({a} + {b} + {c},");
            println!("        {a} + ({b} << 64) - ({c} << 32),");
            println!("        {a} - ({b} << 32) + ({c} << 64));");
        } else if n == 5 {
            let [a, b, c, d, e] = vars else { unreachable!() };
            println!("    let ({a}, {b}, {c}, {d}, {e}) = ({a} + {b} + {c} + {d} + {e},");
            println!("        {a} + {b} * R51 + {c} * R52 + {d} * R53 + {e} * R54,");
            println!("        {a} + {b} * R52 + {c} * R54 + {d} * R51 + {e} * R53,");
            println!("        {a} + {b} * R53 + {c} * R51 + {d} * R54 + {e} * R52,");
            println!("        {a} + {b} * R54 + {c} * R53 + {d} * R52 + {e} * R51);");
        } else {
            let a = split(n);
            let b = n / a;
            assert_eq!(a * b, n);
            assert!(a >= 2 && b >= 2);
            // Interpret vars as a 2D array of size a x b
            transpose_copy(vars, (a, b));
            // Now vars is a 2D array of size b x a
            vars.chunks_exact_mut(a).for_each(recurse);
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
            vars.chunks_exact_mut(b).for_each(recurse);
            transpose_copy(vars, (a, b));
        }
    }

    recurse(&mut vars);

    // Generate writes
    for (i, a) in vars.iter().enumerate() {
        println!("    values[{i}] = {a};");
    }
    println!("}}\n");
}

fn main() {
    let sizes = divisors().iter().map(|n| *n as usize).filter(|&s| is_smooth(s) && s >= 2 && s <= 256).collect::<Vec<_>>();

    // Generate header and dispatch function
    println!(
        "{}",
        r#"//! Generated using `cargo run --bin codegen`
#![allow(unused_parens)] // Makes codegen easier
use crate::Field;

const R51: Field = Field::new(0x0e736627a0aeb983);
const R52: Field = Field::new(0xdb8edc802dc0b266);
const R53: Field = Field::new(0x02efb5c2a6f35241);
const R54: Field = Field::new(0x130e07948a9d41d6);

/// Apply a small NTT to `values`, or return `false` if the size is not supported.
pub fn small_ntt(values: &mut [Field]) -> bool {
    match values.len() {
        ..=1 => return true,"#
    );
    for s in &sizes {
        println!("        {s} => ntt_{s}(values),");
    }
    println!(
        "{}",
        r#"        _ => return false,
    }
    true
}
"#
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
        "{}{}{}",
        r#"#[cfg(test)]
mod tests {
    use super::{super::ntt_naive, *};

    #[test]
    fn test_small_ntt() {
        for size in [0, 1, "#,
        size_list,
        r#"] {
            let input = (0..size).map(Field::from).collect::<Vec<_>>();
            let mut output = input.clone();
            let supported = small_ntt(output.as_mut_slice());
            assert!(supported);
            let mut output_ref = input;
            ntt_naive(output_ref.as_mut_slice());
            assert_eq!(output, output_ref);
        }
    }
"#
    );

    for s in sizes {
        println!(
            r#"
    #[test]
    fn test_ntt_{s}() {{
        let size = {s};
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_{s}(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }}
"#
        );
    }

    println!("}}");
}
