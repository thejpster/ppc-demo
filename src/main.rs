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

    let z = unsafe { ppc_demo::c_library::clib_add_two_double(1.0, 2.0) };
    writeln!(console, "Adding 1.0f64 and 2.0f64 to get {}", z).unwrap();

    let z = unsafe { ppc_demo::c_library::clib_add_two_float(1.0, 2.0) };
    writeln!(console, "Adding 1.0f32 and 2.0f32 to get {}", z).unwrap();

    let z = unsafe { ppc_demo::c_library::clib_add_two_ulong(1, 2) };
    writeln!(console, "Adding 1 and 2 to get {}", z).unwrap();

    let mut sample_struct = ppc_demo::c_library::data_t {
        x: 0x5Au8,
        y: core::f64::consts::PI,
        z: 0x1337C0DE,
    };

    writeln!(console, "Have built structure {:x?}", sample_struct).unwrap();

    let y = unsafe { ppc_demo::c_library::get_double_field(&raw mut sample_struct) };
    writeln!(console, "C code said y={}", y).unwrap();

    let z = unsafe { ppc_demo::c_library::get_int_field(&raw mut sample_struct) };
    writeln!(console, "C code said z=0x{:08x}", z).unwrap();

    writeln!(console, "All done!").unwrap();
}
