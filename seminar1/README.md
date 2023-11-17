# Seminar 1

## Compiling

Suggested command for compiling is: `RUST_MIN_STACK=10000000 cargo b -r`

This seminar is focused around sorting and search algorithms. 

## Sorting

For large input sizes on inneficient algorithms, speed up the algorithm by compiling with optimisations using the `--release` flag in the compile command. Eg. `cargo r --release` or `cargo r -r`.

All algorithms are implemented both recursively and iteratively.
**NOTE**: For recursive functions it is likely that the call stack space will run out will running it for large input sizes. If this happens increase the stack space by using the `RUST_MIN_STACK` environment variable. 
Call stack space: [https://doc.rust-lang.org/std/thread/#stack-size]
