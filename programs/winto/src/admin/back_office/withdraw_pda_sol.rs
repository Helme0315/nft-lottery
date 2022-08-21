use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

/// Withdraw SOL from PDA $SOL
#[derive(Accounts)]
#[instruction(args: WithdrawPdaSolArgs)]
pub struct WithdrawPdaSol<'info> {
    /// withdraw wallet
    pub dev_wallet: Signer<'info>,

    /// $SOL admin wallet token account
    /// CHECK: Safe account
    #[account(
        mut,
        constraint = fund_wallet.key() == win.fund_wallet @ WinError::AccessDenied,
    )]
    pub fund_wallet: UncheckedAccount<'info>,

    /// $SOL withdraw vault account
    /// CHECK: Safe account
    #[account(
        mut,
        seeds = [COMMUNITY.as_ref()],
        bump,
        constraint = args.withdraw_amount <= **vault_account.lamports.borrow() @ WinError::InsufficientSolBalance
    )]
    pub vault_account: UncheckedAccount<'info>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.dev_wallet == dev_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// system program
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawPdaSol<'info> {
    pub fn process(&mut self, args: WithdrawPdaSolArgs) -> Result<()> {
        if args.withdraw_amount == 0 {
            return Err(error!(WinError::InvalidAmount));
        }
        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[COMMUNITY.as_ref()], &crate::ID);

        invoke_signed(
            &system_instruction::transfer(
                self.vault_account.key,
                self.fund_wallet.key,
                args.withdraw_amount,
            ),
            &[
                self.vault_account.to_account_info().clone(),
                self.fund_wallet.to_account_info().clone(),
                self.system_program.to_account_info().clone(),
            ],
            &[&[COMMUNITY.as_ref(), &[bump_seed]]],
        )?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct WithdrawPdaSolArgs {
    /// withdraw amount
    pub withdraw_amount: u64,
}
