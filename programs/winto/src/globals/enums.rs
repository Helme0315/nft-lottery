use anchor_lang::prelude::*;

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CoinType {
    SOL = 0,
    TOKEN = 1,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum FundsStatus {
    NotClaimed = 0,
    Withdrawed = 1,
    Retransfer = 2
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RewardType {
    ClaimAirdrop = 0,
    GameRewardAirdrop = 1,
    DaoAirdrop = 2,
    ContributorsAirdrop = 3
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PdaType {
    Contributors = 0,
    Airdrop = 1,
    Dao = 2,
    Pte = 3,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum WingsType {
    None = 0,
    Gold = 1,
    Silver = 2,
    Bronze = 3,
}