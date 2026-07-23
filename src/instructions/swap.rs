use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap {
    pub payer: Signer,
    pub system_program: Program<SystemProgram>,
}

impl Swap {
    #[inline(always)]
    pub fn swap(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
