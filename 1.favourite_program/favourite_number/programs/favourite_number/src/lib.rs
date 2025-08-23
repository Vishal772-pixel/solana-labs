
// Header for anchor 
use anchor_lang::prelude::*;


declare_id!("");


// this is how contants are assigned in rust or anchor
// specifies the typenof account it is 
pub const ANCHOR_DISCRIMINATOR:usize =8;

// after adding below #program it turned into solana smart contract
#[program]

pub mod favourites{
    use super::*;

    pub fn set_favourites(context:Context<SetFavourites>,
         number:u64,
        color:String,
        hobbies:Vec<String>,
        )->Return<()>{

        msg!("Greeting fom {}", context.program_id);
   let user_public_key= context.account.user.key();
   msg!("User{user_public_key}'s favourite color is {color} and their favourite number is {number} and hobbies are {hobbies:?}")  



   // need to get what below context cod means ...shayd connect karne ke liye hai program to accounts 
   context.accounts.favourites.set_inner(Favourites{
    number,
    color,
    hobbies,
   })

   Ok(())
}
}



#[account]
// to let anchor know ..how big is favourite is 
#[derive(InitSpace)]
pub struct Favourites{
   pub number :u64,

   #[max_len(50)]// isse pta chalega ki  kitna lamba ho sakta  hai ..ye
   pub color :String ,

   #[max_len(5,50)]
   pub hobbies: Vec<String>,

}



#[derive(accounts)]
// another Struct ...( learn lifetime , mutablity ..to uderstand )
pub struct SetFavourites<'info>{
    #[account(mut)]
    // the user will sign for transaction ...thats wht Signer means here
    pub user:Signer<'info>,

// niche wale code ka matal agar ..Favourite account nahi bana hai toh ek naya account bana 
    #[account(
        init_if_needed,
        payer=user,
        space= ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds=[b"favourites",user.key().as_ref()],
         bump // it is used to calculate those seeds
    )]

      pub favourites:Account<'info,Favourites>,


      // System Program ..it lasts for the life time of info and program of type system
  pub system_program:Program<'info,System>,


}
