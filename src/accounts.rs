use anchor_lang::prelude::*;

use crate::constants::MAX_FEE_BASIS_POINTS;

#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub protocol_sol_fee_recipient: Pubkey,
    pub index_fund_recipient: Pubkey,
    pub migrator: Pubkey,
    pub authenticator: Pubkey,
    pub swap_fee_rate_bps: u64,
    pub creator_fee_portion_bps: u64,
    pub pkin_staking_fee_portion_bps: u64,
    pub staking_reward_period_duration: u64,
    pub pkin_staking_rewards_threshold: u64,
    pub index_fund_transfer_threshold: u64,
    pub bonding_curve_completion_threshold: u64,
    pub accrued_protocol_sol_fee: u64,
    pub pkin_staking_rewards: u64,
    pub bump: u8,
}

#[account]
pub struct BondingCurve {
    pub token: Pubkey,
    pub creator: Pubkey,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub accrued_creator_sol_fee: u64,
    pub index_fund_buffer: u64,
    pub complete: bool,
    pub bump: u8,
}

impl BondingCurve {
    pub fn calculate_amount_out(
        &self,
        sol_out: bool,
        amount_in: u64,
        fee_basis_points: u64,
    ) -> Result<u64> {
        if sol_out {
            return self.calculate_sol_out(amount_in, fee_basis_points);
        }
        self.calculate_token_out(amount_in, fee_basis_points)
    }

    fn calculate_token_out(&self, sol_amount_in: u64, fee_basis_points: u64) -> Result<u64> {
        let sol_amount_in_with_fee = u128::from(sol_amount_in)
            .checked_mul(u128::from(
                MAX_FEE_BASIS_POINTS.checked_sub(fee_basis_points).unwrap(),
            ))
            .unwrap();
        let numerator = sol_amount_in_with_fee
            .checked_mul(u128::from(self.virtual_token_reserves))
            .unwrap();
        let denominator = u128::from(self.virtual_sol_reserves)
            .checked_mul(u128::from(MAX_FEE_BASIS_POINTS))
            .unwrap()
            .checked_add(u128::from(sol_amount_in_with_fee))
            .unwrap();
        Ok(numerator
            .checked_div(denominator)
            .unwrap()
            .try_into()
            .unwrap())
    }

    fn calculate_sol_out(&self, token_amount_in: u64, fee_basis_points: u64) -> Result<u64> {
        let token_amount_in_with_fee = u128::from(token_amount_in)
            .checked_mul(u128::from(
                MAX_FEE_BASIS_POINTS.checked_sub(fee_basis_points).unwrap(),
            ))
            .unwrap();
        let numerator = token_amount_in_with_fee
            .checked_mul(u128::from(self.virtual_sol_reserves))
            .unwrap();
        let denominator = u128::from(self.virtual_token_reserves)
            .checked_mul(u128::from(MAX_FEE_BASIS_POINTS))
            .unwrap()
            .checked_add(u128::from(token_amount_in_with_fee))
            .unwrap();
        Ok(numerator
            .checked_div(denominator)
            .unwrap()
            .try_into()
            .unwrap())
    }
}
