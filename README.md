Project 1: Zero-Copy CLI Tooling using POSIX / libc

Goal: Learn Rust error handling (Result/?), ownership transfer, and raw OS syscall bindings.

What to build: Reimplement a high-performance grep or log analyzer in Rust.

Key constraints:

    Do not read files into heap strings line-by-line.

    Use the nix crate or raw libc bindings to call mmap() on input files.

    Work directly with raw byte slices (&[u8]) and zero-copy string scanning.

Rust concepts unlocked: Slice references, lifetime annotations ('a), memory mapping safely across struct boundaries, std::io::Write buffering.

Steps:
