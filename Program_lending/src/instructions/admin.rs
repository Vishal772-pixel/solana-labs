use anchor_lang::prelude ::*;
use anchor_spl::token_interface::(Mint);



#[derive(Accounts)]

// Get a deep knowledge about PDA or u will suffer here
pub struct InitBank <Info>{
   #[account(mut)]{
    pub signer:Signer<Info>,
    pub mint:InterfaceAccounts<'info,Mint>


    #[account(
    init,
    payer=signer,
    space=8+Bank::INIT_SPACE,
    seeds=[mint.key().as_ref()],
    bump,
    ])

    pub bank:Account<'info,Bank>,

    #[accounts(
    init,
    token::mint=mint,
    token:authority=bank_token_account,
    payer=signer,
    seeds=[b"treasury",mint.key().as_ref()],
    bump,///iska matlab pta karna padega bc
     ) ]

pub bank_token_account:InterFaceAccount<'info,TokenAccount>,
pub token_program:Interface<'info,TokenInterface>,
pub system_program:Program<'info,System>
// so this is all we need to structure bank 

pub fn process_init_bank(ctx:Context<InitBank>liquidation_threshold:u64,max_ltv:u64)->Result<()>{
   let bank:& mut Accounts ='_,Bank = & mut ctx.accounts.bank;
   bank.mint_address = ctx.accounts.mint.key();
   bank.authority=ctx.accounts.signer.key();
 bank.max_ltv=max_ltv;
 {(ok)}


}








    )]










   }

}


// add spl tokn to cargo.toml by below command
// cargo add anchor-spl