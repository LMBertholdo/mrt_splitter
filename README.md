mrt_splitter
============

A fast and minimal MRT file splitter written in Rust.

Features:
- Supports both .mrt and .mrt.gz files
- Count MRT records with --count
- Split MRT into valid binary chunks by number of records

Usage:
  mrt_splitter <input.mrt[.gz]> [options]

Options:
  -n, --records <N>     Records per output file (default: 10000)
  -o, --output <DIR>    Output directory (default: output_chunks)
  -c, --count           Only count records
  --help                Show help message

Examples:
  Count records:
    mrt_splitter updates.mrt.gz --count

  Split file:
    mrt_splitter updates.mrt.gz -n 50000 -o parts/

Build:
  cargo build --release
  ./target/release/mrt_splitter --help

Or use the Makefile
make           # builds
make install   # builds + installs to ~/.cargo/bin
make clean     # cleans up

Do not forget to add ~/.cargo/bin to $PATH

