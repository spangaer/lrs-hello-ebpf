# Learning Rust Hello eBPF (_lrs_)

Goal of this (learning) project is to successfully run a hello world eBPF Rust program.

## TODO

- split the project into separate user-space and eBPF crates:
  - user-space crate: CC0 as a stand-in for a potentially proprietary license
  - eBPF crate: own LICENSE file and GPL-compatible ELF `license` string (likely MPL)
  - keep the kernel-facing eBPF claim separate from the user-space license
  - for now, `GPL and additional rights` is a temporary stand-in while the split is
    in progress

## Failed attempts

### Rust CO-RE field access

The experiment uses a Rust BPF LSM `inode_create` program to log the current
process and the attempted filename. Kernel bindings were generated with:

```sh
aya-tool generate task_struct inode dentry > lrs-ebpf-hello/src/vmlinux2.rs
```

That generated file contained fields named `gen`, which the project's Rust
2024 edition rejects. A small hand-written `vmlinux.rs` was used to continue
the experiment; it is not a substitute for CO-RE metadata.

The `build-ebpf` alias now passes `-C debuginfo=2` and `--btf` to `bpf-linker`.
The resulting ELF had `.BTF` and `.BTF.ext`, but its BTF extension header had
`core_relo_len = 0`; inspection also found no local BTF definitions for the
kernel structs used by the program. Aya's loader can apply CO-RE records, but
this Rust build did not emit any field-access records for it to apply.

An `--emit=llvm-ir` linker experiment wrote LLVM IR instead of an ELF object,
so that option was removed from the build alias. The remaining blocker is Rust
frontend/codegen support for BTF-aware field accesses. The Rust support was
experimental and still under review during this attempt; [the bpf-linker
experiment](https://github.com/aya-rs/bpf-linker/pull/363) was closed in favor
of [the rustc work](https://github.com/rust-lang/rust/pull/161107).
