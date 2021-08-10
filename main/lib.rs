#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod main {
    #[cfg(not(feature = "ink-as-dependency"))]
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    // use ink_prelude::string::String;
    use dao_manager::DAOManager;
    use org::OrgManager;
    use template_manager::TemplateManager;
    use template_manager::DAOTemplate;
    const one_unit: u128 = 1_000_000_000_000;
    const template_init_balance: u128 = 100 * 1000 * 1_000_000_000_000;
    const dao_init_balance: u128 = 1000 * 1000 * 1_000_000_000_000;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        dao_manager: DAOManager,
        dao_manager_addr: AccountId,
    }

    #[ink(storage)]
    pub struct Main {
        owner: AccountId,
        template_addr: Option<AccountId>,
        template: Option<TemplateManager>,
        instance_index: u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
        instance_map_by_owner: StorageHashMap<AccountId, Vec<u64>>,
    }

    #[ink(event)]
    pub struct InstanceDAO {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        dao_addr: AccountId,
    }

    impl Main {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_addr: None,
                template: None,
                instance_index: 0,
                instance_map: StorageHashMap::new(),
                instance_map_by_owner: StorageHashMap::new(),
            };
            instance
        }
        #[ink(message)]
        pub fn  init (&mut self, template_code_hash: Hash, salt: Vec<u8>) -> bool 
        {
            let total_balance = Self::env().balance();
            // instance template_manager
            // let salt = version.to_le_bytes();
            let instance_params = TemplateManager::new(self.owner)
                .endowment(template_init_balance)
                .code_hash(template_code_hash)
                .salt_bytes(&salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `TemplateManager` contract");
            let contract_instance = ink_env::call::FromAccountId::from_account_id(contract_addr);

            self.template = Some(contract_instance);
            self.template_addr = Some(contract_addr);
            true
        }

        #[ink(message)]
        pub fn add_template(&mut self, name: String, dao_manager_code_hash: Hash, components: BTreeMap<String, Hash>) -> bool {
            self.template.as_mut().unwrap().add_template(name, dao_manager_code_hash, components)
        }

        #[ink(message)]
        pub fn list_templates(&self) -> Vec<DAOTemplate> {
            self.template.as_ref().unwrap().list_templates()
        }

        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> DAOTemplate {
            self.template.as_ref().unwrap().query_template_by_index(index)
        }

        #[ink(message)]
        pub fn query_template_addr(&self) -> AccountId {
            self.template_addr.unwrap()
        }

        #[ink(message)]
        pub fn instance_by_template(&mut self, index: u64, controller: AccountId, salt: Vec<u8>) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            let total_balance = Self::env().balance();
            // assert_eq!(total_balance >= 20, true);

            // instance dao_manager
            let template = self.query_template_by_index(index);
            let dao_manager_code_hash = template.dao_manager_code_hash;
            // let salt = version.to_le_bytes();
            let dao_instance_params = DAOManager::new(controller, self.instance_index)
                .endowment(dao_init_balance)
                .code_hash(dao_manager_code_hash)
                .salt_bytes(salt)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let mut dao_instance: DAOManager = ink_env::call::FromAccountId::from_account_id(dao_addr);
            dao_instance.set_template(template);
            self.env().emit_event(InstanceDAO {
                index: self.instance_index,
                owner: Some(controller),
                dao_addr: dao_addr,
            });

            let id_list = self.instance_map_by_owner.entry(controller.clone()).or_insert(Vec::new());
            id_list.push(self.instance_index);
            self.instance_map.insert(self.instance_index, DAOInstance {
                id: self.instance_index,
                owner: controller,
                dao_manager: dao_instance,
                dao_manager_addr: dao_addr,
            });
            self.instance_index += 1;
            true
        }

        #[ink(message)]
        pub fn list_dao_instances(&mut self) -> Vec<DAOInstance> {
            let mut dao_vec = Vec::new();
            let mut iter = self.instance_map.values();
            let mut temp = iter.next();
            while temp.is_some() {
                let dao = Main::fillOrgOwner(temp.unwrap().clone());
                dao_vec.push(dao);
                temp = iter.next();
            }
            dao_vec
        }

        fn fillOrgOwner(mut dao: DAOInstance) -> DAOInstance {
            let org_addr_op = dao.dao_manager.query_component_addrs().org_addr;
            if org_addr_op.is_none() {
                return dao
            }
            let org_addr: AccountId = org_addr_op.unwrap();
            let org_instance: OrgManager = ink_env::call::FromAccountId::from_account_id(org_addr);
            dao.owner = org_instance.get_dao_owner();
            dao
        }

        #[ink(message)]
        pub fn list_dao_instances_by_owner(&mut self, owner: AccountId) -> Vec<DAOInstance> {

            let mut dao_vec = Vec::new();
            let mut iter = self.instance_map.values();
            let mut temp = iter.next();
            while temp.is_some() {
                let dao: DAOInstance = Main::fillOrgOwner(temp.unwrap().clone());
                if owner == dao.owner {
                    dao_vec.push(dao);
                }
                temp = iter.next();
            }
            dao_vec
        }
    }
}
