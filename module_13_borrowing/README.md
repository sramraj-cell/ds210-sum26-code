# Module 13: Borrowing, Permissions, and Aquascope

Each subdirectory is a standalone cargo project grouping related code samples from the
interactive lecture notes. Inside each project, the files in `src/` are numbered in the
order they appear in the lecture. Each file is its own binary: run it with
`cargo run --bin <name>` from inside the project directory (or use the run button
above each `main` in VSCode). Note that the binary name is the file name *without*
the number prefix (cargo does not allow binary names to start with a digit).

Several examples are meant to be explored with
[Aquascope](https://cel.cs.brown.edu/aquascope/) (`interpret` and `permissions` views);
see the comments at the top of each file.

## `1_references_are_safe`

The same dangling-element scenario, with pointers vs. references:

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_with_pointers.rs` | `cargo run --bin with_pointers` | **crashes at runtime**: e0 dangles after the vector resizes |
| `src/2_with_references.rs` | `cargo run --bin with_references` | **does not compile** (on purpose): the borrow checker rejects it. Explore with Aquascope's `interpret` |

## `2_borrowing_rules`

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_inactive_reference.rs` | `cargo run --bin inactive_reference` | runs fine: the reference is no longer active when the vector is mutated |
| `src/2_many_const_references.rs` | `cargo run --bin many_const_references` | runs fine: many const references are allowed; try making one mut |
| `src/3_drop_while_borrowed.rs` | `cargo run --bin drop_while_borrowed` | **does not compile** (on purpose): cannot drop data while it is borrowed |

## `3_permissions`

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_read_write_own.rs` | `cargo run --bin read_write_own` | runs fine: R/W/O permissions of plain variables |
| `src/2_missing_write.rs` | `cargo run --bin missing_write` | **does not compile** (on purpose): x = x + 1 requires W permissions |
| `src/3_const_borrow.rs` | `cargo run --bin const_borrow` | runs fine: x loses O while borrowed |
| `src/4_mut_borrow.rs` | `cargo run --bin mut_borrow` | runs fine: x loses R, W, and O while mutably borrowed |
| `src/5_borrow_of_mut_data.rs` | `cargo run --bin borrow_of_mut_data` | runs fine: const reference to mut data -- exercise with Aquascope |
