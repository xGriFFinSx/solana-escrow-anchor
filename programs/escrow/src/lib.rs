pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

// use instructions::*;

declare_id!("8ccPM8xQngqPaKhUS6R4YfgvFa8ix2HaF6nTLTKprMJd");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
	amount: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, amount)
    }
}
