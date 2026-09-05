// Compare the crate's default fold with an experimental try_fold-based override.
use std::hint::black_box;
use std::time::Instant;
use take_until::TakeUntilExt;

mod support;
use support::Candidate;

fn measure(mut run: impl FnMut() -> u64, repetitions: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..repetitions {
        black_box(run());
    }
    start.elapsed().as_secs_f64() * 1e9 / repetitions as f64
}

fn compare<I: Iterator<Item = u64>>(name: &str, make: impl Fn() -> I, stop: u64, n: usize) {
    let candidate =
        || Candidate::new(make(), |x: &u64| *x == black_box(stop)).fold(0u64, u64::wrapping_add);
    let baseline = || {
        make()
            .take_until(|x| *x == black_box(stop))
            .fold(0u64, u64::wrapping_add)
    };
    assert_eq!(candidate(), baseline());
    let consumed = stop.saturating_add(1).min(n as u64) as usize;
    let repetitions = (20_000_000 / consumed).max(2000);
    // Warm both paths and alternate order to reduce timing bias.
    measure(candidate, repetitions);
    measure(baseline, repetitions);
    let mut ratios = Vec::new();
    for sample in 0..11 {
        let (base, new) = if sample % 2 == 0 {
            let base = measure(baseline, repetitions);
            (base, measure(candidate, repetitions))
        } else {
            let new = measure(candidate, repetitions);
            (measure(baseline, repetitions), new)
        };
        ratios.push(new / base);
    }
    ratios.sort_by(f64::total_cmp);
    println!(
        "{name:24} n={n:5} stop={stop:5}: candidate/default median={:.3}, range={:.3}..{:.3}",
        ratios[5], ratios[0], ratios[10]
    );
}

fn main() {
    for n in [32, 4096] {
        let values: Vec<u64> = (0..n as u64).collect();
        for (position, stop) in [
            ("first", 0),
            ("middle", n as u64 / 2),
            ("last", n as u64 - 1),
            ("absent", u64::MAX),
        ] {
            compare(
                &format!("range/{position}"),
                || 0..black_box(n as u64),
                stop,
                n,
            );
            compare(
                &format!("slice/{position}"),
                || black_box(values.as_slice()).iter().copied(),
                stop,
                n,
            );
            // Emit 0..n after map/filter, so stop positions match the other cases.
            compare(
                &format!("map-filter/{position}"),
                || {
                    (0..black_box(n as u64 * 2))
                        .map(|x| x + 1)
                        .filter(|x| x % 2 == 0)
                        .map(|x| x / 2 - 1)
                },
                stop,
                n,
            );
        }
    }
}
