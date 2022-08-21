use anchor_lang::{prelude::*, solana_program::clock};
use crate::constants::*;
use crate::utility::*;
use crate::enums::*;

/// Win details
#[account]
pub struct Win {
    /// admin wallet key
    pub admin_wallet: Pubkey,

    /// bot wallet key
    pub bot_wallet: Pubkey,

    /// dev wallet key
    pub dev_wallet: Pubkey,

    /// receive $WIN or $SOL from pda
    pub fund_wallet: Pubkey,

    /// $WIN mint
    pub win_mint_address: Pubkey,

    /// wings creator
    pub wings_creator: Pubkey,

    /// SOL amount to get bonus tickets
    pub sol_amount_for_bonus_tickets: u64,

    /// token amount to get bonus tickets
    pub token_amount_for_bonus_tickets: u32,

    /// game bonus tickets amount
    pub game_bonus_ticket_amount: u32,

    /// community fee
    pub community_fee: u16,

    /// freely NFT creator
    pub freely_ticket_nft_creators: [Pubkey;3],

    /// freely free tickets amount for freely NFT holders
    pub freely_ticket_amount: [u32;3],

    /// staking locked period time for freely NFT
    pub freely_ticket_nft_staking_lock_period: [u8;3],

    /// emergency flag
    pub is_emergency_flag: bool,

    /// bump
    pub bump: u8,

    /// organizer whitelist
    pub organizer_whitelist_merkle_root: [u8; 32],

    /// wings nft holder whitelist
    pub holder_whitelist_merkle_root: [u8; 32],
}

/// User details
#[account]
pub struct UserDetails {
    /// user key
    pub user_wallet: Pubkey,

    /// $WIN airdrop(5%) reward amount
    pub win_airdrop_reward_amount: u64,

    /// $WIN game(40%) reward amount
    pub win_game_reward_amount: u64,

    /// $WIN dao(10%) reward amount
    pub win_dao_reward_amount: u64,

    /// $WIN contributors(10%) reward amount
    pub win_contributors_reward_amount: u64,

    /// $WIN last reward claimed date
    pub win_last_reward_claimed_date: u32,

    /// freely NFT staked time
    pub freely_nft_staked_time: u32,

    /// freely NFT holder free ticket amount
    pub freely_ticket_amount: u32,

    /// last game distribute time
    pub last_game_distribute_time: u32,

    /// Wings NFT locked count
    pub wings_locked_count: u16,

    /// freely nft type
    pub freely_nft_type: u8,

    /// freely NFT staked or not
    pub is_freely_nft_staked: bool,

    /// bump
    pub bump: u8,

    /// freely NFT mint address
    pub freely_nft_mint_address: Pubkey,
}

impl UserDetails {
    pub fn verify_create_game(
        &self,
        proof: Vec<[u8; 32]>,
        organizer_wallet: Pubkey,
        root: [u8; 32],
        wings_type: WingsType
    ) -> bool {
        let node = anchor_lang::solana_program::keccak::hashv(&[
            &MERKLE_WHITELIST_USER_PROOF.as_ref(),
            &organizer_wallet.to_bytes(),
        ]);
        let verify = match wings_type {
            WingsType::None => {
                merkle_tree_verify(proof, root, node.0)
            }
            _ => {
                true
            }
        };
        verify
    }
}

/// Game PDA
#[account]
pub struct Game {
    /// nft owner wallet
    pub nft_owner_wallet: Pubkey,

    /// token address for buy ticket
    pub ticket_token_address: Pubkey,

    /// ticket price
    pub ticket_price: u64,

    /// minimum cost
    pub minimum_cost: u64,

    /// opened timestamp
    pub opened_timestamp: u32,

    /// duration
    pub duration: u32,

    /// total tickets
    pub current_total_tickets: u32,

    /// total bonus tickets
    pub current_total_bonus_tickets: u32,

    /// coin type; 0 - SOL, 1 - TOKEN
    pub coin_type: CoinType,

    /// nft stake or unstake; true: unstake, false: stake
    pub is_nft_unstaked: bool,

    /// fund status
    pub funds_status: FundsStatus,

    /// Wings NFT type
    pub wings_type: WingsType,

    /// bump
    pub bump: u8,

    /// Wings NFT mint address
    pub wings_nft_mint_address: Pubkey,

    /// winner NFT prize count
    pub winner_nft_count: [u8;5],

    /// winner random number
    pub winner_random_number: [u32;6],

    /// nft mint address
    pub nft_mint_address: [Pubkey;5]
}

impl Game {
    pub fn verify_game_time(
        &self,
    ) -> bool {
        let current_time = clock::Clock::get().unwrap().unix_timestamp as u32;
        if current_time >= self.opened_timestamp.checked_add(self.duration).unwrap() {
            false
        } else {
            true
        }
    }
}

/// User Bid PDA
#[account]
pub struct UserBid {
    /// user key
    pub user_wallet: Pubkey,

    /// game pda key
    pub game_pda_address: Pubkey,

    /// bid create timestamp
    pub opened_timestamp: u32, 

    /// total ticket amount
    pub gbl_total_ticket_amount: u32,

    /// real bought ticket amount
    pub bought_ticket_amount: u32,

    /// global bonus ticket amount
    pub gbl_bonus_ticket_amount: u32,

    /// bonus ticket amount
    pub bonus_ticket_amount: u32,

    /// freely ticket amount
    pub freely_ticket_amount: u32,

    /// total bid number
    pub total_bid_number: u8,

    /// bid number
    pub bid_number: u8,

    /// fund status
    pub funds_status: FundsStatus,

    /// winner claim nft
    pub winner_nft_claim: bool,

    /// bonus ticket amount distribute
    pub gbl_has_distirbuted_bonus: bool,

    /// bump
    pub bump: u8,
}

/// User Wings NFT locked PDA
#[account]
pub struct UserWingsNftLocked {
    /// user key
    pub user_wallet: Pubkey,

    /// wings nft mint address
    pub wings_nft_mint: Pubkey,
}