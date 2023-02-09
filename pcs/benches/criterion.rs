//! Runs criterion benches from inside the library. Require the library to be
//! compiled with the `bench` feature.

use goldilocks_pcs as lib;

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();
    lib::bench::group(&mut criterion);
    criterion.final_summary();
}
