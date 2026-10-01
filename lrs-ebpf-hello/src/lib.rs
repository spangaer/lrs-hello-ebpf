#![no_std]
#![no_main]

use aya_ebpf::{macros::lsm, programs::LsmContext};

#[lsm(hook = "inode_create")]
pub fn lrs_ebpf_hello(ctx: LsmContext) -> i32 {
    let previous_retval: i32 = ctx.arg(3);
    if previous_retval != 0 {
        return previous_retval;
    }

    0
}

// not used by the eBPF program itself, but required by the Rust toolchain/linker
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[used]
#[unsafe(link_section = "license")]
static LICENSE: [u8; 26] = *b"GPL and additional rights\0";
