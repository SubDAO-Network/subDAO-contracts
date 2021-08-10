#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::auth::Auth;

#[ink::contract]
mod auth {


    use alloc::string::String;
    use alloc::vec::Vec;
    
    // #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };


    type ActionId = u32;

    #[derive(scale::Encode,scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Action {
        action_id: ActionId,
        action_title: String,
        contract_name: String,
        function_name: String,
    }

    #[ink(storage)]
    pub struct Auth {
        owner: AccountId,
        action_id: ActionId,
        actions: StorageHashMap<(String, String),Action>,
        actions_auths: StorageHashMap<(AccountId, ActionId), Action>,
    }

    impl Auth {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                action_id: 0,
                actions: StorageHashMap::new(),
                actions_auths: StorageHashMap::new(),
            }
        }
        
        #[ink(message)]
        pub fn has_permission(& self, account_id: AccountId, contract_name: String, function_name: String)  -> bool {
            return self._has_permission(account_id, contract_name, function_name);
        }

        fn _has_permission(& self, account_id: AccountId, contract_name: String, function_name: String)  -> bool {
            if let Some(action) = self.actions.get(&(contract_name, function_name)) {
                if let Some(_) = self.actions_auths.get(&(account_id, action.action_id)) {
                    return true;
                }            
            }
           return false;
        }

        #[ink(message)]
        pub fn grant_permission(& mut self, account_id: AccountId, contract_name: String, function_name: String) ->  bool {
            let caller = self.env().caller();
            assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("grant")));
            if let Some(action) = self.actions.get(&(contract_name, function_name)){
                let a: Action = Action{
                    action_id: action.action_id,
                    action_title: action.action_title.clone(),
                    contract_name: action.contract_name.clone(),
                    function_name: action.function_name.clone(),
                };
                self.actions_auths.insert((account_id, action.action_id), a);
                return true;
           }
           return false;
        }

        #[ink(message)]
        pub fn transfer_owner(
            &mut self,
            to: AccountId,
        ) -> bool {
            assert!(self.owner == self.env().caller());
            self.owner = to;
            true
        }


        #[ink(message)]
        pub fn revoke_permission(& mut self,account_id: AccountId,contract_name: String, function_name: String) -> bool {
            let caller = self.env().caller();
            assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("grant")));
            if let Some(action) = self.actions.get(&(contract_name, function_name)){
                self.actions_auths.take(&(account_id, action.action_id));
                return true;
           }
           return false;
        }

        
        #[ink(message)]
        pub fn register_action(& mut self,contract_name: String, function_name: String, action_title: String) -> bool {
            let caller = self.env().caller();
            assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("register")));
            let action_id = self.action_id;
            self.action_id += 1;
            let action = Action{
                action_id,
                action_title: action_title.clone(),
                contract_name: contract_name.clone(),
                function_name: function_name.clone(),
            };
            self.actions.insert((contract_name, function_name), action);
            true
        }


        #[ink(message)]
        pub fn cancel_action(& mut self,contract_name: String, function_name: String) -> bool {
            let caller = self.env().caller();
            assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("register")));
            self.actions.take(&(contract_name, function_name));
            true
        }

        #[ink(message)]
        pub fn show_actions_by_contract(& self, contract_name: String) -> Vec<Action> {
        
            let mut actions_vec: Vec<Action> = Vec::new();
            for ((cname, _), val) in &self.actions {
                if  *cname == contract_name {
                    let v: Action = Action {
                        action_id: val.action_id,
                        action_title: val.action_title.clone(),
                        contract_name: val.contract_name.clone(),
                        function_name: val.function_name.clone(),
                    };
                    actions_vec.push(v);
                }
            }
            actions_vec
        }

        #[ink(message)]
        pub fn show_actions_by_user(& self, owner: AccountId) -> Vec<Action> {
        
            let mut actions_vec: Vec<Action> = Vec::new();
            for ((account_id, _), val) in &self.actions_auths {
                if *account_id == owner {
                    let v: Action = Action {
                        action_id: val.action_id,
                        action_title: val.action_title.clone(),
                        contract_name: val.contract_name.clone(),
                        function_name: val.function_name.clone(),
                    };
                    actions_vec.push(v);
                }
            }
            actions_vec
        }

        #[ink(message)]
        pub fn get_auth_owner(& self) -> AccountId {
            return self.owner;
        }
    }

    #[cfg(test)]
    mod tests {
        use ink_lang as ink;

        use super::*;
        use ink_env::{
            call,
            test,
        };

        #[ink::test]
        fn test_register_action() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            let r = auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            match r {
                true => ink_env::debug_println("success"),
                false => ink_env::debug_println("failed"),
            }
        }

        #[ink::test]
        fn test_grant_permission() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            let r = auth.grant_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r {
                true => ink_env::debug_println("grant success"),
                false => ink_env::debug_println("grant failed"),
            }
        }

        #[ink::test]
        fn test_grant_permission2() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            auth.register_action("auth".to_string(), "grant".to_string(), "grant".to_string());
            auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            auth.grant_permission(accounts.alice, "auth".to_string(), "grant".to_string());
            auth.transfer_owner(accounts.bob);
            let r = auth.grant_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r {
                true => ink_env::debug_println("grant2 success"),
                false => ink_env::debug_println("grant2 failed"),
            }
        }

        #[ink::test]
        fn test_has_permission() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            auth.grant_permission(accounts.bob, "hello".to_string(), "world".to_string());
            let r1 = auth.has_permission(accounts.alice, "hello".to_string(), "world".to_string());
            match r1 {
                false => ink_env::debug_println("except result"),
                true => ink_env::debug_println("not except"),
            }
            let r2 = auth.has_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r2 {
                true => ink_env::debug_println("except result"),
                false => ink_env::debug_println("not except"),
            }
        }

        #[ink::test]
        fn test_revoke_permission() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            auth.grant_permission(accounts.bob, "hello".to_string(), "world".to_string());
            let r1 = auth.has_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r1 {
                true => ink_env::debug_println("except result"),
                false => ink_env::debug_println("not except"),
            }
            auth.revoke_permission(accounts.bob, "hello".to_string(), "world".to_string());
            let r2 = auth.has_permission(accounts.bob, "hello".to_string(), "world".to_string());
            match r2 {
                false => ink_env::debug_println("except result"),
                true => ink_env::debug_println("not except"),
            }
        }

        #[ink::test]
        fn test_show_actions_by_contract() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut auth = Auth::new(accounts.alice);
            auth.register_action("hello".to_string(), "world".to_string(), "access".to_string());
            let result = auth.show_actions_by_contract("hello".to_string());

            for r in result.iter() {
                ink_env::debug_println(&r.action_title.to_string());
            }
        } 
    }

}
