use solana_program::{
    clock::{UnixTimestamp}, 
    pubkey::{Pubkey, PUBKEY_BYTES},
    program_pack::{Pack,Sealed},
    program_error::ProgramError,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};



pub const TOKEN : &str = "";

pub const STAKED_TOKEN : &str = "";


#[derive(Clone, Debug, PartialEq)]
pub struct StakedToken {

    pub token : Pubkey,

    pub staked_token : Pubkey, 

    pub staked_amount : u64,

    pub rate : u16,

    pub reward : u64, 

    pub date_staked : UnixTimestamp, 

    pub date_last_updated : UnixTimestamp, 

    pub date_unstaked : UnixTimestamp, 

}


impl Sealed for StakedToken{}

const STAKED_TOKEN_DATA_SIZE : usize = PUBKEY_BYTES + PUBKEY_BYTES + 8 + 2 + 8 + 8 + 8 + 8;



impl Pack for StakedToken {

    const LEN: usize = STAKED_TOKEN_DATA_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {

        let output = array_mut_ref![dst, 0, STAKED_TOKEN_DATA_SIZE];
       
        let (token, staked_token, staked_amount, rate, reward, 
        date_staked, date_last_updated, date_unstaked) = 
        mut_array_refs![ output,PUBKEY_BYTES,PUBKEY_BYTES,8,2,8,8,8,8];

        token.copy_from_slice(self.token.as_ref());
        staked_token.copy_from_slice(self.token.as_ref());
        
        *staked_amount = self.staked_amount.to_le_bytes();
        *rate = self.rate.to_le_bytes();
        *reward = self.reward.to_le_bytes();
        *date_staked = self.date_staked.to_le_bytes();
        *date_unstaked = self.date_unstaked.to_le_bytes();
        *date_last_updated = self.date_last_updated.to_le_bytes();
        
    }


    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
   
        let input = array_ref![src, 0, STAKED_TOKEN_DATA_SIZE];
       
        let (token, staked_token, staked_amount, rate, reward, 
        date_staked, date_last_updated, date_unstaked) = 
         
        array_refs![input, PUBKEY_BYTES,PUBKEY_BYTES,8,2,8,8,8,8];

       
        let token = Pubkey::new_from_array(*token);
        let staked_token = Pubkey::new_from_array(*staked_token);
       

        let staked_amount = u64::from_le_bytes(*staked_amount);
        let rate = u16::from_le_bytes(*rate);
        let reward = u64::from_le_bytes(*reward);
    
        let date_staked = i64::from_le_bytes(*date_staked);
        let date_unstaked = i64::from_le_bytes(*date_unstaked);
        let date_last_updated = i64::from_le_bytes(*date_last_updated);
        
       
        Ok( 

            StakedToken{

                token : token,
                staked_token : staked_token,
                staked_amount : staked_amount,
                rate : rate, 
                reward : reward,
                date_staked : date_staked,
                date_last_updated : date_last_updated,
                date_unstaked : date_unstaked, 

            }
        )

    }
}