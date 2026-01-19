use anchor_lang::prelude::*;
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + 32 + 8 + 1,
        seeds = [b"escrow", maker.key().as_ref()],
        bump
    )]

    /// CHECK: PDA vault holds SOL only
    pub vault: UncheckedAccount<'info>,

    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

use anchor_lang::solana_program::{
    program::invoke,
    system_instruction,
};

pub fn handler(
    ctx: Context<Initialize>,
    amount: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.maker = ctx.accounts.maker.key();
    escrow.amount = amount;
    escrow.bump = ctx.bumps.vault;
    //escrow.bump = ctx.bumps.escrow;

    // Transfer SOL from maker → vault PDA
    let transfer_ix = system_instruction::transfer(
        &ctx.accounts.maker.key(),
        &ctx.accounts.vault.key(),
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.maker.to_account_info(),
            ctx.accounts.vault.to_account_info(),
        ],
    )?;

    Ok(())
}

