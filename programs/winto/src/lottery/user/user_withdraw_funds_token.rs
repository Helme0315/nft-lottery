use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// user withdraw funds token
#[derive(Accounts)]
pub struct UserWithdrawFundsToken<'info> {
    /// user wallet
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// CHECK: Safe account
    #[account(
        mut,
        constraint = user_wallet.key() == rec_wallet.key() @ WinError::AccessDenied
    )]
    pub rec_wallet: UncheckedAccount<'info>,

    /// user token ata
    #[account(
        mut,
        constraint = game.coin_type == CoinType::SOL || user_token_ata.owner == user_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = game.coin_type == CoinType::SOL || user_token_ata.mint == game.ticket_token_address @ WinError::InvalidTokenMint
    )]
    pub user_token_ata: Box<Account<'info, TokenAccount>>,

    /// user bid PDA
    #[account(
        mut,
        seeds = [USER_BID.as_ref(), b"1".as_ref(), &user_wallet.key().to_bytes(), &game.key().to_bytes()],
        bump = user_global_bid.bump,
        constraint = user_global_bid.funds_status == FundsStatus::NotClaimed @ WinError::ClaimedAlready
    )]
    pub user_global_bid: Box<Account<'info, UserBid>>,

    /// game pda
    pub game: Box<Account<'info, Game>>,

    /// Game Pool
    /// CHECK: Safe account
    #[account(
        mut,
        seeds = [GAME_POOL.as_ref()],
        bump,
    )]
    pub game_pool: UncheckedAccount<'info>,

    /// Game token Pool
    #[account(
        mut,
        constraint = game.coin_type == CoinType::SOL || game_token_pool.owner == game_pool.key() @ WinError::InvalidTokenMint,
        constraint = game.coin_type == CoinType::SOL || game_token_pool.mint == game.ticket_token_address @ WinError::InvalidTokenMint,
    )]
    pub game_token_pool: Box<Account<'info, TokenAccount>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> UserWithdrawFundsToken<'info> {
    pub fn process(&mut self) -> Result<()> {
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        let total_earn = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;

        if total_earn >= self.game.minimum_cost || current_time < self.game.opened_timestamp.checked_add(self.game.duration).ok_or(WinError::NumericalOverflow)? {
            return Err(error!(WinError::GameIsNotCancelledStatus));
        }
        if self.user_global_bid.gbl_total_ticket_amount > 0 {
            let withdraw_funds = (self.user_global_bid.gbl_total_ticket_amount as u128).checked_mul(self.game.ticket_price as u128).ok_or(WinError::NumericalOverflow)? as u64;
            // authority bump seed
            let (_pda, bump_seed) = Pubkey::find_program_address(&[GAME_POOL.as_ref()], &crate::ID);
            
            if self.game_token_pool.amount < withdraw_funds {
                return Err(error!(WinError::InsufficientTokenBalance));
            }
            
            token::transfer(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.game_token_pool.to_account_info(),
                        to: self.user_token_ata.to_account_info(),
                        authority: self.game_pool.to_account_info(),
                    },
                    &[&[GAME_POOL.as_ref(), &[bump_seed]]],
                ),
                withdraw_funds,
            )?;

            self.user_global_bid.funds_status = FundsStatus::Withdrawed;
        }

        Ok(())
    }
}
