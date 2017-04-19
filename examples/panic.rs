//! What happens when `panic!` is invoked?
//!
//! Find out with this app

#![no_std]

extern crate stm32f03x_hal;

use stm32f03x_hal::exceptions::{self, Exceptions};
use stm32f03x_hal::interrupts::{self, Interrupts};

fn main() {
    panic!()
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
