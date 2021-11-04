use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("EmrnGQ7rfeiiEJjwq4F1P5Y28bTB44co6JvpiijVSXG2");

#[program]
pub mod solana_blog {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let blog_acc = &mut ctx.accounts.blog_account;
        blog_acc.authority = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn make_post(
        ctx: Context<MakePost>,
        new_post: Vec<u8>
    ) -> ProgramResult {
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8, from_byte {}", err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(post);

        let blog_acc = &mut ctx.accounts.blog_account;
        blog_acc.latest_post = new_post; 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
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
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut,
        has_one = authority
    )]
    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>
}

#[account]
pub struct BlogAccount {
    pub latest_post: Vec<u8>,
    pub authority: Pubkey,
}
