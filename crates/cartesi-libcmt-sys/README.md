# cartesi-libcmt-sys

Raw FFI bindings to [libcmt], the C library that Cartesi Machine guest
programs use to talk to the rollup: read advance and inspect requests, emit
vouchers, notices and reports, and issue GIO requests.

Most applications should use the safe [`cartesi-rollup`] crate instead.

## Bundled libcmt

The libcmt sources are vendored in `libcmt/` and built by `build.rs`, so no
system installation is needed. They are the `sys-utils/libcmt` `Makefile`,
`include/` and `src/` from [machine-guest-tools] **v0.18.0**, unmodified, with
that project's `LICENSE` (Apache-2.0) and `AUTHORS`. This libcmt version pairs
with Cartesi Machine v0.21.

They come from the tag tarball
`https://github.com/cartesi/machine-guest-tools/archive/refs/tags/v0.18.0.tar.gz`
(commit `5222250c69371f7cbe96e6082699a8ca22b5969c`, sha256
`a16ee31a7abb0522ae70497721bf4856665297e1dd26d875a43fadbe48814bb6`).

## Building

- **`riscv64` targets** (the guest) build the real library with
  `riscv64-unknown-linux-musl-gcc` and `riscv64-unknown-linux-musl-ar`. Set
  `LIBCMT_TOOLCHAIN_PREFIX` to use a different cross toolchain, for example
  `LIBCMT_TOOLCHAIN_PREFIX=riscv64-linux-gnu-`.
- **Other targets** (the host) build libcmt's mock I/O backend, so crates that
  depend on these bindings still compile, lint and run unit tests outside the
  machine.

Both need `make` and a C compiler. [bindgen] generates the bindings at build
time, so `libclang` must be installed as well.

## Updating libcmt

1. Download the new machine-guest-tools tag tarball and verify its checksum.
2. Replace `libcmt/{Makefile,include,src,LICENSE,AUTHORS}` with the new tag's
   `sys-utils/libcmt` files and top-level `LICENSE` and `AUTHORS`.
3. Update the version in this README and in `build.rs`.
4. Add a changeset describing the upgrade.

[libcmt]: https://github.com/cartesi/machine-guest-tools/tree/main/sys-utils/libcmt
[machine-guest-tools]: https://github.com/cartesi/machine-guest-tools
[bindgen]: https://github.com/rust-lang/rust-bindgen
[`cartesi-rollup`]: https://crates.io/crates/cartesi-rollup
