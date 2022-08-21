use crate::globals::*;
use anchor_lang::{prelude::*};
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};
use spl_token_metadata::state::Metadata;
use std::mem::size_of;

/// lock wings NFT
#[derive(Accounts)]
pub struct LockWingsNft<'info> {
    /// organizer wallet
    #[account(mut)]
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    pub bot_wallet: Signer<'info>,

    // Wings NFT mint
    pub mint_nft: Box<Account<'info, Mint>>,

    /// Wngs NFT metadata
    /// CHECK: Safe account
    pub nft_metadata_account: UncheckedAccount<'info>,

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

    /// Win global PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
        constraint = win.bot_wallet == bot_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// User Wings NFT locked pda
    #[account(
        init,
        seeds = [&organizer_wallet.key().to_bytes(), &mint_nft.key().to_bytes()],
        bump,
        payer = organizer_wallet,
        space = 8 + size_of::<UserWingsNftLocked>(),
    )]
    pub user_wings_nft_locked: Box<Account<'info, UserWingsNftLocked>>,

    /// token program
    pub token_program: Program<'info, Token>,

    /// system program
    pub system_program: Program<'info, System>,
}

impl<'info> LockWingsNft<'info> {
    pub fn process(&mut self) -> Result<()> {

        let metadata_info = Metadata::from_account_info(&self.nft_metadata_account);

        let metadata = match metadata_info {
            Ok(v) => v,
            Err(_e) => return Err(error!(WinError::WrongMetadata)),
        };
        
        if metadata.mint != self.mint_nft.key() {
            return Err(error!(WinError::WrongMetadata));
        }

        let nft_creators = metadata.data.creators;
        let mut verified = false;
        match nft_creators {
            Some(creators) => {
                for (_i, creator) in creators.iter().enumerate() {
                    if creator.verified == true && creator.address == self.win.wings_creator {
                        verified = true;
                        break;
                    }
                }
            }
            None => {
                msg!("No creators found in metadata");
            }
        }

        if !verified {
            return Err(error!(WinError::NoWingsNft));
        }

        // lock Wings NFT
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.organizer_nft_token_account.to_account_info(),
                    to: self.nft_vault_account.to_account_info(),
                    authority: self.organizer_wallet.to_account_info(),
                },
            ),
            1,
        )?;

        self.user_details.wings_locked_count = self.user_details.wings_locked_count.checked_add(1).unwrap();

        self.user_wings_nft_locked.user_wallet = self.organizer_wallet.key();
        self.user_wings_nft_locked.wings_nft_mint = self.mint_nft.key();
        Ok(())
    }
}