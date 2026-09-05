# Take Until

[![Actions Status](https://github.com/hdevalke/take-until/workflows/Rust/badge.svg)](https://github.com/hdevalke/take-until/actions)


This crate adds the `take_until` method as an extension for iterators.

## Examples

### Parsing the next base 128 varint from a byte slice.

```rust
use take_until::TakeUntilExt;

let varint = &[0b1010_1100u8, 0b0000_0010, 0b1000_0001];
let int: u32 = varint
    .iter()
    .take_until(|b| (**b & 0b1000_0000) == 0)
    .enumerate()
    .fold(0, |acc, (i, b)| {
        acc | ((*b & 0b0111_1111) as u32) << (i * 7)
     });
assert_eq!(300, int);
```

### Take Until vs Take While (from Standard Library)
```rust
use take_until::TakeUntilExt;

fn main() {
    let items = [1, 2, 3, 4, -5, -6, -7, -8];
    let filtered_take_while = items
        .into_iter()
        .take_while(|x| *x > 0)
        .collect::<Vec<i32>>();
    let filtered_take_until = items
        .into_iter()
        .take_until(|x| *x <= 0)
        .collect::<Vec<i32>>();
    assert_eq!([1, 2, 3, 4], filtered_take_while.as_slice());
    assert_eq!([1, 2, 3, 4, -5], filtered_take_until.as_slice());
}
```

The library supports `no_std` without allocation or feature flags.

## MSRV
The MSRV is `1.85.0` stable.

## Publishing

Releases are published to crates.io when a `v*` tag is pushed. The tag must
exactly match `v` followed by the version in `Cargo.toml`. Publishing runs only
after all Rust CI checks and a packaging dry run pass.

The workflow uses [trusted publishing](https://crates.io/docs/trusted-publishing)
to obtain a temporary token; no crates.io API token secret is needed in GitHub.

To release:

1. Update `version` in `Cargo.toml` to a new, unpublished version.
2. Commit the version bump and push the commit containing the publishing workflow.
3. Create and push the matching tag, replacing `X.Y.Z` below with that version:

   ```sh
   git tag -a vX.Y.Z -m "chore: release take-until X.Y.Z"
   git push origin vX.Y.Z
   ```

Monitor the Publish workflow in GitHub Actions. The current manifest version is
`0.2.0`; choose a new version before releasing instead of reusing an existing tag.

## Performance

The inclusive stopping item lets `size_hint()` guarantee one item whenever the
underlying iterator guarantees a nonempty remainder.

Run `cargo bench --bench fold` to compare the default `fold()` with an experimental
override delegating to the underlying iterator's `try_fold()`. The override is
kept outside the library because the timing results are inconclusive. See [benchmark methodology and results](benches/README.md).
