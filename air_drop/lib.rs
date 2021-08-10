#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod air_drop {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    use erc20::Erc20;
    use ink_prelude::collections::BTreeMap;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;

    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,)]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct TokenStandardInstance {
        erc20: Option<Erc20>,
        //erc721: Erc721,
    }
    
    #[ink(storage)]
    pub struct AirDrop {
        /// token standard name and contract code hash
        owner: AccountId,
        info: TokenStandardInstance,
    }

    #[ink(event)]
    pub struct AirDropTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: u64,
    }

    #[ink(event)]
    pub struct AirDropApproval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: u64,
    }

    impl AirDrop {
        // new function
        #[ink(constructor)]
        pub fn new(
            token_standard: String,
            erc20_code_hash: Hash,
            name: String,
            symbol: String,
            initial_supply: u64, 
            decimals: u8, 
            controller: AccountId
        ) -> Self {
            let owner = Self::env().caller();
            let total_balance = Self::env().balance();
            let mut info = TokenStandardInstance{ erc20: None };

            // match &token_standard.to_string()[..] {
            match &token_standard.to_string()[..] {    
                "erc20" | "ERC20" => {
                    let erc20_instance = Erc20::new(name, symbol, initial_supply, decimals, controller)
                        .endowment(total_balance / 4)
                        .code_hash(erc20_code_hash)
                        .salt_bytes(1_i32.to_le_bytes())
                        .instantiate()
                        .expect("failed at instantiating the `Erc20` contract.");
                        info.erc20 = Some(erc20_instance);
                },
                // "erc721" | "ERC721" => {
                //    TODO:
                // },
                _ => {
                    // TODO:
                },
            }
            //info.insert(token_standard, token_standard_instance);
            Self { owner, info }  
        }

        // query airDrop info
        #[ink(message)]
        pub fn get(&self, account_id: AccountId) -> u64 {
            if let Some(er20_instance) = self.info.erc20.clone() {
                return er20_instance.balance_of(account_id);
            }
            0
        }

        // do function
        #[ink(message)]
        pub fn invoke_list(&mut self, _token_standard: String, address_list: BTreeMap<AccountId, u64>) -> bool {
            if let Some(mut erc20_instance) = self.info.erc20.clone() {
                let mut total_value = 0 as u64;
                for (_, value) in &address_list {
                    total_value += value
                }

                erc20_instance.approve_from(self.env().caller(), self.env().account_id(), total_value);
                self.env().emit_event(AirDropApproval {
                    owner: self.env().caller(),
                    spender: self.env().account_id(),
                    value: total_value,
                });

                for (spender, value) in &address_list {
                    let spender = *spender;
                    let value = *value;

                    ink_env::debug_println!("wasm contract is air_drop: address {:?}, value {}", spender, value);
                    erc20_instance.transfer_from(self.env().caller(), spender, value);
                }
                // ink_env::debug_println!("wasm contract is air_drop: contract_id {:?}", self.env().account_id());
                return true;
            }
                // TODO: erc721 tranfer
                // if let Some(erc721_instance) = v.erc721 {
                    // for (spender, value) in address_list {
                        // erc721_instancev.transfer_from(Self.env().caller(), spender, value)
                    // }
                // }
            false    
        }
    }
}

