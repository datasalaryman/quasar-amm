use quasar_lang::prelude::*;
use zeropod::ZeroPod;

#[account(discriminator = 1, set_inner)]
#[seeds(b"amm", authority: Address, mint_x: Address, mint_y: Address)]
pub struct Config {
    pub version: u8, 
    pub state: u8, 
    pub seed: [u8; 8], 
    pub authority: Address, 
    pub mint_x: Address, 
    pub mint_y: Address, 
    pub fee: [u8; 2], 
    pub bump: [u8; 1]
}

#[derive(ZeroPod)]
#[repr(u8)]
enum AmmState {
    Uninitialized = 0u8, 
    Initialized = 1u8, 
    Disabled = 2u8, 
    WithdrawOnly = 3u8, 
}



