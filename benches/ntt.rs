use goldilocks_ntt::{ntt::Fft, Field};
use rand::Rng;
use rayon::prelude::*;
use tracing::{info, instrument, trace};
use tracing_flame::FlameLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, registry::Registry, util::SubscriberInitExt,
};

#[instrument()]
fn sample_data(size: usize) -> Vec<Field> {
    let mut result = vec![Field::from(0); size];
    result.par_iter_mut().for_each(|x| *x = rand::random());
    result
}

fn main() {
    // Set up logging and tracing
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .compact();
    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();
    let flame_layer = flame_layer.with_threads_collapsed(true);
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();

    let size = 1_u64 << 32;

    info!(%size, "Benchmarking NTTs");

    let mut input = sample_data(size as usize);
    input.as_mut_slice().fft();
}
