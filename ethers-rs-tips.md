# Ethers-rs Tips
A compiled list of things you might find useful if you are like me and are a bit of a rustacean noob.
As always the [rust-docs](https://docs.rs/ethers/latest/ethers/index.html) can be very helpful.

# Events
## Decoding
Decoding an event log if you dont have the contract bindings. This example is the UniswapV3Pool Swap.
```
use ethers::{types::Address, abi::RawLog, contract::{EthLogDecode,EthEvent} };

#[derive(Debug, Default, EthEvent, PartialEq, Eq)]
#[ethevent(name = "Swap", abi = "Swap(address,address,int256,int256,uint160,uint128,int24)")]
pub struct SwapEventData {
    #[ethevent(indexed)]
    sender: Address,
    #[ethevent(indexed)]
    recipient: Address,
    amount0: i128,
    amount1: u128,
    sqrt_price_x96: u128,
    liquidity: u128,
    tick: i32,
}

let event = <SwapEventData as EthLogDecode>::decode_log(&RawLog::from(event));
```
