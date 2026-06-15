# Module 12: Ownership

Each subdirectory is a standalone cargo project grouping related code samples from the
interactive lecture notes. Inside each project, the files in `src/` are numbered in the
order they appear in the lecture. Each file is its own binary: run it with
`cargo run --bin <name>` from inside the project directory (or use the run button
above each `main` in VSCode). Note that the binary name is the file name *without*
the number prefix (cargo does not allow binary names to start with a digit).

## `1_vec_of_strings`

The storyline of storing strings (on the heap) in a vector, in lecture order:

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_dangling_pointer.rs` | `cargo run --bin dangling_pointer` | **crashes at runtime**: pointer into the vector dangles after resize |
| `src/2_put_on_heap.rs` | `cargo run --bin put_on_heap` | runs fine: each string in its own heap allocation, vector stores pointers |
| `src/3_memory_leak.rs` | `cargo run --bin memory_leak` | runs fine: the Tracker (from project 3) shows the strings are never freed |
| `src/4_manual_cleanup.rs` | `cargo run --bin manual_cleanup` | runs fine: we destruct and free every string ourselves; the Tracker confirms it |
| `src/5_use_after_free.rs` | `cargo run --bin use_after_free` | **undefined behavior**: a string is destroyed while still in use |
| `src/6_boxes.rs` | `cargo run --bin boxes` | runs fine: `Box` gives stable addresses *and* automatic cleanup; the Tracker proves it |

## `2_ownership`

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_box_use_after_free.rs` | `cargo run --bin box_use_after_free` | **does not compile** (on purpose): compiler catches use of a dropped box |
| `src/2_move_clone_borrow.rs` | `cargo run --bin move_clone_borrow` | runs fine: one function per concept (move, clone, borrow), called in order from `main`; each has commented experiments to try |

## `3_references`

| File | Run with | Expected behavior |
|---|---|---|
| `src/1_references.rs` | `cargo run --bin references` | runs fine: references are addresses, but safe |
| `src/2_borrow_checker.rs` | `cargo run --bin borrow_checker` | **does not compile** (on purpose): teaser for the borrow checker |
