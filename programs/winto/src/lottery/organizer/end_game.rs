use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};

/// end game
#[derive(Accounts)]
#[instruction(args: EndGameArgs)]
pub struct EndGame<'info> {
    /// organizer wallet
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
        constraint = game.current_total_tickets > 0 @ WinError::NoTicketAmount,
    )]
    pub game: Box<Account<'info, Game>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus
    )]
    pub win: Box<Account<'info, Win>>,
}

impl<'info> EndGame<'info> {
    pub fn process(&mut self, args: EndGameArgs) -> Result<()> {
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        if current_time
            > self
                .game
                .opened_timestamp
                .checked_add(self.game.duration)
                .ok_or(WinError::NumericalOverflow)?
        {
            return Err(error!(WinError::AlreadyGameEnd));
        } else {
            self.game.duration = current_time.checked_sub(self.game.opened_timestamp).ok_or(WinError::NumericalOverflow)?
        }
        let cur_ticket_money = (self.game.ticket_price as u128).checked_mul(self.game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;
        self.game.minimum_cost = cur_ticket_money;
        self.game.winner_random_number = args.random_number;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct EndGameArgs {
    /// winner random number
    pub random_number: [u32;6],
}
