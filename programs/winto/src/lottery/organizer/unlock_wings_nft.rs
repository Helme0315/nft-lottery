use crate::globals::*;
use anchor_lang::{prelude::*};
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};

/// unlock wings NFT
#[derive(Accounts)]
pub struct UnlockWingsNft<'info> {
    /// organizer wallet
    #[account(mut)]
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    pub bot_wallet: Signer<'info>,

    // Wings NFT mint
    pub mint_nft: Box<Account<'info, Mint>>,

    /// organizer wings nft ata
    #[account(
        mut,
        constraint = organizer_nft_token_account.owner == organizer_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = organizer_nft_token_account.mint == mint_nft.key() @ WinError::InvalidTokenMint,
    )]
    pub organizer_nft_token_account: Box<Account<'info, TokenAccount>>,

    /// Wings vault nft ata
    #[account(
        mut,
        constraint = nft_vault_account.mint == mint_nft.key() @ WinError::InvalidTokenMint,
        constraint = nft_vault_account.owner == vault_authority.key() @ WinError::InvalidTokenOwner
    )]
    pub nft_vault_account: Box<Account<'info, TokenAccount>>,

    /// vault authority
    /// CHECK: Safe account
    #[account(
        seeds = [WINGS_NFT_POOL.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// user details pda
    #[account(
        mut,
        seeds = [USER_DETAILS.as_ref(), &organizer_wallet.key().to_bytes()],
        bump = user_details.bump,
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
        constraint = win.bot_wallet == bot_wallet.key() @ WinError::AccessDenied,
    )]
    pub win: Box<Account<'info, Win>>,

    /// User Wings NFT locked pda
    #[account(
        mut,
        seeds = [&organizer_wallet.key().to_bytes(), &mint_nft.key().to_bytes()],
        bump,
        close = organizer_wallet
    )]
    pub user_wings_nft_locked: Box<Account<'info, UserWingsNftLocked>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> UnlockWingsNft<'info> {
    pub fn process(&mut self) -> Result<()> {

        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[WINGS_NFT_POOL.as_ref()], &crate::ID);

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.nft_vault_account.to_account_info(),
                    to: self.organizer_nft_token_account.to_account_info(),
                    authority: self.vault_authority.to_account_info(),
                },
                &[&[WINGS_NFT_POOL.as_ref(), &[bump_seed]]],
            ),
            1,
        )?;

        self.user_details.wings_locked_count = self.user_details.wings_locked_count.checked_sub(1).unwrap();
        Ok(())
    }
}
