use anchor_lang::prelude::*;

declare_id!("Cfi1sAC95izrTMi24dg4vNKS22uPtvJd9HptSNztWygL");

pub mod library;
pub mod core;

use crate::core::*;

#[program]
mod xscope {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        manager: Pubkey,
        fee_receiver: Pubkey
    ) -> Result<()> {
        Initialize::do_initialize(
            ctx,
            owner,
            manager,
            fee_receiver
        )
    }

    pub fn transfer_owner(
        ctx: Context<ChangeManagement>,
        new_owner: Pubkey,
    ) -> Result<()> {
            ChangeManagement::do_transfer_owner(
                ctx,
                new_owner
            )
    }

    pub fn transfer_manager(
        ctx: Context<ChangeManagement>,
        new_manager: Pubkey,
    ) -> Result<()> {
        ChangeManagement::do_transfer_manager(
                ctx,
                new_manager
            )
    }

    pub fn set_fee_receiver(ctx: Context<ChangeManagement>, new_fee_receiver: Pubkey) -> Result<()> {
        ChangeManagement::do_set_fee_receiver(
                ctx,
                new_fee_receiver
            )
    }

    pub fn init_product_type(
        ctx: Context<InitProductType>, 
        subscribe_type: u8, 
        fee: u64, 
        valid_time: u64
    ) -> Result<()>{
        InitProductType::do_init_product_type(
            ctx,
            subscribe_type,
            fee,
            valid_time
        )
    }

    pub fn set_product_type(
        ctx: Context<SetProductType>, 
        subscribe_type: u8, 
        fee: u64, 
        valid_time: u64
    ) -> Result<()>{
        SetProductType::do_set_product_type(
            ctx,
            subscribe_type,
            fee,
            valid_time
        )
    }

    pub fn register_buy(ctx: Context<InitShopping>, subscribe_type: u8, copies: u8) -> Result<()> {
        InitShopping::do_buy(
            ctx,
            subscribe_type,
            copies
        )
    }

    pub fn buy(ctx: Context<Shopping>, subscribe_type: u8, copies: u8) -> Result<()> {
        Shopping::do_buy(
            ctx,
            subscribe_type,
            copies
        )
    }
}

