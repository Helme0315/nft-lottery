use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};

/// Winner get NFT
#[derive(Accounts)]
pub struct WinnerClaimNft<'info> {
    /// winner
    pub winner_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// winner NFT
    pub winner_nft: Box<Account<'info, Mint>>,

    /// winner nft ata
    #[account(
        mut,
        constraint = winner_nft_ata.mint == winner_nft.key() @ WinError::InvalidTokenMint,
        constraint = winner_nft_ata.owner == winner_wallet.key() @ WinError::InvalidTokenOwner
    )]
    pub winner_nft_ata: Box<Account<'info, TokenAccount>>,

    /// NFT Pool
    /// CHECK: Safe account
    #[account(
        seeds = [NFT_POOL.as_ref()],
        bump
    )]
    pub nft_pool: UncheckedAccount<'info>,

    /// NFT Pool ata of nft
    #[account(
        mut,
        constraint = nft_pool_ata.mint == winner_nft.key() @ WinError::InvalidTokenMint,
        constraint = nft_pool_ata.owner == nft_pool.key() @ WinError::InvalidTokenOwner
    )]
    pub nft_pool_ata: Box<Account<'info, TokenAccount>>,

    /// game pda
    pub game: Box<Account<'info, Game>>,

    /// user bid PDA
    #[account(
        mut,
        seeds = [USER_BID.as_ref(), b"1".as_ref(), &winner_wallet.key().to_bytes(), &game.key().to_bytes()],
        bump = user_global_bid.bump,
        constraint = user_global_bid.winner_nft_claim == false @ WinError::NftAlreadyClaimed
    )]
    pub user_global_bid: Box<Account<'info, UserBid>>,

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

impl<'info> WinnerClaimNft<'info> {
    pub fn process(&mut self) -> Result<()> {
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        let total_earn = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;

        if total_earn >= self.game.minimum_cost && current_time > self.game.opened_timestamp.checked_add(self.game.duration).ok_or(WinError::NumericalOverflow)? {
            let mut nft_exist = false;
            for nft in self.game.nft_mint_address.iter() {
                if nft == &self.winner_nft.key() {
                    nft_exist = true;
                }
            }
            if nft_exist {
                // authority bump seed
                let (_pda, bump_seed) = Pubkey::find_program_address(&[NFT_POOL.as_ref()], &crate::ID);
    
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.nft_pool_ata.to_account_info(),
                            to: self.winner_nft_ata.to_account_info(),
                            authority: self.nft_pool.to_account_info(),
                        },
                        &[&[NFT_POOL.as_ref(), &[bump_seed]]],
                    ),
                    1,
                )?;
                self.user_global_bid.winner_nft_claim = true;
            } else {
                return Err(error!(WinError::AccessDenied));
            }
        } else {
            return Err(error!(WinError::GameIsNotClosedStatus));
        }
        

        Ok(())
    }
}
