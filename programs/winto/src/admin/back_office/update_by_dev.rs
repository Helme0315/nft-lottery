use crate::globals::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateByDevWallet<'info> {
    /// dev wallet
    pub dev_wallet: Signer<'info>,

    /// Win details PDA
    #[account(
        mut,
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.dev_wallet == dev_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,
}

impl<'info> UpdateByDevWallet<'info> {
    pub fn process(&mut self, args: UpdateByDevWalletArgs) -> Result<()> {
        self.win.bot_wallet = args.bot_wallet;
        self.win.wings_creator = args.wings_creator;
        self.win.is_emergency_flag = args.is_emergency_flag;
        self.win.freely_ticket_nft_creators = args.freely_ticket_nft_creators;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct UpdateByDevWalletArgs {
    /// bot wallet
    pub bot_wallet: Pubkey,

    /// wings creator address
    pub wings_creator: Pubkey,

    /// emergency flag
    pub is_emergency_flag: bool,

    /// freely nft creators
    pub freely_ticket_nft_creators: [Pubkey;3],
}
