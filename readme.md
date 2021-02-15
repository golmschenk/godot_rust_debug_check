# Initializing Rust GDNative and debugging

Initialing Rust GDNative seems to break debug symbols in some way when called from C code.

## Steps to reproduce

1. Download this repository.
    * `git clone https://github.com/golmschenk/godot_rust_debug_check.git`
    * `cd godot_rust_debug_check`
2. Build the Rust library.
    * `cargo build`
3. Compile the C code, linking the Rust library.
    * `gcc -o call_rust_from_c call_rust_from_c.c -Ltarget/debug -lgodot_rust_debug_check`
4. In IDE (tested with CLion), add breakpoint in the `count_from_rust` function in `src/lib.rs`.
5. Run C executable.
    * `./call_rust_from_c`
6. Attach to process from IDE. Does not halt at breakpoint.
7. Comment out `godot_gdnative_init!();` line in `src/lib.rs`.
8. Rebuild Rust library.
    * `cargo build`
9. Run C executable.
    * `./call_rust_from_c`
10. Attach to process from IDE. Does halt at breakpoint.

## Overview of code
There is Rust library which contains two simple functions. One is a print, sleep, and loop function. The other calls the Rust GDNative initialization. There is a C header allowing use of the print loop function from Rust. And then a C file calling this function.

## Other notes

* The same issue occurs when calling the library from Godot.
* In both cases, the IDE is properly attached to the process, and will catch interrupt signals. However, the one case allow for breakpoints in the source code while the other does not. This suggests there's something happening with the debug symbols.
* Running the same code from a Rust executable does not have this problem.

## Software

* CLion = 2020.3.2
* MacOS = 11.2
