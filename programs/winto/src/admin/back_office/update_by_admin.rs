use crate::globals::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateByAdminWallet<'info> {
    /// admin wallet
    pub admin_wallet: Signer<'info>,

    /// Win details PDA
    #[account(
        mut,
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.admin_wallet == admin_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,
}

impl<'info> UpdateByAdminWallet<'info> {
    pub fn process(&mut self, args: UpdateByAdminWalletArgs) -> Result<()> {
        for i in args.freely_ticket_amount {
            if i == 0 {
                return Err(error!(WinError::InvalidArgs));
            }
        }
        for i in args.freely_ticket_nft_staking_lock_period {
            if i == 0 {
                return Err(error!(WinError::InvalidArgs));
            }
        }
        self.win.fund_wallet = args.fund_wallet;
        self.win.wings_creator = args.wings_creator;
        self.win.sol_amount_for_bonus_tickets = args.sol_amount_for_bonus_tickets;
        self.win.token_amount_for_bonus_tickets = args.token_amount_for_bonus_tickets;
        self.win.game_bonus_ticket_amount = args.game_bonus_ticket_amount;
        self.win.freely_ticket_nft_creators = args.freely_ticket_nft_creators;
        self.win.freely_ticket_amount = args.freely_ticket_amount;
        self.win.freely_ticket_nft_staking_lock_period = args.freely_ticket_nft_staking_lock_period;
        self.win.community_fee = args.community_fee;
        self.win.is_emergency_flag = args.is_emergency_flag;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct UpdateByAdminWalletArgs {
    /// fund wallet
    pub fund_wallet: Pubkey,

    /// wings creator address
    pub wings_creator: Pubkey,

    /// get free tickes more than bid SOL amount
    pub sol_amount_for_bonus_tickets: u64,

    /// get free tickes more than bid token amount
    pub token_amount_for_bonus_tickets: u32,

    /// free tickets per game
    pub game_bonus_ticket_amount: u32,

    /// freely nft creators
    pub freely_ticket_nft_creators: [Pubkey;3],
    
    /// freely free ticket amount
    pub freely_ticket_amount: [u32; 3],

    /// staking locked period time
    pub freely_ticket_nft_staking_lock_period: [u8;3],

    /// community fee
    pub community_fee: u16,

    /// emergency flag
    pub is_emergency_flag: bool,
}
