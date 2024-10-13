# CosmWasm CPAMM (Constant Product Automated Market Maker)

## Overview

This CosmWasm contract implements a Constant Product Automated Market Maker (CPAMM) for token swaps and liquidity provisioning. It allows users to:

- Swap between two tokens using the x \* y = k invariant.
- Add or remove liquidity.
- Configure fees for liquidity providers and protocol.
- Query pool and price information.

---

## Features

### Initialization

The contract is instantiated with the following parameters:

- **token1_denom**: The denomination of the first token in the pool.
- **token2_denom**: The denomination of the second token in the pool.
- **owner**: (Optional) Address of the contract owner.
- **protocol_fee_recipient**: Address to receive protocol fees.
- **protocol_fee_percent**: Percentage of fees collected for the protocol.
- **lp_fee_percent**: Percentage of fees allocated to liquidity providers.

### Execution Messages

- **AddLiquidity**: Add token1 and token2 to the pool and mint liquidity tokens.
- **RemoveLiquidity**: Remove liquidity from the pool and redeem tokens.
- **Swap**: Swap one token for another, ensuring minimum output and respecting expiration.
- **UpdateConfig**: Update configuration, including fees and protocol fee recipient.
- **FreezeDeposits**: Temporarily prevent liquidity deposits and swaps.

### Query Messages

- **Balance**: Query the pool balance for a given address.
- **Info**: Get detailed pool information, including reserves and token denominations.
- **Token1ForToken2Price**: Calculate the output amount of token2 for a given input amount of token1.
- **Token2ForToken1Price**: Calculate the output amount of token1 for a given input amount of token2.
- **Fee**: Get the current fee configuration.

### Migrations

The **MigrateMsg** supports updating contract parameters, including the owner, protocol fee recipient, and fee percentages. It also allows freezing the pool if needed.

---

## Usage

### Instantiate the Contract

```json
{
  "token1_denom": "denom1",
  "token2_denom": "denom2",
  "owner": "cosmos1...",
  "protocol_fee_recipient": "cosmos1...",
  "protocol_fee_percent": "0.01",
  "lp_fee_percent": "0.02"
}
```

### Execute Examples

#### Add Liquidity

```json
{
  "add_liquidity": {
    "token1_amount": "1000",
    "token2_amount": "2000",
    "min_liquidity": "500",
    "expiration": "2025-01-01T00:00:00Z"
  }
}
```

#### Remove Liquidity

```json
{
  "remove_liquidity": {
    "amount": "100",
    "min_token1": "50",
    "min_token2": "100",
    "expiration": "2025-01-01T00:00:00Z"
  }
}
```

#### Swap Tokens

```json
{
  "swap": {
    "input_token": "token1",
    "input_amount": "100",
    "min_output": "90",
    "expiration": "2025-01-01T00:00:00Z"
  }
}
```

#### Update Configuration

```json
{
  "update_config": {
    "owner": "cosmos1...",
    "protocol_fee_recipient": "cosmos1...",
    "protocol_fee_percent": "0.005",
    "lp_fee_percent": "0.025"
  }
}
```

#### Freeze Deposits

```json
{
  "freeze_deposits": {
    "freeze": true
  }
}
```

### Query Examples

#### Get Pool Info

```json
{
  "info": {}
}
```

#### Get Price

```json
{
  "token1_for_token2_price": {
    "token1_amount": "1000"
  }
}
```

#### Get Fee Configuration

```json
{
  "fee": {}
}
```

---

## Key Design Principles

1. **Fee Management**: The contract ensures that the total fee percentage does not exceed a configurable maximum.
2. **Expiration Handling**: Time-sensitive operations include expiration checks to ensure timely execution.
3. **Ownership and Control**: The owner can update configurations and freeze the pool when necessary.
4. **Modular Queries**: Provides detailed insights into pool state, fees, and price calculations.

---

## Development and Testing

To test the contract, use the CosmWasm testing framework. Sample tests can cover:

1. Adding and removing liquidity.
2. Performing token swaps with various configurations.
3. Querying pool information and fees.
4. Validating fee and expiration constraints.

Run tests using:

```bash
cargo test
```

Deploy to a local blockchain using:

```bash
wasmd tx wasm store <contract.wasm> ...
```

---

## Contributing

Contributions are welcome! Please open issues or pull requests to improve the contract functionality or documentation.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.
