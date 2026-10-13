# cartesi-rollup

Safe Rust API for Cartesi Rollups applications running inside a Cartesi
Machine. It wraps [libcmt] (through [`cartesi-libcmt-sys`]) to provide the
input loop, vouchers, notices, reports and GIO requests.

```rust,no_run
use cartesi_rollup::{Rollup, RollupRequest, cmt::RollupCmt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rollup = RollupCmt::try_new()?;
    loop {
        match rollup.next_input()? {
            RollupRequest::Advance { metadata, payload } => {
                println!("input {} from {}", metadata.index, metadata.msg_sender);
                rollup.emit_notice(&cartesi_rollup::Notice { payload: payload.into() })?;
            }
            RollupRequest::Inspect { payload } => {
                rollup.emit_report(&payload)?;
            }
        }
    }
}
```

The [`Rollup`] trait abstracts the machine, so application logic can be unit
tested on the host against a mock implementation. `RollupCmt` is the libcmt
implementation. On the host it links libcmt's mock I/O backend.

Output types (`Voucher`, `Notice`) come from [`cartesi-codec`] and are
re-exported here.

## Building for the machine

Build for a `riscv64` Linux target with a matching cross C toolchain. See
[`cartesi-libcmt-sys`] for how libcmt is built and how to select the
toolchain.

[libcmt]: https://github.com/cartesi/machine-guest-tools/tree/main/sys-utils/libcmt
[`cartesi-libcmt-sys`]: https://crates.io/crates/cartesi-libcmt-sys
[`cartesi-codec`]: https://crates.io/crates/cartesi-codec
[`Rollup`]: https://docs.rs/cartesi-rollup/latest/cartesi_rollup/trait.Rollup.html
