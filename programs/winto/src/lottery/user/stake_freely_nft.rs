use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};
use spl_token_metadata::state::Metadata;

/// lock wings NFT
#[derive(Accounts)]
pub struct StakeFreelyNft<'info> {
    /// user wallet
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    // freely NFT mint
    pub mint_nft: Box<Account<'info, Mint>>,

    /// Wngs NFT metadata
    /// CHECK: Safe account
    pub nft_metadata_account: UncheckedAccount<'info>,

    /// organizer wings nft ata
    #[account(
        mut,
        constraint = user_nft_token_account.owner == user_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = user_nft_token_account.mint == mint_nft.key() @ WinError::InvalidTokenMint,
    )]
    pub user_nft_token_account: Box<Account<'info, TokenAccount>>,

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
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump = user_details.bump,
        constraint = user_details.is_freely_nft_staked == false @ WinError::AccessDenied
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win global PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> StakeFreelyNft<'info> {
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
        let mut creator_index = 0;
        match nft_creators {
            Some(creators) => {
                for (_i, creator) in creators.iter().enumerate() {
                    for(j, freely_creator) in self.win.freely_ticket_nft_creators.iter().enumerate() {
                        if &creator.address == freely_creator && creator.verified == true {
                            verified = true;
                            creator_index = j;
                            break;
                        }
                    }
                    if verified {
                        break;
                    }
                }
            }
            None => {
                msg!("No creators found in metadata");
            }
        }

        if !verified {
            return Err(error!(WinError::NoFreelyTicketNft));
        }

        // lock Wings NFT
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.user_nft_token_account.to_account_info(),
                    to: self.nft_vault_account.to_account_info(),
                    authority: self.user_wallet.to_account_info(),
                },
            ),
            1,
        )?;

        self.user_details.freely_ticket_amount = self.win.freely_ticket_amount[creator_index];
        self.user_details.freely_nft_staked_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        self.user_details.freely_nft_mint_address = self.mint_nft.key();
        self.user_details.freely_nft_type = creator_index as u8;
        self.user_details.is_freely_nft_staked = true;
        Ok(())
    }
}
