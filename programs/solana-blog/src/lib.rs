use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_blog {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        //ctx.accounts
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(
        init,
        payer = authority,
        space = 8 //account descrtiminator,
        + 32 // pubkey
        + 566 //post 566 bytes long

    )]
    pub blog_account: Account<'info, BlogAccount>,
    //#[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Pubkey<'info, System>
}

#[account]
pub struct BlogAccount {
    pub latest_post: u64,
    pub authority: Pubkey,
}
