# cartesi-machine

Safe Rust bindings to the [Cartesi Machine] emulator: create, load and store
machines, run them, exchange cmio requests and responses, and produce the
state hashes and proofs used for verification.

```rust,no_run
use std::path::Path;

use cartesi_machine::{Machine, config::runtime::RuntimeConfig, constants::break_reason};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine = Machine::load(Path::new("machine-image"), &RuntimeConfig::default())?;
    if machine.run(u64::MAX)? == break_reason::HALTED {
        println!("halted at mcycle {}", machine.mcycle()?);
    }
    println!("root hash: {}", hex::encode(machine.root_hash()?));
    Ok(())
}
```

The crate links `libcartesi` statically through [`cartesi-machine-sys`]. See
that crate for how the library is found.

## Versioning

The major and minor version track the emulator release the bindings target:
`0.21.x` binds Cartesi Machine 0.21. `EXPECTED_EMULATOR_VERSION` holds the
exact version, and the build refuses headers from any other release line.

## Features

- `remote_machine`: link `libcartesi_jsonrpc.a`, the JSON-RPC client for
  remote machines, instead of the local emulator.

## Tests

The tests boot a pinned kernel and rootfs. Prepare them once with the
`cartesi-machine` CLI installed:

```sh
scripts/prepare-test-images.sh
cargo test -p cartesi-machine
```

Set `CARTESI_MACHINE_TEST_IMAGES` to use images from another directory.

[Cartesi Machine]: https://github.com/cartesi/machine-emulator
[`cartesi-machine-sys`]: https://crates.io/crates/cartesi-machine-sys
