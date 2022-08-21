use crate::globals::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// Contributors(10%) or Airdrop(5%) token to users (By CSV )
#[derive(Accounts)]
pub struct AirdropToken<'info> {
    /// bot
    pub bot_wallet: Signer<'info>,

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

    /// vault authority
    /// CHECK: Safe account
    #[account(
        seeds = [VAULT_AUTH.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// user wallet
    /// CHECK: Safe account
    pub user_wallet: AccountInfo<'info>,

    /// User $WIN token account
    #[account(
        mut,
        constraint = user_win_ata.mint == win.win_mint_address @ WinError::InvalidTokenMint,
        constraint = user_win_ata.owner == user_wallet.key() @ WinError::InvalidTokenOwner
    )]
    pub user_win_ata: Box<Account<'info, TokenAccount>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
        constraint = win.bot_wallet == bot_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> AirdropToken<'info> {
    pub fn process(&mut self, args: AirdropTokenArgs) -> Result<()> {
        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[VAULT_AUTH.as_ref()], &crate::ID);

        match args.pda_type {
            PdaType::Contributors => {
                if self.contributors_account.amount < args.airdrop_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.contributors_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.airdrop_amount,
                )?;
            }
            PdaType::Airdrop => {
                if self.airdrop_token_account.amount < args.airdrop_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.airdrop_token_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.airdrop_amount,
                )?;
            }
            PdaType::Dao => {
                if self.dao_treasury_account.amount < args.airdrop_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.dao_treasury_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.airdrop_amount,
                )?;
            }
            PdaType::Pte => {
                if self.p2e_account.amount < args.airdrop_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.p2e_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    args.airdrop_amount,
                )?;
            }
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AirdropTokenArgs {
    /// airdrop amount
    pub airdrop_amount: u64,

    /// pda type;
    pub pda_type: PdaType,
}
