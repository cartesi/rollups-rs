# cartesi-machine-sys

Raw FFI bindings to `libcartesi`, the C API of the [Cartesi Machine] emulator.
Most users want the safe [`cartesi-machine`] crate instead.

## Linking

The crate links a prebuilt emulator statically. It does not build the
emulator. Install a release, for example the `.deb` from the
[machine-emulator releases] or `brew install cartesi/tap/cartesi-machine`.

The build script finds the library as follows:

1. If `LIBCARTESI_PATH` is set, it must be an absolute directory containing
   `libcartesi.a`. The headers are taken from the sibling
   `include/cartesi-machine` directory.
2. Otherwise it searches `/opt/homebrew`, `/usr/local`, `/opt/local` and `/usr`
   for `lib/libcartesi.a` and `include/cartesi-machine/cm.h`.

`INCLUDECARTESI_PATH` overrides the header directory (absolute path) in
either case. With the `remote_machine` feature, `libcartesi_jsonrpc.a` is
required and linked instead.

The bindings are generated with [bindgen] from the installed `cm.h`, so
`libclang` must be available. The build fails if `cm-version.h` reports a
release other than 0.21. The emulator also needs OpenMP: `libgomp` on Linux,
`libomp` on macOS. `libslirp` is needed when the archive was built with
slirp.

[Cartesi Machine]: https://github.com/cartesi/machine-emulator
[machine-emulator releases]: https://github.com/cartesi/machine-emulator/releases
[`cartesi-machine`]: https://crates.io/crates/cartesi-machine
[bindgen]: https://github.com/rust-lang/rust-bindgen
