use quasar_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(
    mint_x: Address, 
    mint_y: Address
)]
pub struct Initialize {
    pub payer: Signer,
    #[account(
        init, 
        payer = payer, 
        address = Config::seeds(payer.address(), mint_x, mint_y) 
    )]
    pub config: Config, 
    pub system_program: Program<SystemProgram>,
}

impl Initialize {
    #[inline(always)]
    pub fn initialize(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
