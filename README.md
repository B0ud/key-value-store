![Rust CI](https://github.com/B0ud/key-value-store/workflows/Rust/badge.svg)
[![Version](https://img.shields.io/badge/rustc-1.48+-ab6000.svg)](https://blog.rust-lang.org/2020/11/19/Rust-1.48.html)
[![codecov](https://codecov.io/gh/B0ud/key-value-store/branch/master/graph/badge.svg?token=NNQ52FZKAG)](https://codecov.io/gh/B0ud/key-value-store)

# An embedded key/velue store in Rust. 

Pincap Talent Plan for Practical Networked Applications (PNA) in Rust

##### Project 1 (in-memory key-value store)

- [x] Make the tests compile
- [x] Accept command line arguments
- [x] Cargo environment variables
- [x] Store values in memory
- [x] Documentation
- [x] Ensure good style with clippy and rustfmt

Optional : 
- [X] Switch from clap to structop

~~I kept clap as a library for the command line part but I have switch on clap derives a "copy" of the structop style~~ 

Finally, during the configuration of part 3, I migrated from clap to StructOp for a more suitable syntax

##### Part 2 (disk-backed key-value store with compacting log file)

- [x] Error handling
- [x] How the log behaves
- [x] Writing to the log
- [x] Reading from the log
- [x] Storing log pointers in the index
- [ ] Stateless vs. stateful KvStore
- [x] Compacting the log

##### Part 3 (networked disk-backed key-value store with multiple engines)

- [X] Command line parsing
- [X] Logging
- [X] Client-server networking setup
- [X] Implementing commands across the network
- [X] Pluggable storage engines 
- [ ] Benchmarking

Note : cargo run --bin 'kvs-server|kvs-client' -- [command]