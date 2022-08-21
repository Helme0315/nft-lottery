use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Transfer, Token};

/// organizer get back NFT
#[derive(Accounts)]
pub struct OrganizerGetBackNft<'info> {
    /// winner
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// NFT Pool
    /// CHECK: Safe account
    #[account(
        seeds = [NFT_POOL.as_ref()],
        bump
    )]
    pub nft_pool: UncheckedAccount<'info>,

    /// game pda
    #[account(
        mut,
        constraint = game.is_nft_unstaked == false @ WinError::NftAlreadyClaimed,
        constraint = game.nft_owner_wallet == organizer_wallet.key() @ WinError::InvalidTokenOwner
    )]
    pub game: Box<Account<'info, Game>>,

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

impl<'info> OrganizerGetBackNft<'info> {
    pub fn process(&mut self, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        let total_earn = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;

        if total_earn < self.game.minimum_cost && current_time > self.game.opened_timestamp.checked_add(self.game.duration).ok_or(WinError::NumericalOverflow)? {
            // authority bump seed
            let (_pda, bump_seed) = Pubkey::find_program_address(&[NFT_POOL.as_ref()], &crate::ID);

            for (i, _ata) in remaining_accounts.iter().enumerate().step_by(2) {
                if i % 2 == 0 {
                    assert_is_ata(&remaining_accounts[i], &self.organizer_wallet.key(), &self.game.nft_mint_address[i/2])?;
                    assert_is_ata(&remaining_accounts[i+1], &self.nft_pool.key(), &self.game.nft_mint_address[i/2])?;

                    // transfer NFT to NFT-Pool
                    token::transfer(
                        CpiContext::new_with_signer(
                            self.token_program.to_account_info(),
                            Transfer {
                                from: remaining_accounts[i+1].to_account_info(),
                                to: remaining_accounts[i].to_account_info(),
                                authority: self.nft_pool.to_account_info(),
                            },
                            &[&[NFT_POOL.as_ref(), &[bump_seed]]],
                        ),
                        1,
                    )?;
                }
            }

            self.game.is_nft_unstaked = true;
        } else {
            return Err(error!(WinError::GameIsNotCancelledStatus));
        }
        
        Ok(())
    }
}

