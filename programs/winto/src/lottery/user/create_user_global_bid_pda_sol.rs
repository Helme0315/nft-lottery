use crate::globals::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_lang::{prelude::*, solana_program::clock};
use std::mem::size_of;

/// User create global bid pda and bid first time
#[derive(Accounts)]
#[instruction(args: CreateUserGlobalBidPdaSolArgs)]
pub struct CreateUserGlobalBidPdaSol<'info> {
    /// user
    #[account(mut)]
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// user bid PDA
    #[account(
        init,
        seeds = [USER_BID.as_ref(), b"1".as_ref(), &user_wallet.key().to_bytes(), &game.key().to_bytes()],
        bump,
        payer = user_wallet,
        space = 8 + size_of::<UserBid>(),
    )]
    pub user_global_bid: Box<Account<'info, UserBid>>,

    /// game pda
    #[account(
        mut,
        constraint = game.verify_game_time() @ WinError::GameIsNotOpenedStatus
    )]
    pub game: Box<Account<'info, Game>>,

    /// Game SOL Pool
    /// CHECK: Safe account
    #[account(
        mut,
        seeds = [GAME_POOL.as_ref()],
        bump,
    )]
    pub game_pool: UncheckedAccount<'info>,

    /// user details pda
    #[account(
        mut,
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump = user_details.bump,
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserGlobalBidPdaSol<'info> {
    pub fn process(&mut self, bump: u8, args: CreateUserGlobalBidPdaSolArgs) -> Result<()> {
        if args.ticket_amount.checked_add(args.bonus_ticket_amount).ok_or(WinError::NumericalOverflow)?  == 0 {
            return Err(error!(WinError::NoTicketAmount));
        }

        // current time, unix timestamp
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        let mut available_freely_ticket_amount = 0;
        if self.user_details.is_freely_nft_staked == true && 
        (current_time - self.user_details.freely_nft_staked_time < (self.win.freely_ticket_nft_staking_lock_period[self.user_details.freely_nft_type as usize] as u32).checked_mul(86400 as u32).ok_or(WinError::NumericalOverflow)?) {
            available_freely_ticket_amount = self.user_details.freely_ticket_amount;
        }

        if args.bonus_ticket_amount > available_freely_ticket_amount {
            return Err(error!(WinError::InvalidBonusTicketAmount));
        }

        let ticket_total_price = (args.ticket_amount as u128).checked_mul(self.game.ticket_price as u128).ok_or(WinError::NumericalOverflow)? as u64;
        if **self.user_wallet.lamports.borrow() < ticket_total_price {
            return Err(error!(WinError::InsufficientSolBalance));
        }
        if ticket_total_price >= self.win.sol_amount_for_bonus_tickets {
            self.user_global_bid.gbl_bonus_ticket_amount = self.win.game_bonus_ticket_amount;
            self.user_global_bid.gbl_has_distirbuted_bonus = true;
        } else {
            self.user_global_bid.gbl_bonus_ticket_amount = 0;
            self.user_global_bid.gbl_has_distirbuted_bonus = false;
        }
        
        self.game.current_total_tickets = self.game.current_total_tickets.checked_add(args.ticket_amount).ok_or(WinError::NumericalOverflow)?;
        self.game.current_total_bonus_tickets = self.game.current_total_bonus_tickets.checked_add(args.bonus_ticket_amount as u32).ok_or(WinError::NumericalOverflow)?;
        self.game.winner_random_number = args.random_number;

        self.user_global_bid.user_wallet = self.user_wallet.key();
        self.user_global_bid.game_pda_address = self.game.key();
        self.user_global_bid.opened_timestamp = current_time;
        self.user_global_bid.gbl_total_ticket_amount = args.ticket_amount;
        self.user_global_bid.bought_ticket_amount = args.ticket_amount;
        self.user_global_bid.bonus_ticket_amount = 0;
        self.user_global_bid.freely_ticket_amount = args.bonus_ticket_amount;
        self.user_details.freely_ticket_amount = self.user_details.freely_ticket_amount.checked_sub(args.bonus_ticket_amount).ok_or(WinError::NumericalOverflow)?;

        self.user_global_bid.total_bid_number = 1;
        self.user_global_bid.bid_number = 1;
        self.user_global_bid.funds_status = FundsStatus::NotClaimed;
        self.user_global_bid.winner_nft_claim = false;
        self.user_global_bid.bump = bump;
        

        if ticket_total_price > 0 {
            invoke(
                &system_instruction::transfer(
                    self.user_wallet.key,
                    self.game_pool.key,
                    ticket_total_price,
                ),
                &[
                    self.user_wallet.to_account_info().clone(),
                    self.game_pool.to_account_info().clone(),
                    self.system_program.to_account_info().clone(),
                ],
            )?;
        }
            

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateUserGlobalBidPdaSolArgs {
    /// ticket amount
    pub ticket_amount: u32,

    /// bonus ticket amount,
    pub bonus_ticket_amount: u32,

    /// winner random number
    pub random_number: [u32;6],
}
