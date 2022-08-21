use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use std::mem::size_of;

/// organizer recreate game
#[derive(Accounts)]
#[instruction(args: RecreateGameArgs)]
pub struct RecreateGame<'info> {
    /// organizer
    #[account(mut)]
    pub organizer_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// game pda
    #[account(
        init,
        seeds = [GAME.as_ref(), &args.game_time_stamp.to_le_bytes(), &organizer_wallet.key().to_bytes()],
        bump,
        payer = organizer_wallet,
        space = 8 + size_of::<Game>(),
    )]
    pub game: Box<Account<'info, Game>>,

    /// old game pda
    #[account(
        mut,
        constraint = old_game.nft_owner_wallet == organizer_wallet.key() @ WinError::IncorrectOwner,
        constraint = old_game.is_nft_unstaked == false @ WinError::NoGameNft
    )]
    pub old_game: Box<Account<'info, Game>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
    )]
    pub win: Box<Account<'info, Win>>,

    /// user details pda
    #[account(
        seeds = [USER_DETAILS.as_ref(), &organizer_wallet.key().to_bytes()],
        bump = user_details.bump,
        constraint = user_details.user_wallet == organizer_wallet.key() @ WinError::AccessDenied,
        constraint = user_details.verify_create_game(args.proof, organizer_wallet.key(), win.organizer_whitelist_merkle_root, args.wings_type) @ WinError::UnableToCreateGame
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// system program
    pub system_program: Program<'info, System>,
}

impl<'info> RecreateGame<'info> {
    pub fn process(&mut self, bump: u8, args: RecreateGameArgs) -> Result<()> {
        let total_earn = (self.old_game.ticket_price as u128).checked_mul(self.old_game.current_total_tickets as u128).ok_or(WinError::NumericalOverflow)? as u64;
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;

        if total_earn < self.old_game.minimum_cost && current_time > self.old_game.opened_timestamp.checked_add(self.old_game.duration).ok_or(WinError::NumericalOverflow)? {
            self.game.nft_owner_wallet = self.organizer_wallet.key();
            self.game.ticket_token_address = self.old_game.ticket_token_address;
            self.game.ticket_price = self.old_game.ticket_price;
            self.game.minimum_cost = self.old_game.minimum_cost;
            self.game.opened_timestamp = args.game_time_stamp;
            self.game.duration = args.duration;
            self.game.current_total_tickets = 0;
            self.game.current_total_bonus_tickets = 0;
            self.game.coin_type = self.old_game.coin_type.clone();
            self.game.is_nft_unstaked = false;
            self.game.funds_status = FundsStatus::NotClaimed;
            self.game.wings_type = args.wings_type;
            self.game.wings_nft_mint_address = args.wings_nft_mint_address;
            self.game.bump = bump;
            self.game.nft_mint_address = self.old_game.nft_mint_address.clone();
            self.game.winner_nft_count = self.old_game.winner_nft_count;
            self.game.winner_random_number = [0;6];

            self.old_game.is_nft_unstaked = true;
            self.old_game.funds_status = FundsStatus::Retransfer;
        } else {
            return Err(error!(WinError::GameIsNotCancelledStatus));
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecreateGameArgs {
    /// proof
    pub proof: Vec<[u8; 32]>,
    
    /// game time stamp
    pub game_time_stamp: u32,

    /// wings type
    pub wings_type: WingsType,

    /// Wings NFT mint address
    pub wings_nft_mint_address: Pubkey,

    /// duration
    pub duration: u32
}