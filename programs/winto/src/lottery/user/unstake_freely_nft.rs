use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

/// unstake freely NFT
#[derive(Accounts)]
pub struct UnstakeFreelyNft<'info> {
    /// user wallet
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// Wngs NFT metadata
    /// CHECK: Safe account
    pub nft_metadata_account: UncheckedAccount<'info>,

    /// user freely nft ata
    #[account(
        mut,
        constraint = user_nft_token_account.owner == user_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = user_nft_token_account.mint == user_details.freely_nft_mint_address @ WinError::InvalidTokenMint,
    )]
    pub user_nft_token_account: Box<Account<'info, TokenAccount>>,

    /// freely vault nft ata
    #[account(
        mut,
        constraint = nft_vault_account.mint == user_details.freely_nft_mint_address @ WinError::InvalidTokenMint,
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
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump = user_details.bump,
        constraint = user_details.is_freely_nft_staked == true @ WinError::AccessDenied
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> UnstakeFreelyNft<'info> {
    pub fn process(&mut self) -> Result<()> {
        let cur_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        if cur_time - self.user_details.freely_nft_staked_time < (self.win.freely_ticket_nft_staking_lock_period[self.user_details.freely_nft_type as usize] as u32).checked_mul(86400 as u32).ok_or(WinError::NumericalOverflow)? {
            return Err(error!(WinError::NoUnstakeFreelyTicketNft));
        }

        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[WINGS_NFT_POOL.as_ref()], &crate::ID);

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.nft_vault_account.to_account_info(),
                    to: self.user_nft_token_account.to_account_info(),
                    authority: self.vault_authority.to_account_info(),
                },
                &[&[WINGS_NFT_POOL.as_ref(), &[bump_seed]]],
            ),
            1,
        )?;

        self.user_details.freely_ticket_amount = 0;
        self.user_details.is_freely_nft_staked = false;
        self.user_details.freely_nft_type = 0;
        Ok(())
    }
}
