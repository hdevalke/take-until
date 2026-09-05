# Assembly inspection

## Reproduce

The exported functions in `examples/codegen.rs` isolate the same range, slice,
and map/filter/map reductions as the benchmark. They accept runtime arguments
and sum with wrapping addition. Functions ending in `_barrier` additionally
reproduce the benchmark's per-item `black_box(stop)`.

```sh
cargo rustc --offline --release --example codegen -- --emit=asm
cargo +1.85.0 rustc --offline --release --example codegen --target-dir target/codegen-msrv -- --emit=asm
```

Inspect `target/release/examples/codegen-*.s` and
`target/codegen-msrv/release/examples/codegen-*.s`, respectively. Compare exported
`default_*` and `candidate_*` functions. These results use the default x86_64
target CPU, optimization level 3, and no explicit LTO or target-cpu override.

## Findings

Inspected locally on 2026-09-05 with Rust 1.98.0 and 1.85.0.

| Input, without per-item barrier | Rust 1.98.0 | Rust 1.85.0 |
| --- | --- | --- |
| Range | Identical instructions, apart from labels | Candidate increments with one instruction; default uses LEA plus MOV |
| Slice | Identical instructions, apart from labels | Identical instructions, apart from labels |
| Map/filter/map | Default has a single loop header; candidate has separate filter and reduction loop headers with extra register moves | Candidate materializes the predicate as a boolean and adds test/branch handling; default branches directly |

For example, both Rust 1.98 slice implementations reduce to this loop (AT&T
syntax; labels renamed for readability):

```asm
loop:
    cmpq %rcx, %rsi          # reached slice byte length?
    je done
    movq (%rdi,%rcx), %r8    # load next element
    addq %r8, %rax           # include element in sum
    addq $8, %rcx
    cmpq %rdx, %r8           # did element equal stop?
    jne loop
done:
    retq
```

The iterator's flag and abstraction layers disappear in these slice functions.
There is no separate function call for each next(), predicate, or consumer, and
no independent bounds-panic check. The pure predicate comparison can be reordered
with the addition without changing observable behavior.

With the per-item barrier on Rust 1.98, slices still have identical instructions;
ranges have equivalent instructions with different register allocation. Each
predicate evaluation now writes the stop value to a stack slot and compares
against memory. This barrier perturbs the loop rather than merely preventing
whole-function constant folding. A future timing harness should keep black_box
at the input/output boundary for this pure runtime predicate.

## Interpretation

There is no generated-code evidence of a slice penalty in these isolated
functions. Nor is there a general simplification from overriding fold: the
pipeline shape depends on the compiler, and instruction counts alone do not
establish throughput or branch-prediction performance.

These are isolated function bodies, not the exact inlined timing harness.
Inlining context, register allocation, alignment, and code layout can change
the benchmark's assembly. Identical isolated code therefore does not prove that
all observed timing differences were thermal noise, but it makes a claim of an
inherent slice regression unsupported.

Keep the default provisionally. Establishing a performance advantage needs a
better-controlled timing run and inspection of the exact measured hot loops.
