#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

extern crate alloc;

mod types;

use types::*;

#[ink::contract]
mod vote {
    use super::*;
    use alloc::collections::BTreeMap;
    use scale::{Encode, Decode};

    type  Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Encode, Decode, scale_info::TypeInfo, PartialEq)]
     pub enum Error{
        AlreayRegistered,
        NotRegisteredToVote,
        VoterAccreditationFailure,
        PartyAlreadyRegistered,
        PartyIsNotRegistered,
        NotPermitted,
        NotOpenForVoting,
        PartyRegistrationLimit,
        UableToCollateElectionResults  
    }
   
    #[ink(storage)]
    pub struct Vote {
        nonce: u32,
        open_for_voting: bool,
        voters_register: BTreeMap<AccountId, PermanentVotersCard>,
        party_register: BTreeMap<Hash, Party>,
        admin: AccountId,
        ballot_box: BTreeMap<BallotId, BallotPaper>
    }

    impl Vote {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
           Self {
            nonce: 1,
            open_for_voting: Default::default(),
            voters_register: Default::default(),
            party_register: Default::default(),
            admin,
            ballot_box: Default::default()
           }
        }

        #[ink(message)]
        pub fn register_to_vote(
            &mut self,
            nin: Hash,
            state: Hash,
            local_govt: Hash,
            ward: Hash
            ) -> Result<()> {

            let caller = self.env().caller();
            let voter_pvc = PermanentVotersCard {
                nin,
                state,
                local_govt,
                ward
            };

            if self.is_registered(caller){
                return Err(Error::AlreayRegistered);
            }
            self.voters_register.insert(caller, voter_pvc);
            
            Ok(())
        }

        #[ink(message)]
        pub fn register_party_for_election(
            &mut self, 
            party_name: Hash, 
            party_candidate: Hash
        ) -> Result<()> {
            let caller = self.env().caller();
          
            let party = Party{
                party_name,
                party_candidate
            } ;
            
            if !(caller == self.admin){
                return Err(Error::NotPermitted);
            }

            if self.party_register.len() == MAX_PARTY_NUM as usize{
                return Err(Error::PartyRegistrationLimit);
            }

            if self.is_registered_party(party_name){
                return Err(Error::PartyAlreadyRegistered);
            }

            self.party_register.insert(party.party_name, party);
          
            Ok(())
        }

        #[ink(message)]
        pub fn cast_vote(
            &mut self,
            nin: Hash, 
            state: Hash,
            local_govt: Hash,
            ward: Hash, 
            party: Hash
        ) -> Result<()> {
                let caller = self.env().caller();

                if !self.is_registered(caller){
                    return Err(Error::NotRegisteredToVote);
                }

                let voter_pvc = self.voters_register.get(&caller).unwrap();

                if !Self::voter_is_accredited (
                    voter_pvc, 
                    state, 
                    local_govt, 
                    ward,
                    nin
                ) {
                    return Err(Error::VoterAccreditationFailure);
                }
                    
                if !self.is_registered_party(party){
                    return Err(Error::PartyIsNotRegistered);
                }

                if !self.open_for_voting{
                    return Err(Error::NotOpenForVoting);
                }

                let party = self.party_register.get(&party).unwrap();
                let ballot_id = self.nonce;

                let ballot_paper = BallotPaper{
                    ballot_id,
                    vote_choice: party.clone(),
                    state
                };

                self.ballot_box.insert(ballot_id, ballot_paper);
                self.nonce += 1;

                Ok(())
            }

            #[ink(message)]
            pub fn total_vote_count(&self) -> VoteCount {
                self.ballot_box
                .values()
                .cloned()
                .collect::<Vec<BallotPaper>>()
                .len() as VoteCount
            }

            #[ink(message)]
            pub fn party_vote_count_for_state(&self, party: Hash, state: Hash) -> VoteCount { 
                if self.is_registered_party(party){
                    return self.state_vote_count(party, state);
                }
        
                0  
            }

            #[ink(message)]
            pub fn party_vote_count(&self, party: Hash) -> VoteCount{
        
                let ballot_paper_filter = |p: &BallotPaper| -> bool {
                    p.vote_choice.party_name == party 
                };

                self.ballot_box
                .values()
                .cloned()
                .filter(|val|ballot_paper_filter(val))
                .map(|v| v.clone())
                .collect::<Vec<BallotPaper>>()
                .len() as VoteCount
            }


            #[ink(message)]
            pub fn start_election(&mut self) -> Result<()> {
                let caller = self.env().caller();
                if !(caller == self.admin){
                    return Err(Error::NotPermitted);
                }
                self.open_for_voting = true;

                Ok(())
            }

            #[ink(message)]
            pub fn end_election(&mut self) -> Result<()> {
                let caller = self.env().caller();

                if !(caller == self.admin) {
                    return Err(Error::NotPermitted);
                }
                self.open_for_voting = false;

                Ok(())
            }

            #[ink(message)]
            pub fn collate_election_results(&mut self) -> Result<Vec<ElectionResult>>{
                let caller = self.env().caller();

                if !(caller == self.admin) {
                    return Err(Error::NotPermitted);
                }

                if self.open_for_voting{
                    return Err(Error::UableToCollateElectionResults);
                }

                let mut results = Vec::new();

                let mut parties = self.party_register
                .values()
                .cloned()
                .collect::<Vec<Party>>();

                let collate_result = |ballot_paper: &BallotPaper| -> ElectionResult{
                    let party = ballot_paper.vote_choice.party_name;
                    let state = ballot_paper.state;
                    let vote_count = self.state_vote_count(party, state);

                    (party, state, vote_count)
                };

                while let Some(party) = parties.pop(){
                    let result = self.ballot_box
                    .values()
                    .filter(|b| b.vote_choice.party_name == party.party_name )
                    .map(|b| collate_result(&b))
                    .collect::<Vec<ElectionResult>>();

                    results.push(result);
                }

                let results = results.concat();
                Ok(results)

            }

            fn is_registered(&self, voter: AccountId) -> bool {
                self.voters_register.contains_key(&voter)
            }

            fn is_registered_party(&self, party: Hash) -> bool {
                self.party_register.contains_key(&party)
            }
        
            fn voter_is_accredited(
                voter_pvc: &PermanentVotersCard,
                nin: Hash, 
                state: Hash, 
                local_govt: Hash,
                 ward: Hash
                ) -> bool {
                    (voter_pvc.state == state) && 
                    (voter_pvc.local_govt == local_govt) &&
                    (voter_pvc.ward == ward) && 
                    (voter_pvc.nin == nin)
                }
        
            fn state_vote_count(&self, party: Hash, state: Hash) -> VoteCount{
                let ballot_paper_filter = |p: &BallotPaper| -> bool {
                    p.state == state && p.vote_choice.party_name == party
                };

                self.ballot_box
                .values()
                .filter(|val| ballot_paper_filter(val))
                .map(|v| v.clone())
                .collect::<Vec<BallotPaper>>()
                .len() as VoteCount
            }

           
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{hash::{Blake2x256, HashOutput}, AccountId, Hash};

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn generate_hash(bytes: &[u8]) -> Hash {
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            <Blake2x256 as ink_env::hash::CryptoHash>::hash(bytes, &mut output);
    
            Hash::from(output)
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let vote = Vote::default();
            assert_eq!(vote.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut vote = Vote::new(false);
            assert_eq!(vote.get(), false);
            vote.flip();
            assert_eq!(vote.get(), true);
        }
    }


    

