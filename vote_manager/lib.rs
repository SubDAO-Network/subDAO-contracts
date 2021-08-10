#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::vote_manager::VoteManager;

#[ink::contract]
mod vote_manager {

    use alloc::format;
    // use alloc::vec;
    use alloc::vec::Vec;
    use alloc::string::String;
    use vault::VaultManager;
    use auth::Auth;

    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    type VoteId = u64;
    type ChoiceId = u32;

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Clone,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Choice {
        choice_id: ChoiceId,
        content: String,
        yea: u64,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
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
    pub struct Vote {
        vote_id: VoteId,
        executed: bool,
        title: String,
        desc: String,
        need_trigger: bool,
        start_date: u64,
        vote_time: u64,
        support_require_num: u64,
        min_require_num: u64,
        support_num: u64,
        erc20_address: AccountId,
        to_address: AccountId,
        value: u64,
        choice_index_lo: u32,
        choice_index_ho: u32,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
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
    pub struct DisplayVote {
        vote_id: VoteId,
        executed: bool,
        title: String,
        desc: String,
        need_trigger: bool,
        start_date: u64,
        vote_time: u64,
        support_require_num: u64,
        min_require_num: u64,
        support_num: u64,
        choices: String,
        erc20_address: AccountId,
        to_address: AccountId,
        transfer_value: u64,
    }


    #[ink(storage)]
    pub struct VoteManager {
        vault: VaultManager,
        auth: Auth,
        votes_length: u64,
        votes: StorageHashMap<VoteId, Vote>,
        voters: StorageHashMap<(VoteId, AccountId), ChoiceId>,
        choices: StorageVec<Choice>,
        choices_num: u32,
    }

    #[ink(event)]
    pub struct StartVote {
        #[ink(topic)]
        vote_id: VoteId,

        #[ink(topic)]
        creator: AccountId,
    }

    #[ink(event)]
    pub struct CastVote {
        #[ink(topic)]
        vote_id: VoteId,

        #[ink(topic)]
        voter: AccountId,

        support_choice: ChoiceId,
    }

    #[ink(event)]
    pub struct ExecuteVote {
        #[ink(topic)]
        vote_id: VoteId,
    }

    impl VoteManager {

        #[ink(constructor)]
        pub fn new(vault_address: AccountId, auth_address: AccountId) -> Self {
            let vault_instance = ink_env::call::FromAccountId::from_account_id(vault_address);
            let auth_instance = ink_env::call::FromAccountId::from_account_id(auth_address);
            Self { 
                vault: vault_instance,
                auth: auth_instance,
                votes_length: 0,
                votes: StorageHashMap::default(),
                voters: StorageHashMap::default(),
                choices: StorageVec::default(),
                choices_num: 0,
            }
        }

        #[ink(message)]
        pub fn new_vote(&mut self, title: String, desc: String, vote_time: u64, support_require_num: u64, min_require_num: u64, choices: String) -> u64 {
            let caller = self.env().caller();
            assert!(self.auth.has_permission(caller,String::from("vote"),String::from("new")));
            let vote_id = self.votes_length.clone();
            self.votes_length += 1;
            let start_date: u64 = self.env().block_timestamp();
            let vec: Vec<&str> = choices.split("|").collect();
            let vote = Vote{
                vote_id: vote_id,
                executed: false,
                title,
                desc,
                start_date: start_date,
                vote_time,
                need_trigger: false,
                support_require_num,
                min_require_num,
                support_num: 0,
                erc20_address: AccountId::default(), 
                to_address: AccountId::default(),
                value: 0,
                choice_index_lo: self.choices_num,
                choice_index_ho: self.choices_num + vec.len() as u32,
            };
            let choices_len = vec.len() as u32;
            if choices_len == 0 {
                return 0;
            }
            self.choices_num += choices_len;
            let mut index = 0;
            for choice_content in vec.iter() {
                self.choices.push(Choice{
                    choice_id: index,
                    content: String::from(*choice_content),
                    yea: 0,
                });
                index += 1;
            }
            self.votes.insert(vote_id, vote);
            self.env().emit_event(StartVote{
                vote_id,
                creator: self.env().caller(),
            });
            vote_id
        }

        #[ink(message)]
        pub fn new_vote_with_transfer(&mut self, title: String, desc: String, vote_time: u64, support_require_num: u64, min_require_num: u64, choices: String, erc20_address:AccountId, to_address:AccountId, value:u64) -> u64 {
            let caller = self.env().caller();
            assert!(self.auth.has_permission(caller,String::from("vote"),String::from("new")));
            let vote_id = self.votes_length.clone();
            self.votes_length += 1;
            let start_date: u64 = self.env().block_timestamp();
            let vec: Vec<&str> = choices.split("|").collect();
            let vote = Vote{
                vote_id: vote_id,
                executed: false,
                title,
                desc,
                start_date: start_date,
                vote_time,
                need_trigger: true,
                support_require_num,
                min_require_num,
                support_num: 0,
                erc20_address,
                to_address,
                value,
                choice_index_lo: self.choices_num,
                choice_index_ho: self.choices_num + vec.len() as u32,
            };
            let choices_len = vec.len() as u32;
            if choices_len == 0 {
                return 0;
            }
            self.choices_num += choices_len;
            let mut index = 0;
            for choice_content in vec.iter() {
                self.choices.push(Choice{
                    choice_id: index,
                    content: String::from(*choice_content),
                    yea: 0,
                });
                index += 1;
            }
            self.votes.insert(vote_id, vote);
            self.env().emit_event(StartVote{
                vote_id,
                creator: self.env().caller(),
            });
            vote_id
        }

       
        #[ink(message)]
        pub fn execute(&mut self, vote_id: VoteId) -> bool {
            assert!(self.vote_exists(vote_id));
            let vote = self.votes.get(&vote_id).unwrap(); 
            if self.can_execute(&vote) {
                let mut vote = self.votes.get_mut(&vote_id).unwrap(); 
                vote.executed = true;
                let result = self.vault.withdraw(vote.erc20_address, vote.to_address, vote.value);
                assert!(result);
                self.env().emit_event(ExecuteVote{
                    vote_id,
                });
                result
            } else {
                true
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, vote_id: VoteId, support_choice: u32, voter: AccountId) -> bool {
            if !self.vote_exists(vote_id) {
                return false;
            }
            let caller = self.env().caller();
            if !self.auth.has_permission(caller,String::from("vote"),String::from("vote")) {
                return false;
            }
            if let Some(vote) = self.votes.get_mut(&vote_id) {
                if support_choice > vote.choice_index_ho - vote.choice_index_lo {
                    return false;
                }
                // has voted
                if let Some(_choice_id) = self.voters.get(&(vote_id, voter)) {
                    // if *choice_id != support_choice {
                    //     let choice_vec_index = vote.choice_index_lo + *choice_id;
                    //     let choices = &mut self.choices;
                    //     choices.get_mut(choice_vec_index).unwrap().yea -= 1;
                    //     vote.support_num -= 1;
                    // }
                    return false;
                } 
                let choices = &mut self.choices;
                let choice_vec_index = vote.choice_index_lo + support_choice;
                let voter_choice = choices.get_mut(choice_vec_index).unwrap();
                voter_choice.yea += 1;
                // record voter choice id
                self.voters.insert((vote_id, voter), support_choice);    
                vote.support_num += 1;
                self.env().emit_event(CastVote{
                    vote_id,
                    voter: self.env().caller(), 
                    support_choice,
                });
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn query_voter_vote_one(&self, vote_id: VoteId, voter: AccountId) -> bool {
            assert!(self.vote_exists(vote_id));
            return self.vote_has_been_voted(vote_id, voter);
        }

        #[ink(message)]
        pub fn query_one_vote(&self, vote_id: VoteId) -> DisplayVote {
            assert!(self.vote_exists(vote_id));
            let vote = self.votes.get(&vote_id).unwrap(); 
            let display_vote = self.convert_vote_to_displayvote(&vote); 
            display_vote
        }

        #[ink(message)]
        pub fn query_all_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, vote) in &self.votes {
                let vote = self.convert_vote_to_displayvote(&vote);
                v.push(vote);
            }
            return v;
        }

        #[ink(message)]
        pub fn query_history_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if !self.is_vote_need_trigger(&val) || self.is_vote_executed(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }

        #[ink(message)]
        pub fn query_active_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if self.is_vote_open(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }

        #[ink(message)]
        pub fn query_pending_vote(&self) -> alloc::vec::Vec<DisplayVote> {
            let mut v: alloc::vec::Vec<DisplayVote> = alloc::vec::Vec::new();
            for (_, val) in &self.votes {
                if self.is_vote_wait(&val) {
                    let vote = self.convert_vote_to_displayvote(&val);
                    v.push(vote);
                }
            }
            return v;
        }
 
        fn convert_vote_to_displayvote(&self, vote: &Vote) -> DisplayVote {
            let mut choices = Vec::new();
            let mut index = 0;
            let source_choices = &self.choices;
            for choice in source_choices.iter() {
                if index >= vote.choice_index_lo && index < vote.choice_index_ho {
                    let s = format!("{0}:{1}", choice.content.clone(), choice.yea);
                    choices.push(s);
                }
                index += 1;
            }
            let choices_content = choices.join("|"); 
            let display_vote = DisplayVote{
                vote_id: vote.vote_id,
                executed: vote.executed,
                title: vote.title.clone(),
                desc: vote.desc.clone(),
                start_date: vote.start_date,
                vote_time: vote.vote_time,
                need_trigger: vote.need_trigger,
                support_require_num: vote.support_require_num,
                min_require_num: vote.min_require_num,
                support_num: vote.support_num,
                choices: choices_content,
                erc20_address: vote.erc20_address,
                to_address: vote.to_address,
                transfer_value: vote.value,
            };
            display_vote
        }

        fn vote_exists(&self, vote_id: u64) -> bool {
            return vote_id < self.votes_length;
        }

        fn vote_has_been_voted(&self, vote_id: VoteId, voter: AccountId) -> bool {
            let result = match self.voters.get(&(vote_id, voter)) {
                None => false,
                Some(_) => true, 
            };
            result
        }

        fn is_vote_open(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() < vote.start_date + vote.vote_time;
        }

        fn is_vote_wait(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() > vote.start_date + vote.vote_time && vote.need_trigger && !vote.executed;
        }

        fn is_vote_executed(&self, vote: &Vote) -> bool {
            return vote.executed;
        }

        fn is_vote_need_trigger(&self, vote: &Vote) -> bool {
            return vote.need_trigger;
        }

        fn is_vote_finished(&self, vote: &Vote) -> bool {
            return self.env().block_timestamp() < vote.start_date + vote.vote_time;
        }

        fn can_execute(&self, vote: &Vote) -> bool {
            if !vote.need_trigger {
                return false;
            }
            if vote.executed {
                return false;
            }
            if self.is_vote_open(&vote) {
                return false;
            }
            if vote.support_num < vote.min_require_num {
                return false;
            }
            if vote.support_num == 0 {
                return false;
            }
            let mut index = 0;
            let choices = &self.choices;
            for choice in choices.iter() {
                if index >= vote.choice_index_lo && index < vote.choice_index_ho {
                    if choice.yea >= vote.support_require_num {
                        return true;
                    }
                }
                index += 1;
            }
            return false;
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
        fn test_split() {

            let choices = "A,B,C".to_string();
            let split = choices.split(",");
            ink_env::debug_println("hello");
            for s in split {
                ink_env::debug_println(&s);
            }
        }

        #[ink::test]
        fn test_split_with_vec() {
            let choices = "A,B,C".to_string();
            let vec: Vec<&str> = choices.split(",").collect();
            let i:u32 = 1;
            let length = i + vec.len() as u32;
            assert!(length == 4);
            for s in vec{
                ink_env::debug_println(&s);
            }
        }

        #[ink::test]
        fn test_calculate() {
            let choice: u64 = 3;
            let support: u64 = 5;
            let t : u64 = choice * 1000 / support; 
            ink_env::debug_println(t.to_string().as_str());
        }

        #[ink::test]
        fn new_vote_manager() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // after update votemanager need an vault_address to be initialized.
            // use alice address to replace here.
            let vote_manager = VoteManager::new(accounts.alice);

            assert_eq!(vote_manager.votes_length, 0);
        }

        #[ink::test]
        fn full_test() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut vote_manager = VoteManager::new(accounts.alice);
            
            let r = vote_manager.new_vote("hello".to_string(), "hello world".to_string(), 100, 1, 0, "A|B|C".to_string());
            assert_eq!(r, 0);

            let vec1 = vote_manager.query_all_vote();
            for elem in vec1.iter() {
                let debug_info = format!("choice id: {}", &elem.choices);
                ink_env::debug_println( &debug_info );
            }

            vote_manager.vote(0, 2, accounts.alice);

            let vec2 = vote_manager.query_all_vote();
            for elem in vec2.iter() {
                let debug_info = format!("choice id: {}", &elem.choices);
                ink_env::debug_println( &debug_info );
            }


            vote_manager.vote(0, 1, accounts.alice);

            let vec3 = vote_manager.query_all_vote();
            for elem in vec3.iter() {
                let debug_info = format!("choice id: {}", &elem.choices);
                ink_env::debug_println( &debug_info );
            }
        }

        #[ink::test]
        fn vote_has_voted_test() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut vote_manager = VoteManager::new(accounts.alice);
            
            let r = vote_manager.new_vote("hello".to_string(), "hello world".to_string(), 100, 1, 0, "A|B|C".to_string());
            assert_eq!(r, 0);

            let has_voted = vote_manager.query_voter_vote_one(0, accounts.alice);
            assert_eq!(has_voted, false);

            vote_manager.vote(0, 2, accounts.alice);

            let has_voted = vote_manager.query_voter_vote_one(0, accounts.alice);
            assert_eq!(has_voted, true);

        } 
    }
}
