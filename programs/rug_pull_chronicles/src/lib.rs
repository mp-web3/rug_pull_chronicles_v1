use anchor_lang::prelude::*;
use anchor_lang::system_program::System;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;
// use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::instructions::CreateMetadataAccountV3;


// pub const RUG_PULL_CHRONICLES_V1_PROGRAM_ID: &str = "3tdk2JLvsyQo3SQUTwDgozdA34SQjWhSVebaMbrHLsN4";
declare_id!("3tdk2JLvsyQo3SQUTwDgozdA34SQjWhSVebaMbrHLsN4");

#[program]
pub mod rug_pull_chronicles {
    use super::*;

    pub fn mint_rug(ctx: Context<Initialize>) -> Result<()> {
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            1,
        )?;
        // Create Metadata
        let cpi_accounts = mpl_token_metadata::accounts::create_metadata_accounts_v3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.authority.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            update_authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_metadata_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let creators = vec![mpl_token_metadata::types::Creator {
            address: ctx.accounts.authority.key(),
            verified: false,
            share: 100,
        }];

        mpl_token_metadata::instructions::create_metadata_accounts_v3(
            cpi_ctx,
            "Rug Pull Chronicles #1".to_string(),
            "RUG".to_string(),
            "https://devnet.irys.xyz/4NprvLvVgEUBWd8aiYA9MguvLd3Cc9RdbhQUZii32erj".to_string(),
            Some(creators),
            100,
            true,
            true,
            None,
            None,
            None,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintRug<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub metadata: UncheckedAccount<'info>,

    #[account(address = TOKEN_METADATA_PROGRAM_ID)]
    pub token_metadata_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
