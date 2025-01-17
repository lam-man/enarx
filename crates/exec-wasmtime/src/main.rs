// SPDX-License-Identifier: Apache-2.0
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]

use enarx_exec_wasmtime::execute;

/// Set FSBASE
///
/// Overwrite the only location in musl, which uses the `arch_prctl` syscall
#[no_mangle]
pub extern "C" fn __set_thread_area(p: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut rax: usize = 0;
    if unsafe { core::arch::x86_64::__cpuid(7).ebx } & 1 == 1 {
        unsafe {
            std::arch::asm!("wrfsbase {}", in(reg) p);
        }
    } else {
        const ARCH_SET_FS: core::ffi::c_int = 0x1002;
        unsafe {
            std::arch::asm!(
            "syscall",
            inlateout("rax")  libc::SYS_arch_prctl => rax,
            in("rdi") ARCH_SET_FS,
            in("rsi") p,
            lateout("rcx") _, // clobbered
            lateout("r11") _, // clobbered
            );
        }
    }
    rax as _
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env().init();

    execute()
}
