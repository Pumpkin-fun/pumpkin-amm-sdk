use anchor_lang::solana_program::pubkey::Pubkey;

pub fn get_global_config_address(bounding_curve_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"global_config"], bounding_curve_program).0
}

pub fn get_sol_vault_address(bounding_curve_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"sol_vault"], bounding_curve_program).0
}
