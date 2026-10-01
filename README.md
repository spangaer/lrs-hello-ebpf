# Learning Rust Hello eBPF (_lrs_)

Goal of this (learning) project is to successfully run a hello world eBPF Rust program.

## TODO

- split the project into separate user-space and eBPF crates:
  - user-space crate: CC0 as a stand-in for a potentially proprietary license
  - eBPF crate: own LICENSE file and GPL-compatible ELF `license` string (likely MPL)
  - keep the kernel-facing eBPF claim separate from the user-space license
  - for now, `GPL and additional rights` is a temporary stand-in while the split is
    in progress
