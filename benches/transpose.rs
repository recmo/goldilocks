use goldilocks_ntt::{ntt::Fft, Field};
use plonky2_util::transpose_in_place_square;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rayon::prelude::*;
use std::time::Instant;
use tracing::{info, instrument, trace};
use tracing_flame::FlameLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, registry::Registry, util::SubscriberInitExt,
};

#[instrument()]
fn rand_vec<T>(size: usize) -> Vec<T>
where
    T: Send,
    Standard: Distribution<T>,
{
    let mut result = Vec::with_capacity(size);
    (0..size)
        .into_par_iter()
        .map_init(|| rand::thread_rng(), |rng, _| rng.gen::<T>())
        .collect_into_vec(&mut result);
    result
}

fn main() {
    const MAX_SIZE: usize = 1_usize << 32;

    // Set up logging and tracing
    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
    //     .compact();
    // let (flame_layer, _guard) =
    // FlameLayer::with_file("./tracing.folded").unwrap(); let flame_layer =
    // flame_layer.with_threads_collapsed(true); tracing_subscriber::registry()
    //     .with(fmt_layer)
    //     .with(flame_layer)
    //     .init();

    let mut input = rand_vec(MAX_SIZE);

    println!("size,duration,throughput");
    for size in 5..=16 {
        let side = 1_usize << size;
        let size = side * side;
        let input = &mut input[..size];

        let mut duration = 0.0;
        let mut count = 0;
        while duration < 5.0 {
            let start = Instant::now();

            input.fft();
            let end = Instant::now();
            duration += end.duration_since(start).as_secs_f64();
            count += 1;
        }
        duration /= count as f64;
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
