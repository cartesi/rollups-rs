# Cartesi Rust libraries

Rust libraries for building and running [Cartesi] applications.

## Crates

| Crate | Description |
| --- | --- |
| [`cartesi-rollup`] | Safe API for applications running inside a Cartesi Machine: the input loop, vouchers, notices, reports and GIO |
| [`cartesi-libcmt-sys`] | Raw bindings to libcmt, the guest rollup library (bundled, built from source) |
| [`cartesi-codec`] | Encoding and decoding of Cartesi Rollups inputs, outputs and portal deposits |
| [`cartesi-machine`] | Safe bindings to the Cartesi Machine emulator |
| [`cartesi-machine-sys`] | Raw bindings to `libcartesi` (links an installed emulator) |

[`cartesi-rollup`]: crates/cartesi-rollup
[`cartesi-libcmt-sys`]: crates/cartesi-libcmt-sys
[`cartesi-codec`]: crates/cartesi-codec
[`cartesi-machine`]: crates/cartesi-machine
[`cartesi-machine-sys`]: crates/cartesi-machine-sys

## Compatibility

| Component | Version |
| --- | --- |
| Cartesi Machine emulator | v0.21 |
| libcmt (machine-guest-tools) | v0.18.0 |
| Minimum supported Rust version | 1.85 |

## Versioning

Each crate is versioned independently, following [Semantic Versioning]. While a
crate is `0.x`, a minor bump may contain breaking changes. The machine crates'
major and minor version track the emulator release they bind: `0.21.x` binds
Cartesi Machine 0.21.

Every release has a `CHANGELOG.md` entry in the crate's directory and a git
tag named `<crate>-v<version>`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Every user-facing change comes with a
changeset, which becomes its changelog entry.

## License

Licensed under [Apache 2.0](LICENSE).

[Cartesi]: https://cartesi.io
[Semantic Versioning]: https://semver.org
