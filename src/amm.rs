use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;
use anyhow::Result;
use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap,
    SwapAndAccountMetas, SwapParams,
};

use crate::account_meta_for_swap::{SwapSolForToken, SwapTokenForSol};
use crate::accounts::{BondingCurve, GlobalConfig};
use crate::constants::MAX_FEE_BASIS_POINTS;
use crate::pda::{get_global_config_address, get_sol_vault_address};

pub struct PumpkinSwap {
    key: Pubkey,
    program_id: Pubkey,
    state: BondingCurve,
    swap_fee_rate_bps: u64,
    index_fund_recipient: Pubkey,
}

impl Clone for PumpkinSwap {
    fn clone(&self) -> Self {
        PumpkinSwap {
            key: self.key.clone(),
            program_id: self.program_id.clone(),
            state: self.state.clone(),
            swap_fee_rate_bps: self.swap_fee_rate_bps,
            index_fund_recipient: self.index_fund_recipient.clone(),
        }
    }
}

impl Amm for PumpkinSwap {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let state = BondingCurve::try_deserialize(&mut &keyed_account.account.data[..]).unwrap();
        Ok(Self {
            key: keyed_account.key,
            program_id: keyed_account.account.owner,
            state,
            swap_fee_rate_bps: 100, //contract init with 1%,
            index_fund_recipient: Pubkey::default(),
        })
    }

    fn label(&self) -> String {
        String::from("Pumpkin swap")
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn key(&self) -> Pubkey {
        self.key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        //test_harness expect two mints
        //pumpkin all pools are with native SOL => SOL/Spl token
        vec![pubkey!("So11111111111111111111111111111111111111112"), self.state.token]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.key, get_global_config_address(&self.program_id)]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let mut bounding_curve_data = try_get_account_data(account_map, &self.key)?;
        self.state = BondingCurve::try_deserialize(&mut bounding_curve_data)?;

        let mut global_config_data = try_get_account_data(account_map, &get_global_config_address(&self.program_id))?;
        let config = GlobalConfig::try_deserialize(&mut global_config_data)?;
        self.swap_fee_rate_bps = config.swap_fee_rate_bps;
        self.index_fund_recipient = config.index_fund_recipient;
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let fee_amount = quote_params
            .amount
            .checked_mul(self.swap_fee_rate_bps)
            .unwrap()
            .checked_div(MAX_FEE_BASIS_POINTS)
            .unwrap();
        let sol_out = quote_params.input_mint.eq(&self.state.token);
        let amount_out = self.state.calculate_amount_out(
            sol_out,
            quote_params.amount,
            self.swap_fee_rate_bps,
        )?;
        Ok(Quote {
            out_amount: amount_out,
            in_amount: quote_params.amount,
            fee_mint: quote_params.input_mint,
            fee_amount,
            ..Quote::default()
        })
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let SwapParams {
            token_transfer_authority,
            source_token_account,
            destination_token_account,
            source_mint,
            ..
        } = swap_params;

        let sol_out = source_mint.eq(&self.state.token);

        let account_metas: Vec<AccountMeta> = if sol_out {
            SwapTokenForSol {
                global_config: get_global_config_address(&self.program_id),
                bonding_curve: self.key,
                mint: self.state.token,
                bonding_curve_token_vault: get_associated_token_address(
                    &self.key,
                    &self.state.token,
                ),
                sol_vault: get_sol_vault_address(&self.program_id),
                index_fund_recipient: self.index_fund_recipient,
                index_fund_recipient_token_account: get_associated_token_address(
                    &self.index_fund_recipient,
                    &self.state.token,
                ),
                user_token_account: *source_token_account,
                user: *token_transfer_authority,
            }
            .into()
        } else {
            SwapSolForToken {
                global_config: get_global_config_address(&self.program_id),
                bonding_curve: self.key,
                mint: self.state.token,
                bonding_curve_token_vault: get_associated_token_address(
                    &self.key,
                    &self.state.token,
                ),
                sol_vault: get_sol_vault_address(&self.program_id),
                user_token_account: *destination_token_account,
                user: *token_transfer_authority,
            }
            .into()
        };

        Ok(SwapAndAccountMetas {
            swap: Swap::PumpkinSwap{sol_out},
            account_metas,
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn is_active(&self) -> bool {
        !self.state.complete
    }
}
