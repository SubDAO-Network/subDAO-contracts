#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;

pub use self::dao_manager::DAOManager;

#[ink::contract]
mod dao_manager {
    use alloc::string::String;
    use auth::Auth;
    use base::Base;
    use erc20::Erc20;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    use org::OrgManager;
    //    use github::Github;
    use template_manager::DAOTemplate;
    use vault::VaultManager;
    use vote_manager::VoteManager;

    const one_unit: u128 = 1_000_000_000_000;
    const contract_init_balance: u128 = 100 * 1000 * 1_000_000_000_000;

    /// DAO component instances
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        pub base: Option<Base>,
        pub erc20: Option<Erc20>,
        pub org: Option<OrgManager>,
        pub vault: Option<VaultManager>,
        pub vote: Option<VoteManager>,
        pub auth: Option<Auth>,
        //    github: Option<Github>,
    }

    /// DAO component instance addresses
    #[derive(
    Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponentAddrs {
        // base module contract's address
        pub base_addr: Option<AccountId>,
        // erc20 module contract's address
        pub erc20_addr: Option<AccountId>,
        // org module contract's address
        pub org_addr: Option<AccountId>,
        // vault module contract's address
        pub vault_addr: Option<AccountId>,
        // vote module contract's address
        pub vote_addr: Option<AccountId>,
        // auth module contract's address
        pub auth_addr: Option<AccountId>,
        // github module contract's address
        // github_addr: Option<AccountId>,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct BaseParam {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct ERC20Param {
        owner: AccountId,
        name: String,
        symbol: String,
        total_supply: u64,
        decimals: u8,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct OrgParam {
        owner: AccountId,
        moderators: BTreeMap<String, AccountId>,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct AuthParam {
        owner: AccountId,
    }

    /// DAO component instance addresses
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInitParams {
        base: BaseParam,
        erc20: ERC20Param,
        erc20Transfers: BTreeMap<AccountId, u64>,
        org: OrgParam,
        auth: AuthParam,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DAOManager {
        pub init: bool,
        pub owner: AccountId,
        pub org_id: u64,
        pub template: Option<DAOTemplate>,
        pub components: DAOComponents,
        pub component_addrs: DAOComponentAddrs,
    }

    impl DAOManager {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner: AccountId, org_id: u64) -> Self {
            Self {
                init: false,
                owner,
                org_id,
                template: None,
                components: DAOComponents {
                    base: None,
                    erc20: None,
                    org: None,
                    vault: None,
                    vote: None,
                    auth: None,
                    //     github: None,
                },
                component_addrs: DAOComponentAddrs {
                    base_addr: None,
                    erc20_addr: None,
                    org_addr: None,
                    vault_addr: None,
                    vote_addr: None,
                    auth_addr: None,
                    //  github_addr: None,
                },
            }
        }

        #[ink(message)]
        pub fn set_template(&mut self, template: DAOTemplate) -> bool {
            assert_eq!(self.init, false);
            self.template = Some(template);
            true
        }
        #[ink(message)]
        pub fn  get_balance(&mut self) -> u128 {
            let total_balance = Self::env().balance();
            return total_balance;
        }

        #[ink(message)]
        pub fn  init_by_params(&mut self, params: DAOInitParams, salt: Vec<u8>) -> bool {
            assert_eq!(self.init, false);
            assert_eq!(self.template.is_some(), true);
            let owner = self.env().caller();
            assert_eq!(owner == self.owner, true);

            // init components
            let components_hash_map = self.template.as_ref().unwrap().components.clone();
            let base_code_hash = components_hash_map.get("BASE");
            let erc20_code_hash = components_hash_map.get("ERC20");
            let org_code_hash = components_hash_map.get("ORG");
            let vault_code_hash = components_hash_map.get("VAULT");
            let vote_code_hash = components_hash_map.get("VOTE");
            let auth_code_hash = components_hash_map.get("AUTH");
            //  let github_code_hash = components_hash_map.get("GITHUB");

            // let version = self.org_id as u32;
            self._init_base(base_code_hash, params.base, &salt);
            self._init_auth(auth_code_hash, params.auth.clone(), &salt);
            self._init_org(org_code_hash, params.org, &salt);
            self._init_vault(vault_code_hash, &salt);
            self._init_vote(vote_code_hash, &salt);
            self._init_erc20(erc20_code_hash, params.erc20, params.erc20Transfers, &salt);
            // self._init_github(github_code_hash);

            // add vault token
            self._after_init_erc20(erc20_code_hash);
            self._after_init_auth(auth_code_hash, params.auth);
            self.init = true;
            true
        }

        fn _after_init_erc20(&mut self, erc20_code_hash: Option<&Hash>) {
            if erc20_code_hash.is_none() {
                return;
            }
            let erc20_addr = self.component_addrs.erc20_addr.unwrap();
            let vault_addr = self.component_addrs.vault_addr.unwrap();
            let mut vault_instance: VaultManager = ink_env::call::FromAccountId::from_account_id(vault_addr);
            vault_instance.add_vault_token(erc20_addr);
        }

        fn _after_init_auth(&mut self, auth_code_hash: Option<&Hash>, auth: AuthParam) {
            if auth_code_hash.is_none() {
                return;
            }
            let auth_addr = self.component_addrs.auth_addr.unwrap();
            let mut auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(auth_addr);

            // transfer owner
            auth_instance.transfer_owner(auth.owner);
        }

        #[ink(message)]
        pub fn query_component_addrs(&self) -> DAOComponentAddrs {
            self.component_addrs
        }

        /// init base
        fn _init_base(&mut self, base_code_hash: Option<&Hash>,
                      param: BaseParam, salt: &Vec<u8>) -> bool {
            if base_code_hash.is_none() {
                return true;
            }
            let base_code_hash = base_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            // instance base
            // let salt = version.to_le_bytes();
            let instance_params = Base::new()
                .endowment(contract_init_balance)
                .code_hash(base_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Base` contract");
            let mut contract_instance: Base = ink_env::call::FromAccountId::from_account_id(contract_addr);
            contract_instance.init_base(param.name, param.logo, param.desc);

            self.components.base = Some(contract_instance);
            self.component_addrs.base_addr = Some(contract_addr);

            true
        }

        /// init erc20
        fn _init_erc20(&mut self, erc20_code_hash: Option<&Hash>,
                       param: ERC20Param, initTransfers: BTreeMap<AccountId, u64>, salt: &Vec<u8>) -> bool {
            if erc20_code_hash.is_none() {
                return true;
            }
            let erc20_code_hash = erc20_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            let vault_addr = self.component_addrs.vault_addr.unwrap();
            // instance erc20
            // let salt = version.to_le_bytes();
            let erc20_instance_params = Erc20::new(param.name, param.symbol,
                0, param.decimals, Self::env().account_id())
                .endowment(contract_init_balance)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(erc20_addr);

            // transfer tokens
            let mut transfer = 0;
            for (to, amount) in &initTransfers {
                erc20_instance.mint_token_by_owner(*to, *amount);
                transfer += amount;
            }

            erc20_instance.mint_token_by_owner(vault_addr, param.total_supply - transfer);
            erc20_instance.transfer_owner(param.owner);

            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            true
        }

        /// init org
        fn _init_org(&mut self, org_code_hash: Option<&Hash>, param: OrgParam, salt: &Vec<u8>) -> bool {
            if org_code_hash.is_none() {
                return true;
            }
            let org_code_hash = org_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            // instance org
            // let salt = version.to_le_bytes();
            let auth_addr = self.component_addrs.auth_addr.unwrap();

            let org_instance_params = OrgManager::new(Self::env().account_id(), self.org_id, auth_addr)
                .endowment(contract_init_balance)
                .code_hash(org_code_hash)
                .salt_bytes(salt)
                .params();
            let org_init_result = ink_env::instantiate_contract(&org_instance_params);
            let org_addr = org_init_result.expect("failed at instantiating the `Org` contract");
            let mut org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(org_addr);

            let mut auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(auth_addr);
            auth_instance.grant_permission(org_addr, String::from("auth"), String::from("grant"));

            // add dao owner as moderator
            org_instance.add_dao_moderator_without_grant(String::from("Dao Owner"), param.owner);
            auth_instance.grant_permission(param.owner, String::from("vote"), String::from("new"));
            auth_instance.grant_permission(param.owner, String::from("vote"), String::from("vote"));

            // add moderator
            for (name, accountId) in &param.moderators {
                org_instance.add_dao_moderator_without_grant(name.clone(), *accountId);
                auth_instance.grant_permission(*accountId, String::from("vote"), String::from("new"));
                auth_instance.grant_permission(*accountId, String::from("vote"), String::from("vote"));
            }
            org_instance.transfer_ownership(param.owner);

            self.components.org = Some(org_instance);
            self.component_addrs.org_addr = Some(org_addr);
            true
        }


        /// init auth
        fn _init_auth(&mut self, auth_code_hash: Option<&Hash>, auth: AuthParam, salt: &Vec<u8>) -> bool {
            if auth_code_hash.is_none() {
                return true;
            }
            let dao_addr = Self::env().account_id();
            let auth_code_hash = auth_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            // instance auth
            // let salt = version.to_le_bytes();
            let auth_instance_params = Auth::new(dao_addr)
                .endowment(contract_init_balance)
                .code_hash(auth_code_hash)
                .salt_bytes(salt)
                .params();
            let auth_init_result = ink_env::instantiate_contract(&auth_instance_params);
            let auth_addr = auth_init_result.expect("failed at instantiating the `Auth` contract");
            let mut auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(auth_addr);

            // register inner action
            auth_instance.register_action(String::from("vault"), String::from("add_vault_token"), String::from("vault.add_vault_token"));
            auth_instance.register_action(String::from("vote"), String::from("new"), String::from("Create Voting"));
            auth_instance.register_action(String::from("vote"), String::from("vote"), String::from("Vote"));
            auth_instance.register_action(String::from("auth"), String::from("grant"), String::from("Grant/Revoke Permission"));
            auth_instance.register_action(String::from("auth"), String::from("register"), String::from("Register/Cancel Action"));

            // grant inner action
            auth_instance.grant_permission(dao_addr, String::from("vault"), String::from("add_vault_token"));

            self.components.auth = Some(auth_instance);
            self.component_addrs.auth_addr = Some(auth_addr);
            true
        }


        /// init vault
        fn _init_vault(&mut self, vault_code_hash: Option<&Hash>, salt: &Vec<u8>) -> bool {
            if vault_code_hash.is_none() {
                return true;
            }
            let vault_code_hash = vault_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            // instance org
            let org_addr = self.component_addrs.org_addr.unwrap();
            let auth_addr = self.component_addrs.auth_addr.unwrap();
            // let salt = version.to_le_bytes();
            let vault_instance_params = VaultManager::new(org_addr, auth_addr)
                .endowment(contract_init_balance)
                .code_hash(vault_code_hash)
                .salt_bytes(salt)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
            let mut vault_instance: VaultManager = ink_env::call::FromAccountId::from_account_id(vault_addr);
            self.components.vault = Some(vault_instance);
            self.component_addrs.vault_addr = Some(vault_addr);
            true
        }

        /// init vote
        fn _init_vote(&mut self, vote_code_hash: Option<&Hash>, salt: &Vec<u8>) -> bool {
            if vote_code_hash.is_none() {
                return true;
            }
            let vote_code_hash = vote_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > contract_init_balance, "not enough unit to instance contract");
            // instance org
            let vault_addr = self.component_addrs.vault_addr.unwrap();
            let auth_addr = self.component_addrs.auth_addr.unwrap();
            // let salt = version.to_le_bytes();
            let vote_instance_params = VoteManager::new(vault_addr, auth_addr)
                .endowment(contract_init_balance)
                .code_hash(vote_code_hash)
                .salt_bytes(salt)
                .params();
            let vote_init_result = ink_env::instantiate_contract(&vote_instance_params);
            let vote_addr = vote_init_result.expect("failed at instantiating the `Vote` contract");
            let vote_instance = ink_env::call::FromAccountId::from_account_id(vote_addr);
            self.components.vote = Some(vote_instance);
            self.component_addrs.vote_addr = Some(vote_addr);

            let mut auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(auth_addr);
            // register inner action
            auth_instance.register_action(String::from("vault"), String::from("withdraw"), String::from("vault.withdraw"));
            // grant inner action
            auth_instance.grant_permission(vote_addr, String::from("vault"), String::from("withdraw"));

            true
        }

        //// init github
        /*  fn _init_github(&mut self, github_code_hash: Option<&Hash>) -> bool {
              if github_code_hash.is_none() {
                  return true;
              }
              let github_code_hash = github_code_hash.unwrap().clone();
              let total_balance = Self::env().balance();
              // instance org
              let github_instance_params = Github::new()
                  .endowment(total_balance / 4)
                  .code_hash(github_code_hash)
                  .params();
              let github_init_result = ink_env::instantiate_contract(&github_instance_params);
              let github_addr = github_init_result.expect("failed at instantiating the `Github` contract");
              let github_instance = ink_env::call::FromAccountId::from_account_id(github_addr);
              self.components.github = Some(github_instance);
              self.component_addrs.github_addr = Some(github_addr);
              true
          }*/
    }
}
