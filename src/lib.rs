#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

declare_id!("Hsaj58jzXd5bq8PuGxZnwZfpMvCrNEk6ajgzAxtMhbSR");

#[program]
mod quasar_amm {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(
        ctx: Ctx<Initialize>, 
        mint_x: Address, 
        mint_y: Address
    ) -> Result<(), ProgramError> {
        ctx.accounts.initialize()
    }
    
    #[instruction(discriminator = 1)]
    pub fn deposit(ctx: Ctx<Deposit>) -> Result<(), ProgramError> {
        ctx.accounts.deposit()
    }
    
    #[instruction(discriminator = 2)]
    pub fn withdraw(ctx: Ctx<Withdraw>) -> Result<(), ProgramError> {
        ctx.accounts.withdraw() 
    }
    
    #[instruction(discriminator = 3)]
    pub fn swap(ctx: Ctx<Swap>) -> Result<(), ProgramError> {
        ctx.accounts.swap()
    }

}

#[cfg(test)]
mod tests;
