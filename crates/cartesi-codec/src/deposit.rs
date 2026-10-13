//! Input payloads added by the portals, as encoded by the `InputEncoding`
//! library of the Cartesi Rollups contracts (v3.0.0-alpha.10).
//!
//! Each deposit starts with fixed-size fields packed together (addresses take
//! 20 bytes, integers 32 bytes). ERC-721 and ERC-1155 deposits follow them
//! with the remaining fields ABI-encoded, as Solidity's `abi.encode` does.

use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolValue;
use std::fmt;

const ADDRESS_LEN: usize = 20;
const UINT_LEN: usize = 32;

/// An Ether deposit, added by the `EtherPortal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtherDeposit {
    /// The Ether sender.
    pub sender: Address,
    /// The amount of Wei sent.
    pub value: U256,
    /// Additional data to be interpreted by the execution layer.
    pub exec_layer_data: Bytes,
}

impl EtherDeposit {
    const PACKED_LEN: usize = ADDRESS_LEN + UINT_LEN;

    /// Encodes the deposit as the `EtherPortal` does.
    pub fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(Self::PACKED_LEN + self.exec_layer_data.len());
        payload.extend_from_slice(self.sender.as_slice());
        payload.extend_from_slice(&self.value.to_be_bytes::<UINT_LEN>());
        payload.extend_from_slice(&self.exec_layer_data);
        payload
    }

    /// Decodes a payload added by the `EtherPortal`.
    pub fn decode(payload: &[u8]) -> Result<Self, DepositDecodeError> {
        let mut fields = PackedFields::new(payload, Self::PACKED_LEN)?;
        Ok(Self {
            sender: fields.address(),
            value: fields.uint(),
            exec_layer_data: Bytes::copy_from_slice(fields.rest()),
        })
    }
}

/// An ERC-20 token deposit, added by the `Erc20Portal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc20Deposit {
    /// The token contract.
    pub token: Address,
    /// The token sender.
    pub sender: Address,
    /// The amount of tokens sent.
    pub value: U256,
    /// Additional data to be interpreted by the execution layer.
    pub exec_layer_data: Bytes,
}

impl Erc20Deposit {
    const PACKED_LEN: usize = 2 * ADDRESS_LEN + UINT_LEN;

    /// Encodes the deposit as the `Erc20Portal` does.
    pub fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(Self::PACKED_LEN + self.exec_layer_data.len());
        payload.extend_from_slice(self.token.as_slice());
        payload.extend_from_slice(self.sender.as_slice());
        payload.extend_from_slice(&self.value.to_be_bytes::<UINT_LEN>());
        payload.extend_from_slice(&self.exec_layer_data);
        payload
    }

    /// Decodes a payload added by the `Erc20Portal`.
    pub fn decode(payload: &[u8]) -> Result<Self, DepositDecodeError> {
        let mut fields = PackedFields::new(payload, Self::PACKED_LEN)?;
        Ok(Self {
            token: fields.address(),
            sender: fields.address(),
            value: fields.uint(),
            exec_layer_data: Bytes::copy_from_slice(fields.rest()),
        })
    }
}

/// An ERC-721 token deposit, added by the `Erc721Portal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc721Deposit {
    /// The token contract.
    pub token: Address,
    /// The token sender.
    pub sender: Address,
    /// The token identifier.
    pub token_id: U256,
    /// Additional data to be interpreted by the base layer, forwarded to the
    /// token contract.
    pub base_layer_data: Bytes,
    /// Additional data to be interpreted by the execution layer.
    pub exec_layer_data: Bytes,
}

impl Erc721Deposit {
    const PACKED_LEN: usize = 2 * ADDRESS_LEN + UINT_LEN;

    /// Encodes the deposit as the `Erc721Portal` does.
    pub fn encode(&self) -> Vec<u8> {
        let data = (&self.base_layer_data, &self.exec_layer_data).abi_encode_params();
        let mut payload = Vec::with_capacity(Self::PACKED_LEN + data.len());
        payload.extend_from_slice(self.token.as_slice());
        payload.extend_from_slice(self.sender.as_slice());
        payload.extend_from_slice(&self.token_id.to_be_bytes::<UINT_LEN>());
        payload.extend_from_slice(&data);
        payload
    }

