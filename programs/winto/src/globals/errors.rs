use anchor_lang::prelude::*;

#[error_code]
pub enum WinError {
    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    #[msg("Invalid token mint")]
    InvalidTokenMint,

    #[msg("Access denied")]
    AccessDenied,

    #[msg("Insufficient token balance")]
    InsufficientTokenBalance,

    #[msg("Insufficient SOL balance")]
    InsufficientSolBalance,

    #[msg("Invalid argument")]
    InvalidArgs,

    #[msg("Invalid whitelist")]
    InvalidProof,

    #[msg("We are under maintenance")]
    EmergencyStatus,

    #[msg("Invalid Account")]
    InvalidAccount,

    #[msg("Game is not opened status")]
    GameIsNotOpenedStatus,

    #[msg("Game is not closed status")]
    GameIsNotClosedStatus,

    #[msg("Game is not cancelled status")]
    GameIsNotCancelledStatus,

    #[msg("NFT has already been claimed")]
    NftAlreadyClaimed,

    #[msg("Game is in progressing now")]
    GameIsInProgress,

    #[msg("Game has been ended already")]
    AlreadyGameEnd,

    #[msg("Insufficient game money")]
    InsufficientGameMoney,

    #[msg("Please lock Wings NFT or you should be whitelist user to create game")]
    UnableToCreateGame,

    #[msg("Funds have been claimed already")]
    ClaimedAlready,

    #[msg("Ticket price or minimum cost should be bigger than zero")]
    WrongVaule,

    #[msg("Invalid Metadata Account")]
    WrongMetadata,

    #[msg("The NFT is not wings NFT")]
    NoWingsNft,

    #[msg("Numerical Overflow Error")]
    NumericalOverflow,

    #[msg("PublicKey is mismatched")]
    PublicKeyMismatch,

    #[msg("Incorrect Owner")]
    IncorrectOwner,

    #[msg("Uninitialized Account")]
    UninitializedAccount,

    #[msg("The NFT is not freely ticket NFT")]
    NoFreelyTicketNft,

    #[msg("You can not unstake freely ticket NFT now")]
    NoUnstakeFreelyTicketNft,

    #[msg("Ticket amount cannot be zero")]
    NoTicketAmount,

    #[msg("You already claimed NFT")]
    NoGameNft,

    #[msg("Already rewarded")]
    AlreadyReceivedGameDistribution,

    #[msg("Invalid Amount")]
    InvalidAmount,

    #[msg("Invalid bonus ticket amount")]
    InvalidBonusTicketAmount,
}
