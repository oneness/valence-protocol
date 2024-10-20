# Valence Generic IBC Transfer service

The **Valence Generic IBC Transfer** service allows to transfer funds over IBC from an **input account** on a source chain to an **output account** on a destination chain. It is typically used as part of a **Valence Workflow**. In that context, a **Processor** contract will be the main contract interacting with the Forwarder service.

Note: this service should not be used on Neutron, which requires some fees to be paid to relayers for IBC transfers. For Neutron, prefer using the dedicated (and optimized) **Neutron IBC Transfer service** instead.

## High-level flow

```mermaid
---
title: Generic IBC Transfer Service
---
graph LR
  IA((Input
      Account))
  OA((Output
		  Account))
  P[Processor]
  S[Gen IBC Transfer
    Service]
  subgraph Chain 1
  P -- 1/IbcTransfer --> S
  S -- 2/Query balances --> IA
  S -- 3/Do Send funds --> IA
  end
  subgraph Chain 2
  IA -- 4/IBC Transfer --> OA
  end
```

## Configuration

The service is configured on instantiation via the `ServiceConfig` type.
```rust
struct ServiceConfig {
  // Account from which the funds are pulled (on the source chain)
  input_addr: ServiceAccountType,
  // Account to which the funds are sent (on the destination chain)
  output_addr: String,
  // Denom of the token to transfer
  denom: UncheckedDenom,
  // Amount to be transferred, either a fixed amount or the whole available balance.
  amount: IbcTransferAmount,
  // Memo to be passed in the IBC transfer message.
  memo: String,
  // Information about the destination chain.
  remote_chain_info: RemoteChainInfo,
  // Denom map for the Packet-Forwarding Middleware, to perform a multi-hop transfer.
  denom_to_pfm_map: BTreeMap<String, PacketForwardMiddlewareConfig>,
}

// Defines the amount to be transferred, either a fixed amount or the whole available balance.
enum IbcTransferAmount {
  // Transfer the full available balance of the input account.
  FullAmount,
  // Transfer the specified amount of tokens.
  FixedAmount(Uint128),
}

pub struct RemoteChainInfo {
  // Channel of the IBC connection to be used.
  channel_id: String,
  // Port of  the IBC connection to be used.
  port_id: Option<String>,
  // Timeout for the IBC transfer.
  ibc_transfer_timeout: Option<Uint64>,
}

// Configuration for a multi-hop transfer using the Packet Forwarding Middleware
struct PacketForwardMiddlewareConfig {
  // Channel ID from the source chain to the intermediate chain
  local_to_hop_chain_channel_id: String,
  // Channel ID from the intermediate to the destination chain
  hop_to_destination_chain_channel_id: String,
  // Temporary receiver address on the intermediate chain
  hop_chain_receiver_address: String,
}
```