# Project

## Build

Build the eBPF object with:

```sh
cargo build-ebpf
```

The object is built for `bpfel-unknown-none` under the workspace's shared
`target/` directory. The devcontainer provides `bpf-linker`; the Rust toolchain
file selects nightly and installs `rust-src` for building `core` for BPF.

## Program

### Hook

The program uses the BPF LSM `inode_create` hook, which runs when a file is
created.

### Loading requirements

Loading it requires a kernel with `CONFIG_BPF_LSM` and BTF support, with BPF LSM
enabled at boot (for example, in the kernel's `lsm=` list). Building the object
does not load or attach it; that requires a userspace BPF loader.
