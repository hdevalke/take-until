# Fold benchmark

Run `cargo bench --bench fold` (release profile, no external dependencies).
The baseline is the library adapter using the default `Iterator::fold`; the
candidate in `support/mod.rs` has equivalent `next()` behavior and overrides
`fold()` using the underlying iterator's `try_fold()` and `ControlFlow`.
Integration tests compare its output, predicate/consumer ordering, partial
consumption, and non-fused behavior with the baseline and repeated `next()`.

The benchmark covers 32 and 4096 elements, ranges, copied slices, and a
map/filter/map pipeline emitting the same values. Predicates stop at the first,
middle, or last item, or never match. Both paths use identical wrapping-add
consumers and inputs, with `black_box` on inputs, predicate targets, and results.
Each case warms both paths, then measures 11 pairs with alternating order.
Each sample processes approximately 20 million output items (at least 2000
iterations). Reported ratios are candidate time divided by default time;
**below 1 is faster**. The output includes medians and sample ranges.

## Local results and decision

Two consecutive runs on an Intel Core i5-8250U, x86_64 Linux, with
rustc 1.98.0 (88d9e12ae 2026-08-18), on 2026-09-05:

| Case (4096 elements) | Run 1 ratio | Run 2 ratio |
| --- | ---: | ---: |
| range/first | 0.933 | 0.948 |
| slice/first | 1.095 | 1.066 |
| map-filter/first | 0.745 | 0.715 |
| range/middle | 0.885 | 0.865 |
| slice/middle | 0.947 | 0.949 |
| map-filter/middle | 0.765 | 0.782 |
| range/last | 0.876 | 0.872 |
| slice/last | 0.974 | 0.904 |
| map-filter/last | 0.760 | 0.741 |
| range/absent | 0.875 | 0.858 |
| slice/absent | 0.899 | 0.996 |
| map-filter/absent | 0.745 | 0.760 |

Keep the default fold provisionally. The timing results are inconclusive on this
laptop: median ratios favor composed pipelines and disfavor first-item slices,
but thermal/frequency effects were not controlled. Earlier measurements with the candidate inside
the library and a default-fold wrapper also showed regressions in longer slice
cases. Moving the candidate into benchmark support changed several results,
highlighting sensitivity to generated code and code layout.

These are local microbenchmarks with noticeable timing variability, not a
portable performance guarantee. The measurements do not establish whether the candidate meets the requested
acceptance rule of repeatable gains without material regressions.
It remains available for testing on other hardware and future compilers.

See [assembly inspection](ASSEMBLY.md) for compiler-generated code comparisons.
