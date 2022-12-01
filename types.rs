use scale::{Encode, Decode};
use  ink_storage::traits::{PackedLayout, SpreadLayout};
use ink_env::Hash;

#[derive(Debug, Clone, Encode, Decode, PackedLayout, SpreadLayout, scale_info::TypeInfo)]
pub struct Party{
    pub party_name: Hash,
    pub party_candidate: Hash,
}

#[derive(Debug, Clone, Encode, Decode, PackedLayout, SpreadLayout, scale_info::TypeInfo)]
pub struct PermanentVotersCard{
    // voter's national idntiification number (NIN)
    pub nin: Hash,
    //voter's state of origin
    pub state: Hash,
    //voter's local government area of origin
    pub local_govt: Hash,
    //voter's ward
    pub ward: Hash
}

#[derive(Debug, Clone, Encode, Decode, PackedLayout, SpreadLayout, scale_info::TypeInfo)]
pub struct BallotPaper{
    pub ballot_id: BallotId,
    pub vote_choice: Party,
    pub state: Hash
}

pub type BallotId = u32;
pub type VoteCount = u32;
pub type ElectionResult = (Hash, Hash, VoteCount);

pub const MAX_PARTY_NUM: u32 = 10;