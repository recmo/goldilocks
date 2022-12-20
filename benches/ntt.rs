use goldilocks_ntt::{ntt::Fft, Field};
use rand::Rng;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .compact()
        .init();

    let size = 1_u64 << 32;

    info!(%size, "Benchmarking NTTs");

    let mut rng = rand::thread_rng();
    let mut input: Vec<Field> = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();

    input.as_mut_slice().fft();

    println!("Hello, world!");
}
