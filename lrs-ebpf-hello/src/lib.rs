#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)] // FIXME

mod vmlinux;

use aya_ebpf::{
    helpers::{bpf_get_current_task, bpf_probe_read_kernel, bpf_probe_read_kernel_buf},
    macros::lsm,
    programs::LsmContext,
};
use aya_log_ebpf::info;
use vmlinux::{dentry, inode, task_struct};

#[lsm(hook = "inode_create")]
pub fn inode_create(ctx: LsmContext) -> i32 {
    match unsafe { try_inode_create(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret as i32,
    }
}

unsafe fn try_inode_create(ctx: LsmContext) -> Result<i32, i64> {
    // 1. Unpack the arguments specific to the `inode_create` hook.
    // Kernel hook signature:
    // int security_inode_create(struct inode *dir, struct dentry *dentry, umode_t mode)
    // LsmContext acts like an array of raw u64 arguments in order:
    let dir_ptr: *const inode = ctx.arg(0);
    let dentry_ptr: *const dentry = ctx.arg(1);

    let previous_retval: i32 = ctx.arg(3);
    if previous_retval != 0 {
        return Ok(previous_retval); // FIXME later log rejections anyway?
    }

    // 2. Fetch the actor details (who is trying to create the file?)
    let task_ptr: *const task_struct = bpf_get_current_task() as *const task_struct;

    let pid: i32 = bpf_probe_read_kernel(core::ptr::addr_of!((*task_ptr).pid))?;
    let comm: [::aya_ebpf::cty::c_char; 16usize] =
        bpf_probe_read_kernel(core::ptr::addr_of!((*task_ptr).comm))?;
    let comm_bytes = comm.map(|character| character as u8);

    // 3. Read some data from the target file's dentry portably (e.g., file name string)
    // struct dentry contains a `d_name` field of type struct qstr, which holds the name pointer
    let d_name = bpf_probe_read_kernel(core::ptr::addr_of!((*dentry_ptr).d_name))?;
    let name_len = d_name.__bindgen_anon_1.__bindgen_anon_1.len as usize;
    let parent_ino = bpf_probe_read_kernel(core::ptr::addr_of!((*dir_ptr).i_ino))?;

    // Read the actual string data buffer (safely capped to avoid stack overflow)
    let mut filename: [u8; 32] = [0; 32];
    let filename_len = core::cmp::min(name_len, filename.len());
    if !d_name.name.is_null() && filename_len != 0 {
        bpf_probe_read_kernel_buf(d_name.name, &mut filename[..filename_len])?;
    }

    if let Ok(comm_str) = core::str::from_utf8(&comm_bytes) {
        if let Ok(file_str) = core::str::from_utf8(&filename[..filename_len]) {
            info!(
                &ctx,
                "PID {} ({}) is creating file: {} inside parent inode: {}",
                pid,
                comm_str.trim_matches('\0'),
                file_str,
                parent_ino
            );
        }
    }

    // 4. Return 0 to allow creation, preserving any prior LSM denial above
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
