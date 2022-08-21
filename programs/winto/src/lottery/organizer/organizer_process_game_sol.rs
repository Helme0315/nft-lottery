use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

/// organizer claim or move to escrow game money(SOL)
#[derive(Accounts)]
pub struct OrganizerProcessGameSol<'info> {
    /// organizer wallet
    #[account(mut)]
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// game pda
    #[account(
        mut,
        constraint = game.nft_owner_wallet == organizer_wallet.key() @ WinError::AccessDenied,
        constraint = game.funds_status == FundsStatus::NotClaimed @ WinError::ClaimedAlready,
        constraint = game.coin_type == CoinType::SOL @ WinError::InvalidAccount
    )]
    pub game: Box<Account<'info, Game>>,

    /// Game SOL Pool
    /// CHECK: Safe account
    #[account(
        mut,
        seeds = [GAME_POOL.as_ref()],
        bump,
    )]
    pub game_sol_pool: UncheckedAccount<'info>,

    /// Community wallet
    /// CHECK: Safe account
    #[account(
        mut,
        seeds = [COMMUNITY.as_ref()],
        bump
    )]
    pub coummunity_account: UncheckedAccount<'info>,

    /// Win Global PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>
}

impl<'info> OrganizerProcessGameSol<'info> {
    pub fn process(&mut self) -> Result<()> {
        let total_earn = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;
        if **self.game_sol_pool.lamports.borrow() < total_earn {
            return Err(error!(WinError::InsufficientSolBalance));
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
    
            invoke_signed(
                &system_instruction::transfer(
                    self.game_sol_pool.key,
                    self.coummunity_account.key,
                    community_fee,
                ),
                &[
                    self.game_sol_pool.to_account_info().clone(),
                    self.coummunity_account.to_account_info().clone(),
                    self.system_program.to_account_info().clone(),
                ],
                &[&[GAME_POOL.as_ref(), &[bump_seed]]],
            )?;
            
            invoke_signed(
                &system_instruction::transfer(
                    self.game_sol_pool.key,
                    self.organizer_wallet.key,
                    organizer_fee,
                ),
                &[
                    self.game_sol_pool.to_account_info().clone(),
                    self.organizer_wallet.to_account_info().clone(),
                    self.system_program.to_account_info().clone(),
                ],
                &[&[GAME_POOL.as_ref(), &[bump_seed]]],
            )?;
    
            self.game.funds_status = FundsStatus::Withdrawed;
        } else {
            return Err(error!(WinError::GameIsNotClosedStatus));
        }        

        Ok(())
    }
}