    /// Decodes a payload added by the `Erc721Portal`.
    pub fn decode(payload: &[u8]) -> Result<Self, DepositDecodeError> {
        let mut fields = PackedFields::new(payload, Self::PACKED_LEN)?;
        let token = fields.address();
        let sender = fields.address();
        let token_id = fields.uint();
        let (base_layer_data, exec_layer_data) =
            <(Bytes, Bytes)>::abi_decode_params(fields.rest())?;
        Ok(Self {
            token,
            sender,
            token_id,
            base_layer_data,
            exec_layer_data,
        })
    }
}

/// An ERC-1155 single token deposit, added by the `Erc1155SinglePortal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc1155SingleDeposit {
    /// The token contract.
    pub token: Address,
    /// The token sender.
    pub sender: Address,
    /// The token identifier.
    pub token_id: U256,
    /// The amount of tokens sent.
    pub value: U256,
    /// Additional data to be interpreted by the base layer, forwarded to the
    /// token contract.
    pub base_layer_data: Bytes,
    /// Additional data to be interpreted by the execution layer.
    pub exec_layer_data: Bytes,
}

impl Erc1155SingleDeposit {
    const PACKED_LEN: usize = 2 * ADDRESS_LEN + 2 * UINT_LEN;

    /// Encodes the deposit as the `Erc1155SinglePortal` does.
    pub fn encode(&self) -> Vec<u8> {
        let data = (&self.base_layer_data, &self.exec_layer_data).abi_encode_params();
        let mut payload = Vec::with_capacity(Self::PACKED_LEN + data.len());
        payload.extend_from_slice(self.token.as_slice());
        payload.extend_from_slice(self.sender.as_slice());
        payload.extend_from_slice(&self.token_id.to_be_bytes::<UINT_LEN>());
        payload.extend_from_slice(&self.value.to_be_bytes::<UINT_LEN>());
        payload.extend_from_slice(&data);
        payload
    }

    /// Decodes a payload added by the `Erc1155SinglePortal`.
    pub fn decode(payload: &[u8]) -> Result<Self, DepositDecodeError> {
        let mut fields = PackedFields::new(payload, Self::PACKED_LEN)?;
        let token = fields.address();
        let sender = fields.address();
        let token_id = fields.uint();
        let value = fields.uint();
        let (base_layer_data, exec_layer_data) =
            <(Bytes, Bytes)>::abi_decode_params(fields.rest())?;
        Ok(Self {
            token,
            sender,
            token_id,
            value,
            base_layer_data,
            exec_layer_data,
        })
    }
}

/// An ERC-1155 batch token deposit, added by the `Erc1155BatchPortal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc1155BatchDeposit {
    /// The token contract.
    pub token: Address,
    /// The token sender.
    pub sender: Address,
    /// The token identifiers.
    pub token_ids: Vec<U256>,
    /// The amount of tokens sent, per token identifier.
    pub values: Vec<U256>,
    /// Additional data to be interpreted by the base layer, forwarded to the
    /// token contract.
    pub base_layer_data: Bytes,
    /// Additional data to be interpreted by the execution layer.
    pub exec_layer_data: Bytes,
}

impl Erc1155BatchDeposit {
    const PACKED_LEN: usize = 2 * ADDRESS_LEN;

    /// Encodes the deposit as the `Erc1155BatchPortal` does.
    pub fn encode(&self) -> Vec<u8> {
        let data = (
            self.token_ids.as_slice(),
            self.values.as_slice(),
            &self.base_layer_data,
            &self.exec_layer_data,
        )
            .abi_encode_params();
        let mut payload = Vec::with_capacity(Self::PACKED_LEN + data.len());
        payload.extend_from_slice(self.token.as_slice());
        payload.extend_from_slice(self.sender.as_slice());
        payload.extend_from_slice(&data);
        payload
    }

    /// Decodes a payload added by the `Erc1155BatchPortal`.
    pub fn decode(payload: &[u8]) -> Result<Self, DepositDecodeError> {
        let mut fields = PackedFields::new(payload, Self::PACKED_LEN)?;
        let token = fields.address();
        let sender = fields.address();
        let (token_ids, values, base_layer_data, exec_layer_data) =
            <(Vec<U256>, Vec<U256>, Bytes, Bytes)>::abi_decode_params(fields.rest())?;
        Ok(Self {
            token,
            sender,
            token_ids,
            values,
            base_layer_data,
            exec_layer_data,
        })
    }
}

