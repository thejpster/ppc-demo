//! Start-up code for the PPCE500 machine emulated by QEMU
//!
//! This code does not initialise .data and .bss because it assumes you are
//! executing from RAM and that when the machine code (.text) was loaded into
//! RAM, the .data, .rodata and .bss sections were also loaded.
//!
//! Requires nightly rust, and my new `powerpc-unknown-none-eabi` target.
//!
//! Copyright (c) Jonathan 'theJPster' Pallant
//!
//! Licensed under the GPL Version 3 or later


#![no_std]
#![feature(asm_experimental_arch)]

use arbitrary_int::{u2, u4, u20};

// Copyright (c) 1995 Cygnus Support
//
// The authors hereby grant permission to use, copy, modify, distribute,
// and license this software and its documentation for any purpose, provided
// that existing copyright notices are retained in all copies and that this
// notice is included verbatim in any distributions. No written agreement,
// license, or royalty fee is required for any of the authorized uses.
// Modifications to this software may be copyrighted by their authors
// and need not follow the licensing terms described here, provided that
// the new terms are clearly indicated on the first page of each file where
// they apply.
core::arch::global_asm!(
    r#"
    .section .text.startup
.Lptr:
    .globl   _Reset
    .type    _Reset,@function
_Reset:
    bl       .Laddr                   // get current address
.Laddr:
    mflr     %r4                      // real address of .Laddr
    lwz      %r0, (.Lstk-.Laddr)(%r4) // absolute stack address location
    mr       %r1, %r0                 // use user defined stack

    addi     %r1, %r1, -4             // make sure we don't overwrite debug mem
    lis      %r0, 0
    stw      %r0, 0(%r1)              // clear back chain
    stwu     %r1, -64(%r1)            // push another stack frame

    bl       kmain                    // jump to rust
    trap
    
.Lstk:
    .long stack_top
"#
);

/// Describes an MMU mapping
struct Mapping {
    use_tlb1: bool,
    virtual_addr: u32,
    phys_addr: u64,
    ux: bool,
    sx: bool,
    uw: bool,
    sw: bool,
    ur: bool,
    sr: bool,
    write_through: bool,
    inhibit_cache: bool,
    memory_coherence: bool,
    guarded: bool,
    little_endian: bool,
    translation_space: bool,
    esel: u4,
    tsize: TSize,
    iprot: bool,
}

impl Mapping {
    pub fn set_tlb(&self) {
        let mas0 = Mas0::default()
            .with_tlbsel(self.use_tlb1)
            .with_esel(self.esel)
            .with_nv(u2::new(0))
            .raw_value();
        let mas1 = Mas1::default()
            .with_v(true)
            .with_iprot(self.iprot)
            .with_tid(0)
            .with_ts(self.translation_space)
            .with_tsize(self.tsize)
            .raw_value();
        let mas2 = Mas2::default()
            .with_epn(u20::new(self.virtual_addr >> 12))
            .with_w(self.write_through)
            .with_i(self.inhibit_cache)
            .with_m(self.memory_coherence)
            .with_g(self.guarded)
            .with_e(self.little_endian)
            .raw_value();
        let mas3 = Mas3::default()
            .with_rpn(u20::new((self.phys_addr as u32) >> 12))
            .with_ux(self.ux)
            .with_sx(self.sx)
            .with_ur(self.ur)
            .with_sr(self.sr)
            .with_uw(self.uw)
            .with_sw(self.sw)
            .raw_value();
        let mas7 = (self.phys_addr >> 32) as u32;
        unsafe {
            write_tlb(mas0, mas1, mas2, mas3, mas7);
        }
    }
}

/// MMU Assist Register 0
#[bitbybit::bitfield(u32, default = 0)]
struct Mas0 {
    #[bit(28, rw)]
    tlbsel: bool,
    #[bits(16..=19, rw)]
    esel: u4,
    #[bits(0..=1, rw)]
    nv: u2,
}

/// MMU Assist Register 1
#[bitbybit::bitfield(u32, default = 0)]
struct Mas1 {
    #[bit(31, rw)]
    v: bool,
    #[bit(30, rw)]
    iprot: bool,
    #[bits(16..=23, rw)]
    tid: u8,
    #[bit(12, rw)]
    ts: bool,
    #[bits(8..=11, rw)]
    tsize: Option<TSize>,
}

/// Translation region size
#[bitbybit::bitenum(u4)]
enum TSize {
    _4K = 0b0001,
    _16K = 0b0010,
    _64K = 0b0011,
    _256K = 0b0100,
    _1M = 0b0101,
    _4M = 0b0110,
    _16M = 0b0111,
    _64M = 0b1000,
    _256M = 0b1001,
    _1G = 0b1010,
    _4G = 0b1011,
}

/// MMU Assist Register 2
#[bitbybit::bitfield(u32, default = 0)]
struct Mas2 {
    #[bits(12..=31, rw)]
    epn: u20,
    #[bit(4, rw)]
    w: bool,
    #[bit(3, rw)]
    i: bool,
    #[bit(2, rw)]
    m: bool,
    #[bit(1, rw)]
    g: bool,
    #[bit(0, rw)]
    e: bool,
}

/// MMU Assist Register 3
#[bitbybit::bitfield(u32, default = 0)]
struct Mas3 {
    #[bits(12..=31, rw)]
    rpn: u20,
    #[bits(6..=9, rw)]
    user: u4,
    #[bit(5, rw)]
    ux: bool,
    #[bit(4, rw)]
    sx: bool,
    #[bit(3, rw)]
    uw: bool,
    #[bit(2, rw)]
    sw: bool,
    #[bit(1, rw)]
    ur: bool,
    #[bit(0, rw)]
    sr: bool,
}

/// Write to the Translation Lookaside Buffer
unsafe fn write_tlb(mas0: u32, mas1: u32, mas2: u32, mas3: u32, mas7: u32) {
    unsafe {
        core::arch::asm!(r#"
            .align 4
            mtspr 624, {0} // MMU Assist Register 0
            mtspr 625, {1} // MMU Assist Register 1
            mtspr 626, {2} // MMU Assist Register 2
            mtspr 627, {3} // MMU Assist Register 3
            mtspr 944, {4} // MMU Assist Register 7
            isync
            tlbwe
            msync
            isync
        "#,
        in(reg) mas0,
        in(reg) mas1,
        in(reg) mas2,
        in(reg) mas3,
        in(reg) mas7);
    }
}

/// Initialise the MMU
///
/// Maps in a 1 MiB region containing UART0, so we can use it
pub fn init_mmu() {
    let mapping = Mapping {
        use_tlb1: true,
        virtual_addr: 0xe000_0000,
        phys_addr: 0xf_e000_0000,
        write_through: false,
        inhibit_cache: true,
        memory_coherence: false,
        guarded: true,
        little_endian: false,
        translation_space: false,
        ux: false,
        sx: false,
        uw: false,
        sw: true,
        ur: false,
        sr: true,
        esel: u4::new(1),
        tsize: TSize::_1M,
        iprot: false,
    };
    mapping.set_tlb();
}

pub struct Uart {
    base_addr: *mut u32
}

impl Uart {
    /// Create a new handle for UART0
    pub unsafe fn new_uart0() -> Uart {
        Uart {
            base_addr: 0xe000_4500 as *mut u32
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                self.base_addr.write_volatile(b as u32);
            }
        }
        Ok(())
    }
}


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
