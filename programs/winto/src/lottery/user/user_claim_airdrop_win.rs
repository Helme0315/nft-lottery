use crate::globals::*;
use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// User claim reward $WIN token
#[derive(Accounts)]
pub struct UserClaimAirdropWin<'info> {
    /// user
    pub user_wallet: Signer<'info>,

    /// bot wallet
    #[account(
        constraint = bot_wallet.key() == win.bot_wallet @ WinError::AccessDenied
    )]
    pub bot_wallet: Signer<'info>,

    /// User $WIN token account
    #[account(
        mut,
        constraint = user_win_ata.mint == win.win_mint_address @ WinError::InvalidTokenMint,
        constraint = user_win_ata.owner == user_wallet.key() @ WinError::InvalidTokenOwner
    )]
    pub user_win_ata: Box<Account<'info, TokenAccount>>,

    /// $WIN airdrop vault account
    #[account(
        mut,
        seeds = [AIRDROP_VAULT.as_ref()],
        bump,
    )]
    pub airdrop_token_account: Box<Account<'info, TokenAccount>>,

    /// $WIN game reward based on participate (40%)
    #[account(
        mut,
        seeds = [AIRDROP_P2E.as_ref()],
        bump,
    )]
    pub game_reward_token_account: Box<Account<'info, TokenAccount>>,

    /// $WIN dao reward pda
    #[account(
        mut,
        seeds = [AIRDROP_DAO_TREASURY.as_ref()],
        bump,
    )]
    pub dao_reward_token_account: Box<Account<'info, TokenAccount>>,

    /// $WIN contirbutors reward pda
    #[account(
        mut,
        seeds = [AIRDROP_CONTRIBUTORS.as_ref()],
        bump,
    )]
    pub contributors_reward_token_account: Box<Account<'info, TokenAccount>>,

    /// vault authority
    /// CHECK: Safe account
    #[account(
        seeds = [VAULT_AUTH.as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// user details pda
    #[account(
        mut,
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump = user_details.bump,
        has_one = user_wallet
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus
    )]
    pub win: Box<Account<'info, Win>>,

    /// token program
    pub token_program: Program<'info, Token>,
}

impl<'info> UserClaimAirdropWin<'info> {
    pub fn process(&mut self, args: UserClaimAirdropWinArgs) -> Result<()> {
        // authority bump seed
        let (_pda, bump_seed) = Pubkey::find_program_address(&[VAULT_AUTH.as_ref()], &crate::ID);
        match args.reward_type {
            RewardType::ClaimAirdrop => {
                // user claim amount
                let claim_amount = self.user_details.win_airdrop_reward_amount;

                if self.airdrop_token_account.amount < claim_amount {
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
                    claim_amount,
                )?;

                self.user_details.win_airdrop_reward_amount = 0;
            }
            RewardType::GameRewardAirdrop => {
                // user claim amount
                let claim_amount = self.user_details.win_game_reward_amount;

                if self.game_reward_token_account.amount < claim_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }

                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.game_reward_token_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    claim_amount,
                )?;

                self.user_details.win_game_reward_amount = 0;
            }
            RewardType::DaoAirdrop => {
                // user claim amount
                let claim_amount = self.user_details.win_dao_reward_amount;

                if self.dao_reward_token_account.amount < claim_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }

                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.dao_reward_token_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    claim_amount,
                )?;

                self.user_details.win_dao_reward_amount = 0;
            }
            RewardType::ContributorsAirdrop => {
                // user claim amount
                let claim_amount = self.user_details.win_contributors_reward_amount;

                if self.contributors_reward_token_account.amount < claim_amount {
                    return Err(error!(WinError::InsufficientTokenBalance));
                }

                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer {
                            from: self.contributors_reward_token_account.to_account_info(),
                            to: self.user_win_ata.to_account_info(),
                            authority: self.vault_authority.to_account_info(),
                        },
                        &[&[VAULT_AUTH.as_ref(), &[bump_seed]]],
                    ),
                    claim_amount,
                )?;

                self.user_details.win_contributors_reward_amount = 0;
            }
        }

        self.user_details.win_last_reward_claimed_date = clock::Clock::get().unwrap().unix_timestamp as u32;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserClaimAirdropWinArgs {
    /// reward type
    pub reward_type: RewardType
}