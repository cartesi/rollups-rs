#![doc = include_str!("../README.md")]

pub use alloy_primitives;
pub use alloy_sol_types;

use alloy_sol_types::sol;

mod deposit;

pub use deposit::{
    DepositDecodeError, Erc20Deposit, Erc721Deposit, Erc1155BatchDeposit, Erc1155SingleDeposit,
    EtherDeposit,
};

sol! {
    /// @notice An advance request from an EVM-compatible blockchain to a Cartesi Machine.
    /// @param chainId The chain ID
    /// @param appContract The application contract address
    /// @param msgSender The address of whoever sent the input
    /// @param blockNumber The number of the block in which the input was added
    /// @param blockTimestamp The timestamp of the block in which the input was added
    /// @param prevRandao The latest RANDAO mix of the post beacon state of the previous block
    /// @param index The index of the input in the input box
    /// @param payload The payload provided by the message sender
    /// @dev See EIP-4399 for safe usage of `prevRandao`.
    #[derive(Debug, PartialEq, Eq)]
    function EvmAdvance(
        uint256 chainId,
        address appContract,
        address msgSender,
        uint256 blockNumber,
        uint256 blockTimestamp,
        uint256 prevRandao,
        uint256 index,
        bytes calldata payload
    ) external;

    /// @notice A piece of verifiable information.
    /// @param payload An arbitrary payload.
    #[derive(Debug, PartialEq, Eq)]
    function Notice(bytes calldata payload) external;

    /// @notice A single-use permission to execute a specific message call
    /// from the context of the application contract.
    /// @param destination The address that will be called
    /// @param value The amount of Wei to be transferred through the call
    /// @param payload The payload, which—in the case of Solidity
    /// contracts—encodes a function call
    #[derive(Debug, PartialEq, Eq)]
    function Voucher(
        address destination,
        uint256 value,
        bytes calldata payload
    ) external;

    /// @notice A single-use permission to execute a specific delegate call
    /// from the context of the application contract.
    /// @param destination The address that will be called
    /// @param payload The payload, which—in the case of Solidity
    /// libraries—encodes a function call
    #[derive(Debug, PartialEq, Eq)]
    function DelegateCallVoucher(address destination, bytes calldata payload) external;
}

sol! {
    interface IERC20 {
        #[derive(Debug, PartialEq, Eq)]
        function transfer(address recipient, uint256 amount) external returns (bool);
    }
}

pub type Input = EvmAdvanceCall;
pub type Voucher = VoucherCall;
pub type Notice = NoticeCall;
pub type DelegateCallVoucher = DelegateCallVoucherCall;
pub type Erc20Transfer = IERC20::transferCall;