/// Why a deposit payload could not be decoded.
#[derive(Debug)]
#[non_exhaustive]
pub enum DepositDecodeError {
    /// The payload is shorter than the deposit's packed fields.
    PayloadTooShort {
        expected_at_least: usize,
        got: usize,
    },
    /// The ABI-encoded part of the payload is malformed.
    InvalidAbiData(alloy_sol_types::Error),
}

impl fmt::Display for DepositDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooShort {
                expected_at_least,
                got,
            } => write!(
                f,
                "deposit payload too short: expected at least {expected_at_least} bytes, got {got}"
            ),
            Self::InvalidAbiData(_) => write!(f, "deposit payload has malformed ABI data"),
        }
    }
}

impl std::error::Error for DepositDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PayloadTooShort { .. } => None,
            Self::InvalidAbiData(e) => Some(e),
        }
    }
}

impl From<alloy_sol_types::Error> for DepositDecodeError {
    fn from(e: alloy_sol_types::Error) -> Self {
        Self::InvalidAbiData(e)
    }
}

/// Reads the packed fields at the start of a payload, in order.
struct PackedFields<'a> {
    payload: &'a [u8],
}

impl<'a> PackedFields<'a> {
    /// Fails unless `payload` holds at least `packed_len` bytes, so the
    /// readers below cannot run past its end.
    fn new(payload: &'a [u8], packed_len: usize) -> Result<Self, DepositDecodeError> {
        if payload.len() < packed_len {
            return Err(DepositDecodeError::PayloadTooShort {
                expected_at_least: packed_len,
                got: payload.len(),
            });
        }
        Ok(Self { payload })
    }

