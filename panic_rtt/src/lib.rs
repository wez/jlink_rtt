//! Set the panicking behavior to log to a JLINK debugger and loop.
//!
//! This crate contains a panic handler that emits the reason to an
//! in-memory ring buffer that an attached JLINK device can print,
//! and then loops forever.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_rtt;
//!
//! fn main() {
//!     panic!("message is logged to debugger");
//! }
//! ```
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;
extern crate jlink_rtt;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m::interrupt;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    let mut out = jlink_rtt::Output::new();
    writeln!(out, "{}", info).ok();

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
