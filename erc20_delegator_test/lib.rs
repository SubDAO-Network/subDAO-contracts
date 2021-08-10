// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20_delegator_test {
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
        // Vec as StorageVec,
    };
    // use ink_prelude::string::String;
    use erc20::Erc20;
    use erc20_test::Erc20Test;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(Debug, scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOTemplate {
        owner: AccountId,
        erc20_code_hash: Hash,
        erc20_test_code_hash: Hash,
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInstance {
        owner: AccountId,
        erc20: Erc20,
        erc20_test: Erc20Test,
    }

    #[ink(storage)]
    pub struct Erc20DelegatorTest {
        owner: AccountId,
        template_index: u64,
        template_map: StorageHashMap<u64, DAOTemplate>,
        instance_index: u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
    }

    impl Erc20DelegatorTest {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_index: 0,
                template_map: StorageHashMap::new(),
                instance_index: 0,
                instance_map: StorageHashMap::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn add_template(&mut self, erc20_code_hash: Hash, erc20_test_code_hash: Hash) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
            let from = self.env().caller();
            // TODO add template event, declare index, owner
            self.template_map.insert(self.template_index, DAOTemplate {
                owner: from,
                erc20_code_hash,
                erc20_test_code_hash,
            });
            self.template_index += 1;
            true
        }

        #[ink(message)]
        pub fn query_all_template(&self, index: u64) -> DAOTemplate {
            return *self.template_map.get(&index).unwrap()
        }

        #[ink(message)]
        pub fn instance_by_template(&mut self, index: u64, name: String, symbol: String, initial_supply: u64, decimals: u8, controller: AccountId) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let total_balance = Self::env().balance();
            assert_eq!(total_balance >= 20, true);

            // query template info
            let template = self.template_map.get(&index).unwrap();

            // instance erc20
            // TODO add instance event
            let erc_instance = Erc20::new(name, symbol, initial_supply, decimals, controller)
                .endowment(total_balance / 4)
                .code_hash(template.erc20_code_hash)
                .salt_bytes(1_i32.to_le_bytes())
                .instantiate()
                .expect("failed at instantiating the `Erc20` contract");

            // instance erc20 test
            // TODO add instance event, declare index, type, instance/accountId
            let erc_test_instance = Erc20Test::new(erc_instance.clone())
                .endowment(total_balance / 4)
                .code_hash(template.erc20_test_code_hash)
                .salt_bytes(1_i32.to_le_bytes())
                .instantiate()
                .expect("failed at instantiating the `Erc20` contract");

            // put instance
            self.instance_map.insert(self.instance_index, DAOInstance {
                owner: controller,
                erc20: erc_instance,
                erc20_test: erc_test_instance,
            });
            self.instance_index += 1;
            true
        }

        #[ink(message)]
        pub fn transfer(&mut self, index: u64, to: AccountId, value: u64) -> bool {
            let instance = self.instance_map.get_mut(&index).unwrap();
            instance.erc20.transfer(to, value)
        }

        #[ink(message)]
        pub fn transfer_by_erc20_test_in_erc20(&mut self, index: u64, to: AccountId, value: u64) -> bool {
            let instance = self.instance_map.get_mut(&index).unwrap();
            instance.erc20_test.transfer_in_erc20(to, value)
        }

    }
}
