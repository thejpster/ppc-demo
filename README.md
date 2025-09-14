# Rust Demo for the QEMU virtual PPCE500 Machine

This example shows how to compile Rust using the `powerpc-unknown-none-eabi` target that I just wrote.

It runs in QEMU.

```console
$ export CROSS_COMPILE=powerpc-unknown-eabi-
$ cargo run --release
  Finished `release` profile [optimized] target(s) in 0.01s
     Running `qemu-system-ppc -cpu e500 -machine ppce500 -d guest_errors,unimp -nographic -bios target/powerpc-unknown-none-eabi/release/ppc_demo`
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

Press `Ctrl+A, X` to quit.

This target does not support the PowerPC Signal Processing Engine (SPE) because support was dropped after GCC 8 due to it being unmaintained. Nor does it support the PowerPC Variable Length Encoding (VLE) extension because LLVM does not support VLE.

## Tools

You will need a C compiler. I use [crosstool-ng](https://github.com/crosstool-ng/crosstool-ng) to build one with [this config](./crosstool-ng/.config).

```bash
cd crosstool-ng
ct-ng build
export PATH=$PATH:~/x-tools/powerpc-unknown-eabi/bin
```

You then need the `powerpc-unknown-none-eabi` target from https://github.com/thejpster/rust/tree/add-powerpc-bare-metal

```bash
git clone https://github.com/thejpster/rust
cd rust
git checkout add-powerpc-bare-metal
cat > bootstrap.toml << EOF
change-id = 145976
profile = 'dist'

[llvm]
download-ci-llvm = true

[gcc]

[build]
configure-args = []

[install]

[rust]

[dist]

[target.x86_64-unknown-linux-gnu]

[target.powerpc-unknown-none-eabi]
cc = "powerpc-unknown-eabi-gcc"
EOF
BOOTSTRAP_SKIP_TARGET_SANITY=1 ./x build --stage 1 compiler library/std proc_macro \
  --target=x86_64-unknown-linux-gnu,powerpc-unknown-none-eabi
rustup toolchain link stage1 $(pwd)/build/x86_64-unknown-linux-gnu/stage1
```

## Licence

This code is licensed under the GPL v3 or Later.

Copyright (c) Jonathan 'theJPster' Pallant, 2025
