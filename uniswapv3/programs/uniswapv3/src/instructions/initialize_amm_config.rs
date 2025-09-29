use anchor_lang::prelude::*;

use crate::state::{AmmConfig, AMM_CONFIG_SEED};

/// Accounts required to initialize a new global AMM configuration.
#[derive(Accounts)]
#[instruction(index: u16)]
pub struct InitializeAmmConfig<'info> {
    /// Funds the rent-exempt allocation of `amm_config`.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Designated authority allowed to manage AMM-wide settings.
    pub authority: Signer<'info>,

    /// PDA that stores global settings shared by all pools created by `authority`.
    #[account(
        init,
        seeds = [AMM_CONFIG_SEED.as_bytes(), &index.to_be_bytes()],
        bump,
        payer = payer,
        space = AmmConfig::space(),
    )]
    pub amm_config: Account<'info, AmmConfig>,

    /// Required by Anchor to create the account using the payer's lamports.
    pub system_program: Program<'info, System>,
}

pub fn initialize_amm_config(
    ctx: Context<InitializeAmmConfig>,
    index: u16,
    tick_spacing: u16,
    trade_fee_rate: u32,
    protocol_fee_rate: u32,
    fund_fee_rate: u32,
) -> Result<()> {
    let bump = ctx.bumps.amm_config;
    initialize_config_account(
        &mut ctx.accounts.amm_config,
        bump,
        ctx.accounts.authority.key(),
        index,
        tick_spacing,
        trade_fee_rate,
        protocol_fee_rate,
        fund_fee_rate,
    )?;

    Ok(())
}

fn initialize_config_account(
    amm_config: &mut AmmConfig,
    bump: u8,
    authority: Pubkey,
    index: u16,
    tick_spacing: u16,
    trade_fee_rate: u32,
    protocol_fee_rate: u32,
    fund_fee_rate: u32,
) -> Result<()> {
    amm_config.bump = bump;
    amm_config.authority = authority;
    amm_config.index = index;
    amm_config.tick_spacing = tick_spacing;
    amm_config.trade_fee_rate = trade_fee_rate;
    amm_config.protocol_fee_rate = protocol_fee_rate;
    amm_config.fund_fee_rate = fund_fee_rate;

    Ok(())
}
