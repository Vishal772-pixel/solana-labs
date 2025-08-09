use anchor_lang::prelude::*;

#[account]
#[derive{InitSpace}]

pub struct User{
    // user ki kya kya chize hongi ( attribute , permission , function)

     pub owner =pubkey,
     pub deposited_sol:u64,
     pub deposited_sol_shares:u64,
     pub borrowed_sol:u64,
     pub borrowed_sol_shares:u64,
     pub borrowed_usdc:u64,
     pub borrowed_usdc_shares:u64,
     pub deposited_usdc:u64,
     pub deposited_usdc_shares:u64,
     pub usdc_address:Pubkey,
     pub last_updated:i64  //timestamp recorded

}
 // ab user ke sath bank bhi hoga ...jo lending and borrowing function perform karega 

#[account]
#[derive{InitSpace}]

pub struct bank{
    pub authority:Pubkey,// would have special permission t chnage 
    pub mint_address:Pubkey,
    pub total_deposits:u64,
    pub total_deposit_shares:u64,
     pub liquidation_threshold:u64,// a loan ..depend on which the loan would be defined as under-collateralized and can be liquidated
     pub liquidation_bonus:u6,/// percentage of liquiadtation .. would be send to the liquiadtor as a bonus for processing liquidaton
     pub liquidation_close_factor:u64, // percentage of collatoral that can be liquidated
     pub max_ltv:u64, /// max percentageof collater that can be borrowed
   pub last_updated:i64,
   
    } 
/// need to track liquidation bonus, liquidation bonus , close factor , max LTV..constants needed to calculate
