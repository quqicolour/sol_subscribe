use anchor_lang::prelude::*;

#[event]
pub struct BuyEvent {
    pub subscribe_type: u8,
    pub buyer: Pubkey,
    pub pay: u64
}