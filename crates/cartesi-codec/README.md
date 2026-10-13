# cartesi-codec

Encoding and decoding of Cartesi Rollups inputs, outputs and portal deposits,
shared by guest applications and host tooling.

- **Inputs:** `Input` (the `EvmAdvance` call the application receives).
- **Outputs:** `Voucher`, `Notice` and `DelegateCallVoucherCall`.
- **Portals:** ABI encoders for Ether, ERC-20, ERC-721 and ERC-1155 deposits,
  and an `Erc20Deposit` decoder for ERC-20 portal payloads.

The types are generated with [`alloy-sol-types`], so they implement
`SolCall` for ABI encoding and decoding:

```rust
use cartesi_codec::{Notice, alloy_sol_types::SolCall};

let notice = Notice { payload: b"hello".to_vec().into() };
let encoded = notice.abi_encode();
assert_eq!(Notice::abi_decode(&encoded).unwrap(), notice);
```

`alloy-primitives` and `alloy-sol-types` are re-exported, so callers can use
the same versions.

[`alloy-sol-types`]: https://crates.io/crates/alloy-sol-types
