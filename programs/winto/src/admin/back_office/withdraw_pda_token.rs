use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// Withdraw from PDA $WIN
#[derive(Accounts)]
#[instruction(args: WithdrawPdaTokenArgs)]
pub struct WithdrawPdaToken<'info> {
    /// withdraw wallet
    pub dev_wallet: Signer<'info>,

    /// withdraw token from COMMUNITY PDA
    #[account(
        mut,
        constraint = receive_ata.owner == win.fund_wallet @ WinError::InvalidTokenOwner,
    )]
    pub receive_ata: Box<Account<'info, TokenAccount>>,

    /// $WIN withdraw vault account
    #[account(
        mut,
        constraint = vault_token_account.owner == vault_authority.key() @ WinError::InvalidTokenOwner,
        constraint = args.withdraw_amount <= vault_token_account.amount @ WinError::InsufficientTokenBalance
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.dev_wallet == dev_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// vault authority
    /// CHECK: Safe account
    #[account(
        seeds = [COMMUNITY.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawPdaToken<'info> {
    pub fn process(&mut self, args: WithdrawPdaTokenArgs) -> Result<()> {
        if args.withdraw_amount == 0 {
            return Err(error!(WinError::InvalidAmount));
        }
        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[COMMUNITY.as_ref()], &crate::ID);

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.vault_token_account.to_account_info(),
                    to: self.receive_ata.to_account_info(),
                    authority: self.vault_authority.to_account_info(),
                },
                &[&[COMMUNITY.as_ref(), &[bump_seed]]],
            ),
            args.withdraw_amount,
        )?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct WithdrawPdaTokenArgs {
    /// withdraw amount
    pub withdraw_amount: u64,
}
