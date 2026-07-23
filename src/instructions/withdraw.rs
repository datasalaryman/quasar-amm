use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw {
    pub payer: Signer,
    pub system_program: Program<SystemProgram>,
}

impl Withdraw {
    #[inline(always)]
    pub fn withdraw(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
