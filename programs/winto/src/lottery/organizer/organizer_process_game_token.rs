use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// organizer claim or move to escrow game money(any token)
#[derive(Accounts)]
pub struct OrganizerProcessGameToken<'info> {
    /// organizer wallet
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// User token account
    #[account(
        mut,
        constraint = organizer_token_ata.mint == game.ticket_token_address @ WinError::InvalidTokenMint,
        constraint = organizer_token_ata.owner == organizer_wallet.key() @ WinError::InvalidTokenOwner
    )]
    pub organizer_token_ata: Box<Account<'info, TokenAccount>>,

    /// game pda
    #[account(
        mut,
        constraint = game.nft_owner_wallet == organizer_wallet.key() @ WinError::AccessDenied,
        constraint = game.funds_status == FundsStatus::NotClaimed @ WinError::ClaimedAlready,
        constraint = game.coin_type == CoinType::TOKEN @ WinError::InvalidAccount
    )]
    pub game: Box<Account<'info, Game>>,

    /// Community PDA
    /// CHECK: Safe account
    #[account(
        seeds = [COMMUNITY.as_ref()],
        bump
    )]
    pub vault_token_account: UncheckedAccount<'info>,

    /// admin wallet token account
    #[account(
        mut,
        constraint = receive_token_ata.mint == game.ticket_token_address @ WinError::InvalidTokenMint,
        constraint = receive_token_ata.owner == vault_token_account.key() @ WinError::InvalidTokenOwner,
    )]
    pub receive_token_ata: Box<Account<'info, TokenAccount>>,

    /// Game Token Pool
    #[account(
        mut,
        constraint = game_token_pool.mint == game.ticket_token_address @ WinError::InvalidTokenMint,
        constraint = game_token_pool.owner == game_pool.key() @ WinError::AccessDenied,
    )]
    pub game_token_pool: Box<Account<'info, TokenAccount>>,

    /// CHECK: Safe account
    #[account(
        seeds = [GAME_POOL.as_ref()],
        bump,
    )]
    pub game_pool: UncheckedAccount<'info>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> OrganizerProcessGameToken<'info> {
    pub fn process(&mut self) -> Result<()> {
        let total_earn = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;
        if total_earn > self.game_token_pool.amount {
            return Err(error!(WinError::InsufficientTokenBalance));
        }
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        
        if total_earn >= self.game.minimum_cost && current_time > self.game.opened_timestamp.checked_add(self.game.duration).ok_or(WinError::NumericalOverflow)? {

            let commision_fee = match self.game.wings_type {
                WingsType::Gold => {
                    self.win.community_fee.checked_mul(7).ok_or(WinError::NumericalOverflow)?
                }
                WingsType::Silver => {
                    self.win.community_fee.checked_mul(8).ok_or(WinError::NumericalOverflow)?
                }
                WingsType::Bronze => {
                    self.win.community_fee.checked_mul(9).ok_or(WinError::NumericalOverflow)?
                }
                WingsType::None => {
                    self.win.community_fee.checked_mul(10).ok_or(WinError::NumericalOverflow)?
                }
            };
            
            let community_fee = (total_earn as u128).checked_mul(commision_fee as u128).ok_or(WinError::NumericalOverflow)?.checked_div(10000 as u128).ok_or(WinError::NumericalOverflow)? as u64;
            let organizer_fee = total_earn.checked_sub(community_fee).ok_or(WinError::NumericalOverflow)?;
            
            // authority bump seed
            let (_pda, bump_seed) = Pubkey::find_program_address(&[GAME_POOL.as_ref()], &crate::ID);
    
            token::transfer(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.game_token_pool.to_account_info(),
                        to: self.receive_token_ata.to_account_info(),
                        authority: self.game_pool.to_account_info(),
                    },
                    &[&[GAME_POOL.as_ref(), &[bump_seed]]],
                ),
                community_fee,
            )?;

            token::transfer(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.game_token_pool.to_account_info(),
                        to: self.organizer_token_ata.to_account_info(),
                        authority: self.game_pool.to_account_info(),
                    },
                    &[&[GAME_POOL.as_ref(), &[bump_seed]]],
                ),
                organizer_fee,
            )?;
    
            self.game.funds_status = FundsStatus::Withdrawed;
        } else {
            return Err(error!(WinError::GameIsNotClosedStatus));
        }

        Ok(())
    }
}
