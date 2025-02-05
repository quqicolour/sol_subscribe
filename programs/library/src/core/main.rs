use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer as SolTransfer};

use crate::library::error::ErrorCode;
use crate::library::event::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ProductType {
    pub subscribe_type: u8,
    pub fee: u64,
    pub valid_time: u64,
}

#[account]
#[derive(InitSpace)]
pub struct MappingProductType {
    #[max_len(50)]
    pub product_type_mapping: Vec<ProductType>,
}
impl MappingProductType {
    pub fn set(&mut self, subscribe_type: u8, fee: u64, valid_time: u64) {
        if let Some(pair) = self
            .product_type_mapping
            .iter_mut()
            .find(|pair| pair.subscribe_type == subscribe_type)
        {
            pair.fee = fee;
            pair.valid_time = valid_time;
        } else {
            self.product_type_mapping.push(ProductType {
                subscribe_type,
                fee,
                valid_time,
            });
        }
    }

    pub fn get(&self, subscribe_type: u8) -> ProductType {
        if let Some(pair) = self
            .product_type_mapping
            .iter()
            .find(|pair| pair.subscribe_type == subscribe_type)
            .cloned()
        {
            pair
        } else {
            ProductType {
                subscribe_type: 0,
                fee: u64::MAX,
                valid_time: u64::MAX,
            }
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct UserInfo {
    pub user: Pubkey,
    pub last_buy_time: u64,
    pub end_time: u64,
}

#[account]
#[derive(InitSpace)]
pub struct MappingUserInfo {
    #[max_len(1)]
    pub user_info_mapping: Vec<UserInfo>,
}
impl MappingUserInfo {
    pub fn set(&mut self, user: Pubkey, last_buy_time: u64, end_time: u64) {
        if let Some(pair) = self
            .user_info_mapping
            .iter_mut()
            .find(|pair| pair.user == user)
        {
            pair.last_buy_time = last_buy_time;
            pair.end_time = end_time;
        } else {
            self.user_info_mapping.push(UserInfo {
                user,
                last_buy_time,
                end_time,
            });
        }
    }

    pub fn get(&self, user: Pubkey) -> UserInfo {
        if let Some(pair) = self
            .user_info_mapping
            .iter()
            .find(|pair| pair.user == user)
            .cloned()
        {
            pair
        } else {
            UserInfo {
                user: Pubkey::default(),
                last_buy_time: 0,
                end_time: 0,
            }
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + Management::INIT_SPACE,
        seeds = [b"subscribe_management".as_ref()],
        bump
    )]
    pub management: Account<'info, Management>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl Initialize<'_> {
    pub fn do_initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        manager: Pubkey,
        fee_receiver: Pubkey
    ) -> Result<()> {
        let management = &mut ctx.accounts.management;
        management.owner = owner;
        management.manager = manager;
        management.fee_receiver = fee_receiver;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChangeManagement<'info> {
    #[account(
        mut,
        seeds = [b"subscribe_management".as_ref()],
        bump,
        constraint = signer.key() == management.owner @ErrorCode::NonOwner
    )]
    pub management: Account<'info, Management>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl ChangeManagement<'_> {
    pub fn do_transfer_owner(ctx: Context<ChangeManagement>, new_owner: Pubkey) -> Result<()> {
        let management = &mut ctx.accounts.management;
        management.owner = new_owner;
        Ok(())
    }

    pub fn do_transfer_manager(ctx: Context<ChangeManagement>, new_manager: Pubkey) -> Result<()> {
        let management = &mut ctx.accounts.management;
        management.manager = new_manager;
        Ok(())
    }

    pub fn do_set_fee_receiver(
        ctx: Context<ChangeManagement>,
        new_fee_receiver: Pubkey,
    ) -> Result<()> {
        let management = &mut ctx.accounts.management;
        management.fee_receiver = new_fee_receiver;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProductType<'info> {
    #[account(
        mut,
        seeds = [b"subscribe_management".as_ref()],
        bump,
        constraint = signer.key() == management.manager @ErrorCode::NonManager
    )]
    pub management: Account<'info, Management>,
    #[account(
        init, 
        payer = signer, 
        space = 8 + MappingProductType::INIT_SPACE,
        seeds = [b"subscribe_type".as_ref()],
        bump
    )]
    pub mapping_product_type: Account<'info, MappingProductType>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl InitProductType<'_> {
    pub fn do_init_product_type(
        ctx: Context<InitProductType>,
        subscribe_type: u8,
        fee: u64,
        valid_time: u64,
    ) -> Result<()> {
        let mapping_product_type = &mut ctx.accounts.mapping_product_type;
        mapping_product_type.set(subscribe_type, fee, valid_time);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetProductType<'info> {
    #[account(
        mut,
        seeds = [b"subscribe_management".as_ref()],
        bump,
        constraint = signer.key() == management.manager @ErrorCode::NonManager
    )]
    pub management: Account<'info, Management>,
    #[account(
        mut,
        seeds = [b"subscribe_type".as_ref()],
        bump
    )]
    pub mapping_product_type: Account<'info, MappingProductType>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl SetProductType<'_> {
    pub fn do_set_product_type(
        ctx: Context<SetProductType>,
        subscribe_type: u8,
        fee: u64,
        valid_time: u64,
    ) -> Result<()> {
        let mapping_product_type = &mut ctx.accounts.mapping_product_type;
        mapping_product_type.set(subscribe_type, fee, valid_time);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitShopping<'info> {
    #[account(
        mut,
        seeds = [b"subscribe_management".as_ref()],
        bump,
        constraint = fee_receiver.key() == management.fee_receiver @ErrorCode::NonFeeReceiver
    )]
    pub management: Account<'info, Management>,
    #[account(
        mut,
        seeds = [b"subscribe_type".as_ref()],
        bump
    )]
    pub mapping_product_type: Account<'info, MappingProductType>,
    #[account(
        init, 
        payer = user, 
        space = 8 + MappingUserInfo::INIT_SPACE,
        seeds = [b"subscribe_user_info".as_ref(), user.key().as_ref()],
        bump
    )]
    pub mapping_user_info: Account<'info, MappingUserInfo>,
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl InitShopping<'_> {
    pub fn do_buy(ctx: Context<InitShopping>, subscribe_type: u8, copies: u8) -> Result<()> {
        let mapping_product_type = &mut ctx.accounts.mapping_product_type;
        let mapping_user_info = &mut ctx.accounts.mapping_user_info;
        let product_type = mapping_product_type.get(subscribe_type);
        let fee: u64 = product_type.fee;
        let valid_time: u64 = product_type.valid_time.checked_mul(copies as u64).ok_or(ErrorCode::MulOverflow)?;

        // fee
        let sender = ctx.accounts.user.to_account_info();
        let fee_receiver = ctx.accounts.fee_receiver.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(
            program_id,
            SolTransfer {
                from: sender.clone(),
                to: fee_receiver,
            },
        );
        transfer(cpi_context, fee)?;

        let sender_key = sender.key();
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;
        msg!("Current timestamp: {}", current_timestamp);
        require!(current_timestamp >0, ErrorCode::InvalidTime);
        //update state
        let valid_end_time: u64 = current_timestamp + valid_time;
        mapping_user_info.set(sender_key, current_timestamp, valid_end_time);
        
        emit!(BuyEvent {
            subscribe_type: subscribe_type,
            buyer: ctx.accounts.user.key(),
            pay: fee
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Shopping<'info> {
    #[account(
        mut,
        seeds = [b"subscribe_management".as_ref()],
        bump,
        constraint = fee_receiver.key() == management.fee_receiver @ErrorCode::NonFeeReceiver
    )]
    pub management: Account<'info, Management>,
    #[account(
        mut,
        seeds = [b"subscribe_type".as_ref()],
        bump
    )]
    pub mapping_product_type: Account<'info, MappingProductType>,
    #[account(
        mut,
        seeds = [b"subscribe_user_info".as_ref(), user.key().as_ref()],
        bump
    )]
    pub mapping_user_info: Account<'info, MappingUserInfo>,
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl Shopping<'_> {
    pub fn do_buy(ctx: Context<Shopping>, subscribe_type: u8, copies: u8) -> Result<()> {
        let mapping_product_type = &mut ctx.accounts.mapping_product_type;
        let mapping_user_info = &mut ctx.accounts.mapping_user_info;
        let product_type = mapping_product_type.get(subscribe_type);
        let fee: u64 = product_type.fee;
        let valid_time: u64 = product_type.valid_time.checked_mul(copies as u64).ok_or(ErrorCode::MulOverflow)?;

        // fee
        let sender = ctx.accounts.user.to_account_info();
        let fee_receiver = ctx.accounts.fee_receiver.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(
            program_id,
            SolTransfer {
                from: sender.clone(),
                to: fee_receiver,
            },
        );
        transfer(cpi_context, fee)?;

        let sender_key = sender.key();
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;
        msg!("Current timestamp: {}", current_timestamp);
        require!(current_timestamp>0, ErrorCode::InvalidTime);

        //update state
        let this_user_info = mapping_user_info.get(sender_key);
        let end_time: u64 = this_user_info.end_time;
        let mut valid_end_time: u64 = 0;
        if end_time < current_timestamp {
            valid_end_time = current_timestamp + valid_time;
        }else {
            valid_end_time = end_time + valid_time;
        }
        mapping_user_info.set(sender_key, current_timestamp, valid_end_time);
        
        emit!(BuyEvent {
            subscribe_type: subscribe_type,
            buyer: ctx.accounts.user.key(),
            pay: fee
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Management {
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub fee_receiver: Pubkey
}
