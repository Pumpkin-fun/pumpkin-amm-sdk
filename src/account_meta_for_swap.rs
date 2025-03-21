use anchor_lang::prelude::{AccountMeta, Pubkey};
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use anchor_spl::associated_token::ID as ASSOCIATED_TOKEN_PROGRAM_ID;
use anchor_spl::token::ID as TOKEN_PROGRAM_ID;

#[derive(Copy, Clone, Debug)]
pub struct SwapSolForToken {
    pub global_config: Pubkey,
    pub bonding_curve: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve_token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub user: Pubkey,
}

impl From<SwapSolForToken> for Vec<AccountMeta> {
    fn from(accounts: SwapSolForToken) -> Self {
        vec![
            AccountMeta::new(accounts.global_config, false),
            AccountMeta::new(accounts.bonding_curve, false),
            AccountMeta::new_readonly(accounts.mint, false),
            AccountMeta::new(accounts.bonding_curve_token_vault, false),
            AccountMeta::new(accounts.sol_vault, false),
            AccountMeta::new(accounts.user_token_account, false),
            AccountMeta::new(accounts.user, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SwapTokenForSol {
    pub global_config: Pubkey,
    pub bonding_curve: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve_token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub index_fund_recipient: Pubkey,
    pub index_fund_recipient_token_account: Pubkey,
    pub user_token_account: Pubkey,
    pub user: Pubkey,
}

impl From<SwapTokenForSol> for Vec<AccountMeta> {
    fn from(accounts: SwapTokenForSol) -> Self {
        vec![
            AccountMeta::new_readonly(accounts.global_config, false),
            AccountMeta::new(accounts.bonding_curve, false),
            AccountMeta::new(accounts.mint, false),
            AccountMeta::new(accounts.bonding_curve_token_vault, false),
            AccountMeta::new(accounts.sol_vault, false),
            AccountMeta::new_readonly(accounts.index_fund_recipient, false),
            AccountMeta::new(accounts.index_fund_recipient_token_account, false),
            AccountMeta::new(accounts.user_token_account, false),
            AccountMeta::new(accounts.user, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ]
    }
}
