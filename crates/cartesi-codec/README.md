# cartesi-codec

Encoding and decoding of Cartesi Rollups inputs, outputs and portal deposits,
shared by guest applications and host tooling. The formats follow the Cartesi
Rollups contracts v3.0.0-alpha.10.

- **Inputs:** `Input`, the `EvmAdvance` call the application receives.
- **Outputs:** `Voucher`, `DelegateCallVoucher` and `Notice`.
- **Deposits:** `EtherDeposit`, `Erc20Deposit`, `Erc721Deposit`,
  `Erc1155SingleDeposit` and `Erc1155BatchDeposit`, the payloads the portals
  add to the input box.

Inputs and outputs are generated with [`alloy-sol-types`], so they implement
`SolCall` for ABI encoding and decoding:

```rust
use cartesi_codec::{Notice, alloy_sol_types::SolCall};

let notice = Notice { payload: b"hello".to_vec().into() };
let encoded = notice.abi_encode();
assert_eq!(Notice::abi_decode(&encoded).unwrap(), notice);
```

Deposits have `encode` and `decode` methods that produce and read the
portals' payloads byte for byte:

```rust
use cartesi_codec::{Erc20Deposit, alloy_primitives::{U256, address}};

let deposit = Erc20Deposit {
    token: address!("1111111111111111111111111111111111111111"),
    sender: address!("2222222222222222222222222222222222222222"),
    value: U256::from(5),
    exec_layer_data: Default::default(),
};
let payload = deposit.encode();
assert_eq!(payload.len(), 20 + 20 + 32);
assert_eq!(Erc20Deposit::decode(&payload).unwrap(), deposit);
```

`alloy-primitives` and `alloy-sol-types` are re-exported, so callers can use
the same versions.

[`alloy-sol-types`]: https://crates.io/crates/alloy-sol-types
