use crate::globals::*;
use anchor_lang::prelude::*;

/// Update reward of NFT holders by cron job - TopCollection-Claim-Airdrop
#[derive(Accounts)]
#[instruction(args: UpdateUserRewardArgs)]
pub struct UpdateUserReward<'info> {
    /// bot
    pub bot_wallet: Signer<'info>,

    /// user
    /// CHECK: Safe account
    pub user_wallet: UncheckedAccount<'info>,

    /// UserDetails PDA
    #[account(
        mut,
        seeds = [USER_DETAILS.as_ref(), &user_wallet.key().to_bytes()],
        bump = user_details.bump,
        constraint = user_details.user_wallet == user_wallet.key() @ WinError::AccessDenied,
        constraint = user_details.last_game_distribute_time <= args.last_game_distribute_time @ WinError::AlreadyReceivedGameDistribution
    )]
    pub user_details: Box<Account<'info, UserDetails>>,

    /// Win details PDA
    #[account(
        seeds = [WIN.as_ref()],
        bump = win.bump,
        constraint = win.is_emergency_flag == false @ WinError::EmergencyStatus,
        constraint = win.bot_wallet == bot_wallet.key() @ WinError::AccessDenied
    )]
    pub win: Box<Account<'info, Win>>,
}

impl<'info> UpdateUserReward<'info> {
    pub fn process(&mut self, args: UpdateUserRewardArgs) -> Result<()> {
        if args.reward_amount == 0 {
            return Err(error!(WinError::InvalidAmount));
        }
        match args.reward_type {
            RewardType::ClaimAirdrop => {
                self.user_details.win_airdrop_reward_amount = self
                .user_details
                .win_airdrop_reward_amount
                .checked_add(args.reward_amount)
                .ok_or(WinError::NumericalOverflow)?;
            }
            RewardType::GameRewardAirdrop => {
                self.user_details.win_game_reward_amount = self
                .user_details
                .win_game_reward_amount
                .checked_add(args.reward_amount)
                .ok_or(WinError::NumericalOverflow)?;

                self.user_details.last_game_distribute_time = args.last_game_distribute_time;
            }
            RewardType::DaoAirdrop => {
                self.user_details.win_dao_reward_amount = self
                .user_details
                .win_dao_reward_amount
                .checked_add(args.reward_amount)
                .ok_or(WinError::NumericalOverflow)?;
            }
            RewardType::ContributorsAirdrop => {
                self.user_details.win_contributors_reward_amount = self
                .user_details
                .win_contributors_reward_amount
                .checked_add(args.reward_amount)
                .ok_or(WinError::NumericalOverflow)?;
            }
        }
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateUserRewardArgs {
    /// reward amount
    pub reward_amount: u64,

    /// last game distribute time
    pub last_game_distribute_time: u32,

    /// reward type
    pub reward_type: RewardType
}
