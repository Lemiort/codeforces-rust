# Codeforces Rust Solutions

`src/bin` is Cargo's convention for multiple standalone binaries. Each
problem is a separate executable named with its Codeforces ID:

```text
src/bin/s808D_array_division.rs
```

The solution and its unit tests live in the same file. Add future problems as
`src/bin/s<problem-id>_<short_desc_english>.rs`; Cargo discovers them
automatically, without a central dispatcher or `Cargo.toml` entry.

## New solution

Copy `templates/solution.rs` to `src/bin/`, rename it using the problem's hex
code, and implement `solve`:

```text
templates/solution.rs -> src/bin/sABC_short_desc.rs
```

Replace the ignored `sample_case` test with the problem's sample input and
expected output, then enable it by removing `#[ignore]`.

## Commands

Run a solution with stdin:

```text
cargo run --bin s808D_array_division < input.txt
```

Run all solution tests:

```text
cargo test
```

Run only one solution's tests:

```text
cargo test --bin s808D_array_division
```
