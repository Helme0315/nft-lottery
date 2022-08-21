use anchor_lang::prelude::*;

#[cfg(feature = "main-net")]
declare_id!("WNTo1QYGHcGuPaHBoXz74wdmqTnCvdk2oKzyWB48DRE");
#[cfg(feature = "dev-testing")]
declare_id!("WNToqhWt4BQYMxsirBDLSTuyKFgq6nQBvUHha7QygmD");
#[cfg(feature = "local-testing")]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod admin;
pub mod globals;
pub mod lottery;

use admin::*;
use globals::*;
use lottery::*;

#[program]
pub mod winto {
    use super::*;

    ////////////////////////////////////////////////////////////////////////
    /// Admin
    ////////////////////////////////////////////////////////////////////////

    ///////////////////////////////
    /// Backoffice
    ///////////////////////////////

    /// initial win details
    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        let win_bump = *ctx.bumps.get("win").unwrap();
        ctx.accounts.process(win_bump, args)
    }

    /// admin can change fund wallet, wings creator, community fee, emergency flag
    pub fn update_by_admin(
        ctx: Context<UpdateByAdminWallet>,
        args: UpdateByAdminWalletArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// dev wallet can change; bot wallet, wings creator, emergency flag
    pub fn update_by_dev(
        ctx: Context<UpdateByDevWallet>,
        args: UpdateByDevWalletArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// distribute assets by tokenomic
    pub fn assets_distribution(
        ctx: Context<AssetsDistribution>,
        args: AssetsDistributionArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// withdraw PDA token
    pub fn withdraw_pda_token(
        ctx: Context<WithdrawPdaToken>,
        args: WithdrawPdaTokenArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// Withdraw SOL from PDA
    pub fn withdraw_pda_sol(ctx: Context<WithdrawPdaSol>, args: WithdrawPdaSolArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// update merkle whitelist
    pub fn update_merkle_whitelist(
        ctx: Context<UpdateMerkleWhitelist>,
        args: UpdateMerkleWhitelistArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    ///////////////////////////////
    /// Bot
    ///////////////////////////////

    /// airdrop $WIN to user by csv
    pub fn airdrop_token(ctx: Context<AirdropToken>, args: AirdropTokenArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// create user details pda by bot
    pub fn create_user_details_by_bot(ctx: Context<CreateUserDetailsByBot>) -> Result<()> {
        let bump = *ctx.bumps.get("user_details").unwrap();
        ctx.accounts.process(bump)
    }

    /// create user global bid pda SOL
    pub fn create_user_global_bid_pda_sol(ctx: Context<CreateUserGlobalBidPdaSol>, args: CreateUserGlobalBidPdaSolArgs) -> Result<()> {
        let bump = *ctx.bumps.get("user_global_bid").unwrap();
        ctx.accounts.process(bump, args)
    }

    /// create user global bid pda token
    pub fn create_user_global_bid_pda_token(ctx: Context<CreateUserGlobalBidPdaToken>, args: CreateUserGlobalBidPdaTokenArgs) -> Result<()> {
        let bump = *ctx.bumps.get("user_global_bid").unwrap();
        ctx.accounts.process(bump, args)
    }

    /// update user reward
    pub fn update_user_reward(
        ctx: Context<UpdateUserReward>,
        args: UpdateUserRewardArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }

    ////////////////////////////////////////////////////////////////////////
    /// Lottery
    ////////////////////////////////////////////////////////////////////////

    ///////////////////////////////
    /// Organizer
    ///////////////////////////////

    /// organizer create game
    pub fn create_game<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateGame<'info>>,
        args: CreateGameArgs
    ) -> Result<()> {
        let bump = *ctx.bumps.get("game").unwrap();
        let remaining_accounts = ctx.remaining_accounts;
        ctx.accounts.process(bump, args, remaining_accounts)
    }

    /// Lock Wings NFT
    pub fn lock_wings_nft(ctx: Context<LockWingsNft>) -> Result<()> {
        ctx.accounts.process()
    }

    /// Unlock Wings NFT
    pub fn unlock_wings_nft(ctx: Context<UnlockWingsNft>) -> Result<()> {
        ctx.accounts.process()
    }

    /// end game
    pub fn end_game(ctx: Context<EndGame>, args: EndGameArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// organizer get back NFT in cancelled game
    pub fn organizer_get_back_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, OrganizerGetBackNft<'info>>
    ) -> Result<()> {
        let remaining_accounts = ctx.remaining_accounts;
        ctx.accounts.process(remaining_accounts)
    }

    /// organizer withdraw game money(SOL)
    pub fn organizer_process_game_sol(
        ctx: Context<OrganizerProcessGameSol>
    ) -> Result<()> {
        ctx.accounts.process()
    }

    /// organizer withdraw game money(Token)
    pub fn organizer_process_game_token(
        ctx: Context<OrganizerProcessGameToken>,
    ) -> Result<()> {
        ctx.accounts.process()
    }

    /// organizer recreate game
    pub fn recreate_game(ctx: Context<RecreateGame>, args: RecreateGameArgs) -> Result<()> {
        let bump = *ctx.bumps.get("game").unwrap();
        ctx.accounts.process(bump, args)
    }

    ///////////////////////////////
    /// User
    ///////////////////////////////

    /// create user details pda by user
    pub fn create_user_details_by_user(ctx: Context<CreateUserDetailsByUser>) -> Result<()> {
        let bump = *ctx.bumps.get("user_details").unwrap();
        ctx.accounts.process(bump)
    }

    /// User game bid
    pub fn user_game_bid_sol(ctx: Context<UserGameBidSol>, args: UserGameBidSolArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// User game bid
    pub fn user_game_bid_token(ctx: Context<UserGameBidToken>, args: UserGameBidTokenArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// Winner claim NFT
    pub fn winner_claim_nft(ctx: Context<WinnerClaimNft>) -> Result<()> {
        ctx.accounts.process()
    }

    /// user claim airdrop $WIN (5% or 40%)
    pub fn user_claim_airdrop_win(ctx: Context<UserClaimAirdropWin>, args: UserClaimAirdropWinArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// user withdraw funds sol
    pub fn user_withdraw_funds_sol(ctx: Context<UserWithdrawFundsSol>) -> Result<()> {
        ctx.accounts.process()
    }

    /// user withdraw funds token
    pub fn user_withdraw_funds_token(ctx: Context<UserWithdrawFundsToken>) -> Result<()> {
        ctx.accounts.process()
    }

    /// stake freely NFT
    pub fn stake_freely_nft(ctx: Context<StakeFreelyNft>) -> Result<()> {
        ctx.accounts.process()
    }

    /// Unstake freely NFT
    pub fn unstake_freely_nft(ctx: Context<UnstakeFreelyNft>) -> Result<()> {
        ctx.accounts.process()
    }

    /// withdraw $WIN from PDA
    pub fn withdraw_from_pda(
        ctx: Context<WithdrawFromPda>,
        args: WithdrawFromPdaArgs,
    ) -> Result<()> {
        ctx.accounts.process(args)
    }
}
