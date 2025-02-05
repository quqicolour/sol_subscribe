use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Management {
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub fee_receiver: Pubkey
}

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