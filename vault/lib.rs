#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
//use ink_prelude::vec::Vec;
pub use self::vault::VaultManager;

#[ink::contract]
mod vault {

    use alloc::string::String;

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},

    };

    use erc20::Erc20;
    use org::OrgManager;

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Transfer {
        transfer_id:u64,
        transfer_direction:u64,// 1: out 2 : in
        token_name: String,
        from_address:AccountId,
        to_address:AccountId,
        value: u64,
        transfer_time:u64,
    }





    #[ink(storage)]
    pub struct VaultManager {

        tokens: StorageHashMap<AccountId, AccountId>,
        visible_tokens: StorageHashMap<AccountId, AccountId>,
        transfer_history:StorageHashMap<u64,Transfer>,
        org_contract_address:AccountId,
        vault_contract_address:AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        InvalidTransferRecord,
    }


    #[ink(event)]
    pub struct AddVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }




    #[ink(event)]
    pub struct RemoveVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }



    #[ink(event)]
    pub struct GetTokenBalanceEvent {
        #[ink(topic)]
        token_address:AccountId,

        #[ink(topic)]
        balance:u64,
    }

    #[ink(event)]
    pub struct DepositTokenEvent {

        #[ink(topic)]
        token_name:String,
        #[ink(topic)]
        from_address:AccountId,

        #[ink(topic)]
        value:u64,
    }


    #[ink(event)]
    pub struct WithdrawTokenEvent {
        #[ink(topic)]
        token_name:String,

        #[ink(topic)]
        to_address:AccountId,

        #[ink(topic)]
        value:u64,
    }





    impl VaultManager {

        #[ink(constructor)]
        pub fn new(org_contract_address: AccountId) -> Self {

            let vault_contract_address = Self::env().account_id();

            Self {
                org_contract_address:org_contract_address,
                tokens: StorageHashMap::default(),
                visible_tokens: StorageHashMap::default(),
                transfer_history: StorageHashMap::default(),
                vault_contract_address: vault_contract_address,


            }
        }



        pub fn get_erc20_by_address(&self, address:AccountId) -> Erc20 {
            let  erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(address);
            erc20_instance

        }

        pub fn get_orgmanager_by_address(&self, address:AccountId) -> OrgManager {
            let  org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(address);
            org_instance

        }

        #[ink(message)]
        pub fn check_authority(&self, caller:AccountId) -> bool {
            
            return true;
            let  org = self.get_orgmanager_by_address(self.org_contract_address);

            let creator = org.get_dao_creator();
            let moderator_list = org.get_dao_moderator_list();

            if caller == creator {
                return true;
            }
            for key in moderator_list {
                let moderator = key;
                if caller == moderator {
                    return true;
                }
            }
            return false;

        }



        #[ink(message)]
        pub fn add_vault_token(&mut self,erc_20_address:AccountId) -> bool  {

            let caller = self.env().caller();


             let can_operate = self.check_authority(caller);


            if can_operate == false {
                return false;
            }


            match self.tokens.insert(
                                     erc_20_address,self.vault_contract_address
            ) {

                Some(_) => { false},
                None => {
                    self.visible_tokens.insert(
                                               erc_20_address,self.vault_contract_address);



                    self.env().emit_event(AddVaultTokenEvent{
                        token_address:erc_20_address,
                        });
                    true
                }
            }
        }


        #[ink(message)]
        pub fn remove_vault_token(&mut self,erc_20_address: AccountId) -> bool  {

            let caller = self.env().caller();
            let can_operate = self.check_authority(caller);

            if can_operate == false {
                return false;
            }

            match self.visible_tokens.take(&erc_20_address) {
                None => { false}
                Some(_) => {

                    self.env().emit_event(RemoveVaultTokenEvent{
                        token_address:erc_20_address,
                        });
                    true
                }
            }
        }


        #[ink(message)]
        pub fn get_token_list(&self) -> ink_prelude::vec::Vec<AccountId> {
            self.visible_tokens.keys();
            let mut v:ink_prelude::vec::Vec<AccountId> = ink_prelude::vec::Vec::new();
            for key in self.visible_tokens.keys() {
                v.push(*key)
            }
            v
        }



        #[ink(message)]
        pub fn get_balance_of(&self,erc_20_address: AccountId) -> u64 {

            if self.tokens.contains_key(&erc_20_address) {

               // let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let  erc_20 = self.get_erc20_by_address(erc_20_address);
                //let token_name = (&erc_20).name();
                let balanceof = erc_20.balance_of(self.vault_contract_address);


                self.env().emit_event(GetTokenBalanceEvent{
                    token_address:erc_20_address,
                    balance:balanceof,});

                balanceof

            } else{
                0
            }
        }


        #[ink(message)]
        pub fn deposit(&mut self, erc_20_address:AccountId, from_address:AccountId,value:u64) -> bool {

            let to_address = self.vault_contract_address;

            if self.tokens.contains_key(&erc_20_address) {

                // let  balanceof =  self.get_balance_of(erc_20_address);


                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                erc_20.transfer_from(from_address,to_address, value);

                let transfer_id:u64 = (self.transfer_history.len()+1).into();

                let transfer_time: u64 = self.env().block_timestamp();


                self.transfer_history.insert(transfer_id,
                                             Transfer{

                                                 transfer_direction:2,// 1: out 2: in
                                                 token_name:token_name.clone(),
                                                 transfer_id:transfer_id,
                                                 from_address:from_address,
                                                 to_address:to_address,
                                                 value,
                                                 transfer_time});


                self.env().emit_event(DepositTokenEvent{
                    token_name: token_name.clone(),
                    from_address:from_address,
                    value:value});
                true

            } else{
                false
            }
        }



        #[ink(message)]
        pub fn withdraw(&mut self,erc_20_address:AccountId,to_address:AccountId,value:u64) -> bool {

            let from_address = self.vault_contract_address;

            if self.visible_tokens.contains_key(&erc_20_address) {


                let caller = self.env().caller();
                let can_operate = self.check_authority(caller);

                if can_operate == false {
                    return false;
                }


                // let  balanceof =  self.get_balance_of(erc_20_address);


                //let mut erc_20 = self.get_erc20_by_address(*erc_20_address.unwrap());
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);

                let token_name = (&erc_20).name();

                //erc_20.transfer_from(from_address,to_address, value);
                erc_20.transfer(to_address, value);



                let transfer_id:u64 = (self.transfer_history.len()+1).into();

                let transfer_time: u64 = self.env().block_timestamp();

                self.transfer_history.insert(transfer_id,
                                             Transfer{
                                                 transfer_direction:1,// 1: out 2: in
                                                 token_name: token_name.clone(),
                                                 transfer_id:transfer_id,
                                                 from_address:from_address,
                                                 to_address:to_address,
                                                 value:value,
                                                 transfer_time:transfer_time});




                self.env().emit_event(WithdrawTokenEvent{
                    token_name: token_name.clone(),
                    to_address:to_address,
                    value:value,});

                true

            } else{
                false
            }
        }



        #[ink(message)]
        pub fn get_transfer_history(&self) -> ink_prelude::vec::Vec<Transfer> {
            let mut temp_vec = ink_prelude::vec::Vec::new();
            let mut iter = self.transfer_history.values();
            let mut temp = iter.next();
            while temp.is_some() {
                temp_vec.push(temp.unwrap().clone());
                temp = iter.next();
            }
            temp_vec
        }


    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_vault_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            assert_eq!(vault_manager.org_id, 1);
        }

        #[ink::test]
        fn add_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
        }


        #[ink::test]
        fn remove_token_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.remove_vault_token(accounts.bob);
            assert_eq!(vault_manager.tokens.len(), 1);
            assert_eq!(vault_manager.visible_tokens.len(), 0);
        }


        #[ink::test]
        fn get_token_list_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            let dot_name = String::from("dot");
            vault_manager.add_vault_token(dot_name,accounts.alice);
            assert_eq!(vault_manager.get_token_list().len(), 2);
        }


        #[ink::test]
        fn get_balance_of_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            assert_eq!(vault_manager.get_balance_of(accounts.bob), 0);
        }



        #[ink::test]
        fn deposit_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob),100);

        }


        #[ink::test]
        fn withdraw_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.eve,1000);
            vault_manager.withdraw(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_balance_of(accounts.bob),900);

        }


        #[ink::test]
        fn transfer_history_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut vault_manager = VaultManager::new(1);
            let eth_name = String::from("eth");
            vault_manager.add_vault_token(eth_name,accounts.bob);
            vault_manager.deposit(accounts.bob,accounts.eve,1000);
            vault_manager.withdraw(accounts.bob,accounts.alice,100);
            assert_eq!(vault_manager.get_transfer_history().len(),2);

        }

    }
}
