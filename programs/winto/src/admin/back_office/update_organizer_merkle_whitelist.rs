use crate::globals::*;
use anchor_lang::prelude::*;


/// update merkle tree
#[derive(Accounts)]
pub struct UpdateMerkleWhitelist<'info> {
    /// admin
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

impl<'info> UpdateMerkleWhitelist<'info> {
    pub fn process(&mut self, args: UpdateMerkleWhitelistArgs) -> Result<()> {
        self.win.organizer_whitelist_merkle_root = args.organizer_whitelist_merkle_root;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct UpdateMerkleWhitelistArgs {
    pub organizer_whitelist_merkle_root: [u8; 32],
}