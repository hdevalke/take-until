// Named functions for inspecting optimized assembly. Runtime arguments prevent
// constant folding; the *_barrier functions also model the benchmark's per-item
// black_box on the predicate target.
use take_until::TakeUntilExt;

#[path = "../benches/support/mod.rs"]
mod support;

macro_rules! cases {
    ($range:ident, $slice:ident, $pipeline:ident, $adapter:expr, $target:expr) => {
        #[unsafe(no_mangle)]
        pub fn $range(end: u64, stop: u64) -> u64 {
            $adapter(0..end, |x: &u64| *x == $target(stop)).fold(0u64, u64::wrapping_add)
        }

        #[unsafe(no_mangle)]
        pub fn $slice(values: &[u64], stop: u64) -> u64 {
            $adapter(values.iter().copied(), |x: &u64| *x == $target(stop))
                .fold(0u64, u64::wrapping_add)
        }

        #[unsafe(no_mangle)]
        pub fn $pipeline(end: u64, stop: u64) -> u64 {
            let iter = (0..end)
                .map(|x| x + 1)
                .filter(|x| x % 2 == 0)
                .map(|x| x / 2 - 1);
            $adapter(iter, |x: &u64| *x == $target(stop)).fold(0u64, u64::wrapping_add)
        }
    };
}

cases!(
    default_range,
    default_slice,
    default_pipeline,
    TakeUntilExt::take_until,
    core::convert::identity
);
cases!(
    candidate_range,
    candidate_slice,
    candidate_pipeline,
    support::Candidate::new,
    core::convert::identity
);
cases!(
    default_range_barrier,
    default_slice_barrier,
    default_pipeline_barrier,
    TakeUntilExt::take_until,
    core::hint::black_box
);
cases!(
    candidate_range_barrier,
    candidate_slice_barrier,
    candidate_pipeline_barrier,
    support::Candidate::new,
    core::hint::black_box
);

fn main() {}
