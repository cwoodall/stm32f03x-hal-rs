//! What happens when `panic!` is invoked?
//!
//! Find out with this app

#![no_std]

extern crate f0;

use f0::exceptions::{self, Exceptions};
use f0::interrupts::{self, Interrupts};

fn main() {
    panic!()
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
