Implement a high-performance grep in Rust.

Key constraints:

    Do not read files into heap strings line-by-line.

    Use the nix crate or raw libc bindings to call mmap() on input files.

    Work directly with raw byte slices (&[u8]) and zero-copy string scanning.
