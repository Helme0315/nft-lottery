use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

///////////////////////////////
/// Distrinute $WIN to vault account by tokenomic
///////////////////////////////

#[derive(Accounts)]
pub struct AssetsDistribution<'info> {
    /// admin
    pub admin_wallet: Signer<'info>,

    /// $WIN admin wallet token account
    #[account(
        mut,
        constraint = admin_win_ata.owner == admin_wallet.key() @ WinError::InvalidTokenOwner,
        constraint = admin_win_ata.mint == win.win_mint_address @ WinError::InvalidTokenMint,
    )]
    pub admin_win_ata: Box<Account<'info, TokenAccount>>,

    /// $WIN airdrop vault account
    #[account(
        mut,
        constraint = vault_token_account.mint == win.win_mint_address @ WinError::InvalidTokenMint,
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.admin_wallet == admin_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> AssetsDistribution<'info> {
    pub fn process(&mut self, args: AssetsDistributionArgs) -> Result<()> {
        if args.amount == 0 {
            return Err(error!(WinError::InvalidAmount));
        }
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.admin_win_ata.to_account_info(),
                    to: self.vault_token_account.to_account_info(),
                    authority: self.admin_wallet.to_account_info(),
                },
            ),
            args.amount,
        )?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct AssetsDistributionArgs {
    /// amount
    pub amount: u64,
}
