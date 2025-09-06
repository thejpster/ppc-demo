//! Very basic demo for the PPCE500 machine emulated by QEMU
//!
//! Requires nightly rust, and my new `powerpc-unknown-none-eabi` target.
//!
//! Copyright (c) Jonathan 'theJPster' Pallant
//!
//! Licensed under the GPL Version 3 or later

#![no_std]
#![no_main]

use core::fmt::Write;

use ppc_demo as _;

#[unsafe(no_mangle)]
extern "C" fn kmain() {
    ppc_demo::init_mmu();
    main();
    loop {}
}

fn main() {
    let mut console = unsafe { ppc_demo::Uart::new_uart0() };
    writeln!(console, "Hello, this is Rust on the PPCE500 machine").unwrap();
    for x in 1..=10 {
        for y in 1..=10 {
            write!(console, "{:03} ", f64::from(x) * f64::from(y)).unwrap();
        }
        writeln!(console).unwrap();
    }

    writeln!(console, "All done!").unwrap();
}
