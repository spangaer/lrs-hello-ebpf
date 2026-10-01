#![no_std]
#![no_main]

use aya_ebpf::{bindings::xdp_action::XDP_PASS, macros::xdp, programs::XdpContext};

#[xdp]
pub fn lrs_ebpf_hello(_ctx: XdpContext) -> u32 {
    XDP_PASS as u32
}

// not used by the eBPF program itself, but required by the Rust toolchain/linker
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[used]
#[unsafe(link_section = "license")]
static LICENSE: [u8; 26] = *b"GPL and additional rights\0";
