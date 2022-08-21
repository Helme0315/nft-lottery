use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// Withdraw from PDA $WIN
#[derive(Accounts)]
#[instruction(args: WithdrawFromPdaArgs)]
pub struct WithdrawFromPda<'info> {
    /// dev wallet
    pub dev_wallet: Signer<'info>,

    /// withdraw token from one of 4 PDAs
    #[account(
        mut,
        constraint = receive_ata.owner == win.fund_wallet @ WinError::InvalidTokenOwner,
    )]
    pub receive_ata: Box<Account<'info, TokenAccount>>,

    /// $WIN airdrop vault account
    #[account(
        mut,
        seeds = [AIRDROP_VAULT.as_ref()],
        bump,
    )]
    pub airdrop_token_account: Box<Account<'info, TokenAccount>>,

    /// $WIN contributors airdrop vault account
    #[account(
        mut,
        seeds = [AIRDROP_CONTRIBUTORS.as_ref()],
        bump,
    )]
    pub contributors_account: Box<Account<'info, TokenAccount>>,

    /// $WIN DAO Treasury(10%) vault account
    #[account(
        mut,
        seeds = [AIRDROP_DAO_TREASURY.as_ref()],
        bump,
    )]
    pub dao_treasury_account: Box<Account<'info, TokenAccount>>,

    /// $WIN P2E(40%) vault account
    #[account(
        mut,
        seeds = [AIRDROP_P2E.as_ref()],
        bump,
    )]
    pub p2e_account: Box<Account<'info, TokenAccount>>,

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
        seeds = [VAULT_AUTH.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFromPda<'info> {
    pub fn process(&mut self, args: WithdrawFromPdaArgs) -> Result<()> {
        if args.withdraw_amount == 0 {
            return Err(error!(WinError::InvalidAmount));
        }

        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[VAULT_AUTH.as_ref()], &crate::ID);

        match args.pda_type {
            PdaType::Contributors => {
                if self.contributors_account.amount < args.withdraw_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.contributors_account.to_account_info(),
                            to: self.receive_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.withdraw_amount,
                )?;
            }
            PdaType::Airdrop => {
                if self.airdrop_token_account.amount < args.withdraw_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.airdrop_token_account.to_account_info(),
                            to: self.receive_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.withdraw_amount,
                )?;
            }
            PdaType::Dao => {
                if self.dao_treasury_account.amount < args.withdraw_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.dao_treasury_account.to_account_info(),
                            to: self.receive_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.withdraw_amount,
                )?;
            }
            PdaType::Pte => {
                if self.p2e_account.amount < args.withdraw_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.p2e_account.to_account_info(),
                            to: self.receive_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.withdraw_amount,
                )?;
            }
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WithdrawFromPdaArgs {
    /// withdraw amount
    pub withdraw_amount: u64,

    /// pda type;
    pub pda_type: PdaType,
}
