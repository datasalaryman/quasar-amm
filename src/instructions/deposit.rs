use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Deposit {
    pub payer: Signer,
    pub system_program: Program<SystemProgram>,
}

impl Deposit {
    #[inline(always)]
    pub fn deposit(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
