#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::org::OrgManager;

#[ink::contract]
mod org {
    use alloc::string::String;
    use ink_prelude::collections::BTreeMap;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };


    use auth::Auth;
    #[ink(storage)]
    pub struct OrgManager {

        moderators: StorageHashMap<AccountId, String>,
        members: StorageHashMap<AccountId, String>,
        applying_members: StorageHashMap<AccountId, String>,
        owner: AccountId,
        org_id:u64,
        can_free_add_member: bool,
        is_member: bool,
        is_moderator:bool,
        is_owner:bool,
        auth_contract_address:AccountId,
    }


    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        DaoModeratorExistAlready,
        DaoMemberExistAlready,
        DaoMemberNotExist,
        DaoModeratorNotExist,
    }

    #[ink(event)]
    pub struct AddDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct RemoveDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id:u64,
    }


    #[ink(event)]
    pub struct AddDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct RemoveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }



    #[ink(event)]
    pub struct ApplyDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct ApproveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
        #[ink(topic)]
        approver: AccountId,
    }


    impl OrgManager {

        #[ink(constructor)]
        pub fn new(_owner: AccountId,org_id:u64,auth_contract_address: AccountId) -> Self {
            Self {
                org_id:org_id,
                owner:_owner,
                moderators: StorageHashMap::default(),
                members: StorageHashMap::default(),
                applying_members: StorageHashMap::default(),
                auth_contract_address:auth_contract_address,
                can_free_add_member: false,
                is_member: false,
                is_moderator:false,
                is_owner:false,
            }
        }


        #[ink(message)]
        pub fn get_dao_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_can_free_add_member(&self) -> bool {
            self.can_free_add_member
        }

        #[ink(message)]
        pub fn set_can_free_add_member(&mut self,can_free_add_member:bool) -> bool {
            self.can_free_add_member = can_free_add_member;
            self.can_free_add_member
        }

        pub fn get_auth_by_address(&self, address:AccountId) -> Auth {
            let  auth_instance: Auth = ink_env::call::FromAccountId::from_account_id(address);
            auth_instance

        }


        #[ink(message)]
        pub fn get_orgid(&self) -> u64 {
            self.org_id
        }


        #[ink(message)]
        pub fn get_dao_moderator_list(&self) -> alloc::vec::Vec<AccountId> {
            self.moderators.keys();
            let mut v:alloc::vec::Vec<AccountId> = alloc::vec::Vec::new();
            for key in self.moderators.keys() {
                v.push(*key)
            }
            v

        }


        #[ink(message)]
        pub fn get_dao_members_list(&self) -> alloc::vec::Vec<AccountId> {
            self.members.keys();
            let mut v:alloc::vec::Vec<AccountId> = alloc::vec::Vec::new();
            for key in self.members.keys() {
                v.push(*key)
            }
            v
        }


        #[ink(message)]
        pub fn get_dao_moderator_detail_list(&self) -> alloc::vec::Vec<(AccountId, String)> {
            self.moderators.keys();
            let mut v:alloc::vec::Vec<(AccountId, String)> = alloc::vec::Vec::new();
            for key in self.moderators.keys() {

                let value = self.moderators.get(key).unwrap().clone();

                v.push((*key,value))
            }
            v

        }

        #[ink(message)]
        pub fn get_dao_member_detail_list(&self) -> alloc::vec::Vec<(AccountId, String)> {
            self.members.keys();
            let mut v:alloc::vec::Vec<(AccountId, String)> = alloc::vec::Vec::new();
            for key in self.members.keys() {

                let value = self.members.get(key).unwrap().clone();

                v.push((*key,value))
            }
            v

        }


        #[ink(message)]
        pub fn get_dao_apply_member_detail_list(&self) -> alloc::vec::Vec<(AccountId, String)> {
            self.applying_members.keys();
            let mut v:alloc::vec::Vec<(AccountId, String)> = alloc::vec::Vec::new();
            for key in self.applying_members.keys() {

                let value = self.applying_members.get(key).unwrap().clone();

                v.push((*key,value))
            }
            v

        }


        #[ink(message)]
        pub fn add_dao_moderator(&mut self,name:String,moderator: AccountId) -> bool  {
            let caller = self.env().caller();

            
            if caller != self.owner {
                return false;
            }



            let  mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
           // assert!(self.owner == caller || self._has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(moderator, String::from("vote"), String::from("new"));
            auth_instance.grant_permission(moderator, String::from("vote"), String::from("vote"));


            match self.moderators.insert(moderator,name) {
                Some(_) => { false},
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOModeratorEvent{
                        moderator,
                        org_id,});
                    true
                }
            }
        }


        #[ink(message)]
        pub fn add_dao_moderator_without_grant(&mut self,name:String,moderator: AccountId) -> bool  {
            let caller = self.env().caller();


            if caller != self.owner {
                return false;
            }

            match self.moderators.insert(moderator,name) {
                Some(_) => { false},
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOModeratorEvent{
                        moderator,
                        org_id,});
                    true
                }
            }
        }

        #[ink(message)]
        pub fn add_dao_member(&mut self,name:String,member: AccountId) -> bool {

            let caller = self.env().caller();

            if self.can_free_add_member == false {
                return false
            }

            let  mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
           // assert!(auth_instance.has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(member, String::from("vote"), String::from("vote"));

            match self.members.insert(member,name) {
                Some(_) => { false},
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOMemberEvent{
                        member,
                        org_id,
                    });
                    true
                }
            }

        }


        
        pub fn add_dao_member_private(&mut self,name:String,member: AccountId) -> bool {

            let caller = self.env().caller();
            let  mut auth_instance = self.get_auth_by_address(self.auth_contract_address);
           // assert!(auth_instance.has_permission(caller, String::from("auth"),String::from("grant")));
            auth_instance.grant_permission(member, String::from("vote"), String::from("vote"));

            match self.members.insert(member,name) {
                Some(_) => { false},
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(AddDAOMemberEvent{
                        member,
                        org_id,
                    });
                    true
                }
            }

        }


        #[ink(message)]
        pub fn batch_add_dao_member(&mut self, members:BTreeMap<String, AccountId>) -> bool {
            for (name, accountId) in &members {
                self.add_dao_member(String::from(name),*accountId);
            }
            true
            
        }


        #[ink(message)]
        pub fn remove_dao_moderator(&mut self,member: AccountId) -> bool  {

            let caller = self.env().caller();

            if caller !=  self.owner {
                return false;
            }


            match self.moderators.take(&member) {
                None => { false}
                Some(_) => {
                    let org_id = self.org_id;
                    self.env().emit_event(RemoveDAOModeratorEvent{
                        moderator:member,
                        org_id,
                    });
                    true
                }
            }


        }

        #[ink(message)]
        pub fn remove_dao_member(&mut self, member: AccountId) -> bool  {


            match self.members.take(&member) {
                None => { false}
                Some(_) => {
                    let org_id = self.org_id;
                    self.env().emit_event(RemoveDAOMemberEvent{
                        member:member,
                        org_id:org_id,
                    });
                    true
                }
            }

        }


        // #[ink(message)]
        // pub fn resign(&mut self,member: AccountId) -> bool  {


        //     if self.members.contains_key(&member) {
        //         self.members.take(&member);
        //         return true;
        //     };

        //     if self.moderators.contains_key(&member) {
        //         self.moderators.take(&member);
        //         return true;
        //     };
        //     return false;
        // }

        #[ink(message)]
        pub fn resign_member(&mut self,member: AccountId) -> bool  {


            if self.members.contains_key(&member) {
                self.members.take(&member);
                return true;
            };

            return false;
        }


        #[ink(message)]
        pub fn resign_moderator(&mut self,moderator: AccountId) -> bool  {

            if self.moderators.contains_key(&moderator) {
                self.moderators.take(&moderator);
                return true;
            };
            return false;
        }



        #[ink(message)]
        pub fn who_am_i(&mut self) -> (bool,bool,bool)  {

            let caller = self.env().caller();
           
            if self.members.contains_key(&caller) {
                self.is_member =  true;
            } else {
                self.is_member =  false;
            }

            if self.moderators.contains_key(&caller) {
                self.is_moderator =  true;
            } else {
                self.is_moderator =  false;
            }
            
            if caller == self.owner {
                self.is_owner = true;
            } else {
                self.is_owner = false;
            }

            return (self.is_member,self.is_moderator,self.is_owner)
            
        }


        #[ink(message)]
        pub fn transfer_ownership(&mut self,new_owner: AccountId) -> bool  {

            let caller = self.env().caller();

            // only owner can transfer the ownership of the org
            if caller != self.owner {
                return false;
            }

            self.owner = new_owner;
            return true;
        }
        #[ink(message)]
        pub fn apply_member(&mut self,name:String,member: AccountId) -> bool {
            match self.applying_members.insert(member,name) {
                Some(_) => { false},
                None => {
                    let org_id = self.org_id;
                    self.env().emit_event(ApplyDAOMemberEvent{
                        member,
                        org_id,
                    });
                    true
                }
            }

        }


   
        pub fn check_authority(&self, caller:AccountId) -> bool {

                let moderator_list = self.get_dao_moderator_list();

                if caller == self.owner {
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
        pub fn approve_member(&mut self,name:String,member: AccountId) -> bool {

            let caller = self.env().caller();

            let can_operate = self.check_authority(caller);

            if can_operate == false {
                return false;
            }

            if self.applying_members.contains_key(&member) {
                let caller_new = self.env().caller();
                self.add_dao_member_private(name,member);
                self.applying_members.take(&member);
                let org_id = self.org_id;

                self.env().emit_event(ApproveDAOMemberEvent{
                    member,
                    org_id,
                    approver:caller_new,
                });

                return true;
            };
            return false;

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
        fn new_org_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);

            assert_eq!(org_manager.creator, accounts.alice);
            assert_eq!(org_manager.org_id, 1);
        }

        #[ink::test]
        fn add_member_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            let mut member = org_manager.get_dao_members_list()[0];
            assert_eq!(member, accounts.bob);

        }

        #[ink::test]
        fn add_moderator_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name,accounts.bob);
            let mut member = org_manager.get_dao_moderator_list()[0];
            assert_eq!(member, accounts.bob);

        }

        #[ink::test]
        fn remove_moderator_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_moderator(bob_name,accounts.bob);
            org_manager.remove_dao_moderator(accounts.bob);

            let mut members = org_manager.get_dao_moderator_list();
            assert_eq!(members.len(), 0);

        }


        #[ink::test]
        fn remove_members_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            org_manager.remove_dao_member(accounts.bob);
            let mut members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 0);
        }

        #[ink::test]
        fn resign_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut org_manager = OrgManager::new(accounts.alice,1);
            let bob_name = String::from("bob");
            org_manager.add_dao_member(bob_name,accounts.bob);
            let eve_name = String::from("eve");
            org_manager.add_dao_member(eve_name,accounts.eve);
            let mut members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 2);
            org_manager.resign(accounts.bob);
            members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 1);
            org_manager.resign(accounts.eve);
            members = org_manager.get_dao_members_list();
            assert_eq!(members.len(), 0);
        }
    }
}
