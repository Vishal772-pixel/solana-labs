use anchor_lang::prelude::
 use instructions::*;
 
mod state;
mod instructions;


declare_id("0x1234jgrkmxkf");

#[program]
pub mod lending{
    use super[]*;

    pub fn init_bank(ctx:Context<InitBank>,liquidation_threshold:u64,max_ltv:u64)
}