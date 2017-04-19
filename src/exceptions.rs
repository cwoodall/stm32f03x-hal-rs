//! Exceptions
#![allow(unused_imports)]
use cortex_m::asm;
pub use cortex_m::exception::DEFAULT_HANDLERS;
pub use cortex_m::exception::Handlers as Exceptions;
use cortex_m::exception::{StackedRegisters, Exception};

/// The default exception handler
///
/// This handler triggers a breakpoint (`bkpt`) and gives you access, within a
/// GDB session, to the stack frame (`_sf`) where the exception occurred.
// This needs asm!, #[naked] and unreachable() to avoid modifying the stack
// pointer (MSP), that way it points to the previous stack frame
#[naked]
pub unsafe extern "C" fn default_handler() {
    // This is the actual exception handler. `_sf` is a pointer to the previous
    // stack frame
    extern "C" fn handler(_sf: &StackedRegisters) -> ! {
        hprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), _sf.pc);

        asm::bkpt();
        loop {}
    }

    // Do not modify this `asm!` block! This is a "trampoline" to get you to the
    // real exception handler.
    asm!("mrs r0, MSP
          ldr r1, [r0, #20]
          b $0"
         :
         : "i"(handler as extern "C" fn(&StackedRegisters) -> !) :: "volatile");

    ::core::intrinsics::unreachable()
}

/// The reset handler
///
/// This is the entry point of all programs
#[doc(hidden)]
#[export_name = "start"]
pub unsafe extern "C" fn reset_handler() -> ! {
    extern "C" {
        static mut _ebss: u32;
        static mut _sbss: u32;

        static mut _edata: u32;
        static mut _sdata: u32;

        static _sidata: u32;
    }

    ::r0::zero_bss(&mut _sbss, &mut _ebss);
    ::r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    // NOTE `rustc` forces this signature on us. See `src/rt.rs`
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    // Neither `argc` or `argv` make sense in bare metal contexts so we just
    // stub them
    main(0, ::core::ptr::null());

    // If `main` returns, then we go into "reactive" mode and attend interrupts
    // as they occur.
    loop {
        asm::wfi()
    }
}

/// A reserved spot in the vector table
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Reserved {
    /// Reserved
    Vector = 0,
}
