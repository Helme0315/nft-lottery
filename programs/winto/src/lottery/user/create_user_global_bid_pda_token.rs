use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use std::mem::size_of;

/// User create global bid pda and bid first time
#[derive(Accounts)]
#[instruction(args: CreateUserGlobalBidPdaTokenArgs)]
pub struct CreateUserGlobalBidPdaToken<'info> {
    /// user
    #[account(mut)]
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    #[account(
        constraint = token_mint.key() == game.ticket_token_address @ WinError::InvalidTokenMint
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    /// user token ata
    #[account(
        mut,
        constraint = user_token_ata.owner == user_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = user_token_ata.mint == game.ticket_token_address @ WinError::InvalidTokenMint
    )]
    pub user_token_ata: Box<Account<'info, TokenAccount>>,

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

    /// Game token Pool
    #[account(
        mut,
        constraint = game_token_pool.owner == game_pool.key() @ WinError::InvalidTokenMint,
        constraint = game_token_pool.mint == game.ticket_token_address @ WinError::InvalidTokenMint,
    )]
    pub game_token_pool: Box<Account<'info, TokenAccount>>,

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

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateUserGlobalBidPdaToken<'info> {
    pub fn process(&mut self, bump: u8, args: CreateUserGlobalBidPdaTokenArgs) -> Result<()> {
        if args.ticket_amount.checked_add(args.bonus_ticket_amount).ok_or(WinError::NumericalOverflow)? == 0 {
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
        if self.user_token_ata.amount < ticket_total_price {
            return Err(error!(WinError::InsufficientTokenBalance));
        }

        if ticket_total_price >= (self.win.token_amount_for_bonus_tickets as u128).checked_mul(self.game.ticket_price as u128).ok_or(WinError::NumericalOverflow)? as u64 {
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
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.user_token_ata.to_account_info(),
                        to: self.game_token_pool.to_account_info(),
                        authority: self.user_wallet.to_account_info(),
                    },
                ),
                ticket_total_price,
            )?;
        }
            

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateUserGlobalBidPdaTokenArgs {
    /// ticket amount
    pub ticket_amount: u32,

    /// bonus ticket amount,
    pub bonus_ticket_amount: u32,

    /// winner random number
    pub random_number: [u32;6],
}
