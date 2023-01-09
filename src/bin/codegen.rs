use goldilocks_ntt::{ntt::recursive::divisor_split, permute::transpose_copy};

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
            for vars in vars.chunks_exact(2) {
                let [a, b] = vars else { unreachable!() };
                println!("    let ({a}, {b}) = ({a} + {b}, {a} - {b});");
            }
        } else {
            let a = divisor_split(n);
            let b = n / a;
            assert_eq!(a * b, n);
            assert!(a >= 2 && b >= 2);
            // Interpret vars as a 2D array of size a x b
            transpose_copy(vars, (a, b));
            // Now vars is a 2D array of size b x a
            vars.chunks_exact_mut(a).for_each(recurse);
            for i in 1..a {
                for j in 1..b {
                    let exp = (i * j) * 384 / n;
                    let var = vars[i * a + j];
                    println!("    let {var} = {var}.mul_root_384({exp});")
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
    let sizes = [2, 4, 8]; // , 16, 32, 64, 128];

    // Generate header and dispatch function
    println!(
        "{}",
        r#"//! Generated using `cargo run --bin codegen`
use crate::Field;

/// Apply a small NTT to `values`, or return `false` if the size is not supported.
pub fn small_ntt(values: &mut [Field]) -> bool {
    match values.len() {
        ..=1 => return true,"#
    );
    for s in sizes {
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
    for s in sizes {
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