    fn take(&mut self, len: usize) -> &'a [u8] {
        let (field, rest) = self.payload.split_at(len);
        self.payload = rest;
        field
    }

    fn address(&mut self) -> Address {
        Address::from_slice(self.take(ADDRESS_LEN))
    }

    fn uint(&mut self) -> U256 {
        U256::from_be_slice(self.take(UINT_LEN))
    }

    fn rest(self) -> &'a [u8] {
        self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, bytes, hex};

    // The expected payloads were produced by `InputEncoding` from
    // cartesi/rollups-contracts v3.0.0-alpha.10, with these inputs.
    const TOKEN: Address = address!("1111111111111111111111111111111111111111");
    const SENDER: Address = address!("2222222222222222222222222222222222222222");
    const BASE_LAYER_DATA: Bytes = bytes!("ba5e");
    const EXEC_LAYER_DATA: Bytes = bytes!("abcd");

    fn assert_round_trip<T>(
        deposit: &T,
        expected: &[u8],
        encode: fn(&T) -> Vec<u8>,
        decode: fn(&[u8]) -> Result<T, DepositDecodeError>,
    ) where
        T: PartialEq + fmt::Debug,
    {
        assert_eq!(hex::encode(encode(deposit)), hex::encode(expected));
        assert_eq!(&decode(expected).unwrap(), deposit);
    }

    #[test]
    fn ether_deposit() {
        let deposit = EtherDeposit {
            sender: SENDER,
            value: U256::from(5),
            exec_layer_data: EXEC_LAYER_DATA,
        };
        let expected = hex!(
            "2222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000005"
            "abcd"
        );
        assert_round_trip(
            &deposit,
            &expected,
            EtherDeposit::encode,
            EtherDeposit::decode,
        );
    }

    #[test]
    fn erc20_deposit() {
        let deposit = Erc20Deposit {
            token: TOKEN,
            sender: SENDER,
            value: U256::from(5),
            exec_layer_data: EXEC_LAYER_DATA,
        };
        let expected = hex!(
            "1111111111111111111111111111111111111111"
            "2222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000005"
            "abcd"
        );
        assert_round_trip(
            &deposit,
            &expected,
            Erc20Deposit::encode,
            Erc20Deposit::decode,
        );
    }

    #[test]
    fn erc721_deposit() {
        let deposit = Erc721Deposit {
            token: TOKEN,
            sender: SENDER,
            token_id: U256::from(7),
            base_layer_data: BASE_LAYER_DATA,
            exec_layer_data: EXEC_LAYER_DATA,
        };
        let expected = hex!(
            "1111111111111111111111111111111111111111"
            "2222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000007"
            "0000000000000000000000000000000000000000000000000000000000000040"
            "0000000000000000000000000000000000000000000000000000000000000080"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "ba5e000000000000000000000000000000000000000000000000000000000000"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "abcd000000000000000000000000000000000000000000000000000000000000"
        );
        assert_round_trip(
            &deposit,
            &expected,
            Erc721Deposit::encode,
            Erc721Deposit::decode,
        );
    }

    #[test]
    fn erc1155_single_deposit() {
        let deposit = Erc1155SingleDeposit {
            token: TOKEN,
            sender: SENDER,
            token_id: U256::from(7),
            value: U256::from(5),
            base_layer_data: BASE_LAYER_DATA,
            exec_layer_data: EXEC_LAYER_DATA,
        };
        let expected = hex!(
            "1111111111111111111111111111111111111111"
            "2222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000007"
            "0000000000000000000000000000000000000000000000000000000000000005"
            "0000000000000000000000000000000000000000000000000000000000000040"
            "0000000000000000000000000000000000000000000000000000000000000080"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "ba5e000000000000000000000000000000000000000000000000000000000000"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "abcd000000000000000000000000000000000000000000000000000000000000"
        );
        assert_round_trip(
            &deposit,
            &expected,
            Erc1155SingleDeposit::encode,
            Erc1155SingleDeposit::decode,
        );
    }

    #[test]
    fn erc1155_batch_deposit() {
        let deposit = Erc1155BatchDeposit {
            token: TOKEN,
            sender: SENDER,
            token_ids: vec![U256::from(1), U256::from(2)],
            values: vec![U256::from(10), U256::from(20)],
            base_layer_data: BASE_LAYER_DATA,
            exec_layer_data: EXEC_LAYER_DATA,
        };
        let expected = hex!(
            "1111111111111111111111111111111111111111"
            "2222222222222222222222222222222222222222"
            "0000000000000000000000000000000000000000000000000000000000000080"
            "00000000000000000000000000000000000000000000000000000000000000e0"
            "0000000000000000000000000000000000000000000000000000000000000140"
            "0000000000000000000000000000000000000000000000000000000000000180"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "0000000000000000000000000000000000000000000000000000000000000001"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "000000000000000000000000000000000000000000000000000000000000000a"
            "0000000000000000000000000000000000000000000000000000000000000014"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "ba5e000000000000000000000000000000000000000000000000000000000000"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "abcd000000000000000000000000000000000000000000000000000000000000"
        );
        assert_round_trip(
            &deposit,
            &expected,
            Erc1155BatchDeposit::encode,
            Erc1155BatchDeposit::decode,
        );
    }

    #[test]
    fn short_payloads_are_rejected() {
        let short = [0u8; 39];
        let expected_at_least = |result: Result<_, DepositDecodeError>| match result {
            Err(DepositDecodeError::PayloadTooShort {
                expected_at_least,
                got,
            }) => {
                assert_eq!(got, short.len());
                expected_at_least
            }
            other => panic!("expected PayloadTooShort, got {other:?}"),
        };
        assert_eq!(
            expected_at_least(EtherDeposit::decode(&short).map(drop)),
            52
        );
        assert_eq!(
            expected_at_least(Erc20Deposit::decode(&short).map(drop)),
            72
        );
        assert_eq!(
            expected_at_least(Erc721Deposit::decode(&short).map(drop)),
            72
        );
        assert_eq!(
            expected_at_least(Erc1155SingleDeposit::decode(&short).map(drop)),
            104
        );
        assert_eq!(
            expected_at_least(Erc1155BatchDeposit::decode(&short).map(drop)),
            40
        );
    }

    #[test]
    fn malformed_abi_data_is_rejected() {
        // The packed fields of an ERC-721 deposit, followed by an offset that
        // points past the end of the payload.
        let mut payload = vec![0u8; 72];
        payload.extend_from_slice(&U256::from(0x1000).to_be_bytes::<32>());
        payload.extend_from_slice(&U256::from(0x1000).to_be_bytes::<32>());
        assert!(matches!(
            Erc721Deposit::decode(&payload),
            Err(DepositDecodeError::InvalidAbiData(_))
        ));
    }
}
