use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use borsh::BorshSerialize;
use mpl_core::accounts::BaseAssetV1;
use mpl_core::ID as MPL_CORE_PROGRAM_ID;
use mpl_core::{Asset, DataBlob};
use mpl_core::{ExternalPluginAdaptersList, PluginsList};

declare_id!("3tdk2JLvsyQo3SQUTwDgozdA34SQjWhSVebaMbrHLsN4");

#[program]
pub mod rug_pull_chronicles {
    use super::*;

    pub fn mint_rug(ctx: Context<MintRug>) -> Result<()> {
        // Mint 1 token to user
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

        // Create the asset
        let asset = Asset {
            base: BaseAssetV1 {
                key: mpl_core::types::Key::AssetV1,
                owner: ctx.accounts.authority.key(),
                update_authority: mpl_core::types::UpdateAuthority::None,
                name: "Rug Pull Chronicles #1".to_string(),
                uri: "https://your-uploaded-image-url.json".to_string(),
                seq: Some(1),
            },
            plugin_list: PluginsList::default(),
            external_plugin_adapter_list: ExternalPluginAdaptersList::default(),
            plugin_header: None,
        };

        // Serialize the asset data
        let mut data = vec![];
        borsh::to_writer(&mut data, &asset)?;

        // Create the asset account
        let create_asset_ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.authority.key(),
            &ctx.accounts.asset.key(),
            Rent::get()?.minimum_balance(data.len()),
            data.len() as u64,
            &MPL_CORE_PROGRAM_ID,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &create_asset_ix,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.asset.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        // Write the asset data
        ctx.accounts.asset.data.borrow_mut().copy_from_slice(&data);

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
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: We create and validate the asset account in the instruction
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
