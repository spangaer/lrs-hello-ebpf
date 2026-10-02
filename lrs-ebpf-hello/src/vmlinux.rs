#![allow(non_camel_case_types)]

pub type __u64 = ::aya_ebpf::cty::c_ulonglong;
pub type u64_ = __u64;
pub type __u32 = ::aya_ebpf::cty::c_uint;
pub type u32_ = __u32;
pub type __kernel_pid_t = ::aya_ebpf::cty::c_int;
pub type pid_t = __kernel_pid_t;

// A custom structure that matches the names for CO-RE relocation
#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub comm: [::aya_ebpf::cty::c_char; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qstr {
    pub __bindgen_anon_1: qstr__bindgen_ty_1,
    pub name: *const ::aya_ebpf::cty::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union qstr__bindgen_ty_1 {
    pub __bindgen_anon_1: qstr__bindgen_ty_1__bindgen_ty_1,
    pub hash_len: u64_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qstr__bindgen_ty_1__bindgen_ty_1 {
    pub hash: u32_,
    pub len: u32_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dentry {
    pub d_name: qstr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode {
    pub i_ino: ::aya_ebpf::cty::c_ulong,
}
