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

Generate a solution directly from a Codeforces ID or problem URL:

```text
cargo run --bin new_solution -- 808D
cargo run --bin new_solution -- https://codeforces.com/problemset/problem/808/D
```

The generator fetches the problem metadata and creates
`src/bin/s808D_array_division.rs` from `templates/solution.rs`. The generated
file includes the title, Codeforces link, rating, tags, and a colocated test
placeholder. Replace the ignored `sample_case` test with the problem's sample
input and expected output, then enable it by removing `#[ignore]`.

The generator refuses to overwrite an existing solution. It uses the
Codeforces API, so network access is required when creating a solution.

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
