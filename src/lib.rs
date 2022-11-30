use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("DxDyTMXpy3LQ4XfsoP44fBY4EyAUYhEdycedwPjbo4ay");

#[program]
pub mod public_posts_sol {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        // get the public key of the authority
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        // Initzialize the post and set properties

        // Increment the post total and the id
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        // order is also important
        post_account.id = user_account.last_post_id;
        post_account.title = title;
        post_account.content = content;
        // get the publicKey of Useraccount
        post_account.user = user_account.key();
        // get the wallet pubkey
        post_account.authority = authority.key();

        // Increase post_id by 1, specified in checked_add
        user_account.last_post_id = user_account.last_post_id.checked_add(1).unwrap();

        // total posts
        user_account.post_count = user_account.post_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    // info is a lifetime variable
    // js variables have unlimited lifetime, only until the garbage collector collects it
    // in rust, the lifetime lasts only as long as we need it
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 264 + 8,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [POST_SEED, authority.key().as_ref(), &[user_account.last_post_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 2376 + 8
    )]
    // 8 = account discriminator
    pub post_account: Account<'info, PostAccount>,

    // get ctx of useraccount
    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
