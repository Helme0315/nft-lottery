use crate::globals::*;
use anchor_lang::{prelude::*};
use anchor_spl::token::{self, Transfer, Token};
use std::mem::size_of;

/// organizer create game
#[derive(Accounts)]
#[instruction(args: CreateGameArgs)]
pub struct CreateGame<'info> {
    /// organizer
    #[account(mut)]
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
        init,
        seeds = [GAME.as_ref(), &args.game_time_stamp.to_le_bytes(), &organizer_wallet.key().to_bytes()],
        bump,
        payer = organizer_wallet,
        space = 8 + size_of::<Game>(),
    )]
    pub game: Box<Account<'info, Game>>,

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

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateGame<'info> {
    pub fn process(&mut self, bump: u8, args: CreateGameArgs, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        if args.minimum_cost <= 0 || args.ticket_price <= 0 {
            return Err(error!(WinError::WrongVaule));
        }
        for (i, _user_nft_ata) in remaining_accounts.iter().enumerate().step_by(3) {
            if i % 3 == 0 {
                assert_is_ata(&remaining_accounts[i], &self.organizer_wallet.key(), &remaining_accounts[i+2].to_account_info().key())?;
                assert_is_ata(&remaining_accounts[i+1], &self.nft_pool.key(), &remaining_accounts[i+2].to_account_info().key())?;

                // transfer NFT to NFT-Pool
                token::transfer(
                    CpiContext::new(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: remaining_accounts[i].to_account_info(),
                            to: remaining_accounts[i+1].to_account_info(),
                            authority: self.organizer_wallet.to_account_info(),
                        },
                    ),
                    1,
                )?;
                let index = i.checked_div(3).unwrap();
                self.game.nft_mint_address[index] = remaining_accounts[i+2].to_account_info().key();
            }
        }

        self.game.nft_owner_wallet = self.organizer_wallet.key();
        self.game.ticket_token_address = args.ticket_token_address;
        self.game.ticket_price = args.ticket_price;
        self.game.minimum_cost = args.minimum_cost;
        self.game.opened_timestamp = args.game_time_stamp;
        self.game.duration = args.duration;
        self.game.current_total_tickets = 0;
        self.game.current_total_bonus_tickets = 0;
        self.game.coin_type = args.coin_type;
        self.game.is_nft_unstaked = false;
        self.game.funds_status = FundsStatus::NotClaimed;
        self.game.wings_type = args.wings_type;
        self.game.bump = bump;
        self.game.winner_nft_count = [1,0,0,0,0];
        self.game.winner_random_number = [0;6];
        self.game.wings_nft_mint_address = args.wings_nft_mint_address;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateGameArgs {
    /// token address for buy ticket
    pub ticket_token_address: Pubkey,

    /// ticket price
    pub ticket_price: u64,

    /// minimum cost
    pub minimum_cost: u64,

    /// proof
    pub proof: Vec<[u8; 32]>,

    /// game time stamp
    pub game_time_stamp: u32,

    /// duration
    pub duration: u32,

    /// coin type
    pub coin_type: CoinType,

    /// wings type
    pub wings_type: WingsType,

    /// Wings NFT mint address
    pub wings_nft_mint_address: Pubkey
}