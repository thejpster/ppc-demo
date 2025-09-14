# Rust Demo for the QEMU virtual PPCE500 Machine

This example shows how to compile Rust for PowerPC using a JSON target.

This target has no FPU support, because we're targeting the Freescale E500v1
CPU and that does not have an FPU.

The Freescale E500v1 does have a "Signal Processing Engine" (SPE) for SIMD and
a "Single Precision Embedded Scalar Floating Point" (SPESFP) for scalar
single-precision FP, but I have not been able to get either to work.

```console
$ export CC_powerpc_unknown_none_eabi=clang
$ export CFLAGS_powerpc_unknown_none_eabi="-mcpu=e500 -msoft-float"
$ cargo +nightly run --target=powerpc-unknown-none-eabi.json -Zbuild-std=core
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.74s
     Running `qemu-system-ppc -machine ppce500 -cpu e500v1 -d guest_errors,unimp -nographic -bios target/powerpc-unknown-none-eabi/debug/ppc_demo`
Hello, this is Rust on the PPCE500 machine
001 002 003 004 005 006 007 008 009 010 
002 004 006 008 010 012 014 016 018 020 
003 006 009 012 015 018 021 024 027 030 
004 008 012 016 020 024 028 032 036 040 
005 010 015 020 025 030 035 040 045 050 
006 012 018 024 030 036 042 048 054 060 
007 014 021 028 035 042 049 056 063 070 
008 016 024 032 040 048 056 064 072 080 
009 018 027 036 045 054 063 072 081 090 
010 020 030 040 050 060 070 080 090 100 
Adding 1.0f64 and 2.0f64 to get 3
Adding 1.0f32 and 2.0f32 to get 3
Adding 1 and 2 to get 3
Have built structure data_t { x: 5a, y: 3.141592653589793, z: 1337c0de }
C code said y=3.141592653589793
C code said z=0x1337c0de
All done!
```

Press `Ctrl+A, X` to quit QEMU.

## Tools

You will need a C compiler. I have used [crosstool-ng](https://github.com/crosstool-ng/crosstool-ng) to build one with [this config](./crosstool-ng/.config).

```bash
cd crosstool-ng
ct-ng build
export PATH=$PATH:~/x-tools/powerpc-unknown-eabi/bin
export CC_powerpc_unknown_none_eabi=powerpc-unknown-eabi-gcc
export CFLAGS_powerpc_unknown_none_eabi="-mcpu=8540 -msoft-float"
  ```

I have also used `clang`.

```bash
export CC_powerpc_unknown_none_eabi=clang
export CFLAGS_powerpc_unknown_none_eabi="-mcpu=e500 -msoft-float"
```

## Licence

This code is licensed under the GPL v3 or Later.

Copyright (c) Jonathan 'theJPster' Pallant, 2025
