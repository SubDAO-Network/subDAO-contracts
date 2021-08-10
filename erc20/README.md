# SubDAO ERC20 Module

ERC20 is a module to generate DAO's token.

## Interfaces

### instance module
instance module.
```bash
type: tx
definition: pub fn new(name: String, symbol: String, initial_supply: u64, decimals: u8, controller: AccountId) -> Self;
```

### query name
query ERC20 name.
```bash
type: query
definition: pub fn name(&self) -> String;
```


### query symbol
query ERC20 symbol.
```bash
type: query
definition: pub fn symbol(&self) -> String;
```

### query supply
query ERC20 total supply.
```bash
type: query
definition: pub fn total_supply(&self) -> u64;
```

### query token info
query ERC20 info.
```bash
type: query
definition: pub fn query_info(&self) -> DisplayInfo;
pub struct DisplayInfo {
        name: String,
        symbol: String,
        total_supply: u64,
        decimals: u8,
        owner: AccountId,
    }
```

### query account's balance
query account's balance.
```bash
type: query
definition: pub fn balance_of(&self, owner: AccountId) -> u64;
```

### query approve mount
query mount of owner approve to spender.
```bash
type: query
definition: pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u64;
```

### transfer token
transfer erc20 to other from caller.
```bash
type: tx
definition: pub fn transfer(&mut self, to: AccountId, value: u64) -> bool;
```

### approve token
approve erc20 to other from caller.
```bash
type: tx
definition: pub fn approve(&mut self, spender: AccountId, value: u64) -> bool;
```

### transfer from spender
transfer erc20 to other from spender.
```bash
type: tx
definition: pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u64,
        ) -> bool;
```

### mint erc20 to account
mint erc20 by owner to account.
```bash
type: tx
definition: pub fn mint_token_by_owner(
            &mut self,
            to: AccountId,
            value: u64,
        ) -> bool;
```

### destroy erc20 to account
destroy erc20 by owner to account.
```bash
type: tx
definition: pub fn destroy_token_by_owner(
            &mut self,
            from: AccountId,
            value: u64,
        ) -> bool;
```

