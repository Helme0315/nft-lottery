use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use std::mem::size_of;

/// initialize win details
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// admin
    #[account(mut)]
    pub admin_wallet: Signer<'info>,

    /// $WIN token address
    pub token_mint_address: Box<Account<'info, Mint>>,

    /// $WIN airdrop(5%) vault account
    #[account(
        init,
        seeds = [AIRDROP_VAULT.as_ref()],
        bump,
        payer = admin_wallet,
        token::mint = token_mint_address,
        token::authority = vault_authority,
    )]
    pub airdrop_token_account: Box<Account<'info, TokenAccount>>,

    /// $WIN DAO Treasury(10%) vault account
    #[account(
        init,
        seeds = [AIRDROP_DAO_TREASURY.as_ref()],
        bump,
        payer = admin_wallet,
        token::mint = token_mint_address,
        token::authority = vault_authority,
    )]
    pub dao_treasury_account: Box<Account<'info, TokenAccount>>,

    /// $WIN Contributors(10%) vault account
    #[account(
        init,
        seeds = [AIRDROP_CONTRIBUTORS.as_ref()],
        bump,
        payer = admin_wallet,
        token::mint = token_mint_address,
        token::authority = vault_authority,
    )]
    pub contributor_account: Box<Account<'info, TokenAccount>>,

    /// $WIN P2E(40%) vault account
    #[account(
        init,
        seeds = [AIRDROP_P2E.as_ref()],
        bump,
        payer = admin_wallet,
        token::mint = token_mint_address,
        token::authority = vault_authority,
    )]
    pub p2e_account: Box<Account<'info, TokenAccount>>,

    /// vault authority
    /// CHECK: Safe account
    #[account(
        seeds = [VAULT_AUTH.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// Win details PDA
    #[account(
        init,
        seeds = [WIN.as_ref()],
        bump,
        payer = admin_wallet,
        space = 8 + size_of::<Win>(),
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>,

    /// rent var
    pub rent: Sysvar<'info, Rent>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, bump: u8, args: InitializeArgs) -> Result<()> {
        self.win.admin_wallet = self.admin_wallet.key();
        self.win.bot_wallet = args.bot_wallet;
        self.win.dev_wallet = args.dev_wallet;
        self.win.fund_wallet = args.fund_wallet;
        self.win.win_mint_address = self.token_mint_address.key();
        self.win.wings_creator = args.wings_creator;
        self.win.sol_amount_for_bonus_tickets = 0;
        self.win.token_amount_for_bonus_tickets = 0;
        self.win.game_bonus_ticket_amount = 0;
        self.win.freely_ticket_nft_creators = args.freely_ticket_nft_creators;
        self.win.freely_ticket_amount = [0,0,0];
        self.win.freely_ticket_nft_staking_lock_period = [0,0,0];
        self.win.community_fee = 0;
        self.win.is_emergency_flag = false;
        self.win.bump = bump;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitializeArgs {
    /// bot wallet
    pub bot_wallet: Pubkey,

    /// withdraw wallet
    pub dev_wallet: Pubkey,

    /// receive wallet
    pub fund_wallet: Pubkey,

    /// wings creator address
    pub wings_creator: Pubkey,

    /// freely nft creator
    pub freely_ticket_nft_creators: [Pubkey; 3],
}