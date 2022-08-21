import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Win } from '../target/types/win';
import { TOKEN_PROGRAM_ID, Token, ASSOCIATED_TOKEN_PROGRAM_ID, } from '@solana/spl-token';
import * as constant from "./libs/constant";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";

describe('win', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Win as Program<Win>;

  const organizerWallet = anchor.web3.Keypair.generate();
  const adminWalletAddress = anchor.web3.Keypair.generate();
  const nftAuthority = anchor.web3.Keypair.generate();

  let sellNFT = null;
  let organizerNftTokenAccount = null;
  let win_pda = null;
  let user_pda = null;
  let winMintAddress = null;

  it('Init win pda', async () => {
    // Airdrop 2 SOL to payer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(organizerWallet.publicKey, 2000000000),
      "confirmed"
    );

    const tokenMintAddress = new PublicKey("Hdjxx3w6AwEDXBfFyV8t7snP1zVss4NQusV7EyAGaPnf")
    const sale_fee = 5; //  5%

    winMintAddress = await Token.createMint(
      provider.connection,
      organizerWallet,
      nftAuthority.publicKey,
      null,
      6, // Decimal is 6
      TOKEN_PROGRAM_ID,
    );

    win_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.WIN_SEED),
    ], program.programId))[0];

    await program.rpc.initialize(
      {
        botWallet: organizerWallet.publicKey,
        devWallet: organizerWallet.publicKey,
        fundWallet: organizerWallet.publicKey,
        saleFee: sale_fee
      },
      {
        accounts: {
          adminWallet: organizerWallet.publicKey,
          tokenMintAddress: winMintAddress.publicKey,
          win: win_pda,
          systemProgram: SystemProgram.programId,
        },
        signers: [organizerWallet]
      }
    );
  });

  it('User create details pda', async () => {
    user_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.USER_DETAILS_SEED),
      organizerWallet.publicKey.toBuffer()
    ], program.programId))[0];

    await program.rpc.createUserDetailsByUser(
      {
        accounts: {
          userWallet: organizerWallet.publicKey,
          userDetails: user_pda,
          win: win_pda,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [organizerWallet]
      }
    );
  });

  it('Organizer create game!', async () => {
    // Create Wings NFT to sell
    sellNFT = await Token.createMint(
      provider.connection,
      organizerWallet,
      nftAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID,
    );

    // Create token account of organizer
    organizerNftTokenAccount = await sellNFT.createAccount(organizerWallet.publicKey);

    // Create the 1 NFT to user account
    await sellNFT.mintTo(
      organizerNftTokenAccount,
      nftAuthority.publicKey,
      [nftAuthority],
      1
    );

    let cur_time = Math.round((new Date()).getTime() / 1000);

    const [nft_pool_pda, nft_pool_bump] = await PublicKey.findProgramAddress([
      Buffer.from(constant.NFT_POOL_SEED),
    ], program.programId);

    cur_time = new anchor.BN(111);

    const [game_pda, game_bump] = await PublicKey.findProgramAddress([
      Buffer.from(constant.GAME_SEED),
      new anchor.BN(cur_time).toArrayLike(Buffer, "le", 4),
      organizerWallet.publicKey.toBytes(),
      sellNFT.publicKey.toBytes()
    ], program.programId);

    const [nft_pool_ata_pda, nft_pool_ata_bump] = await PublicKey.findProgramAddress([
      organizerWallet.publicKey.toBuffer(),
      sellNFT.publicKey.toBuffer(),
      game_pda.toBuffer()
    ], program.programId);
    console.log(nft_pool_ata_pda.toString());

    await program.rpc.createGame(
      {
        ticketPrice: new anchor.BN(10000000), // 0.01SOL
        minimumCost: new anchor.BN(20000000), // 0.02SOL
        gameTimeStamp: new anchor.BN(cur_time),
        duration: new anchor.BN(3600), // 1 hour
        coinType: 0, //0 - SOL, 1 - WIN
      },
      {
        accounts: {
          organizerWallet: organizerWallet.publicKey,
          nftMintAddress: sellNFT.publicKey,
          organizerNftAta: organizerNftTokenAccount,
          nftPool: nft_pool_pda,
          nftPoolAta: nft_pool_ata_pda,
          game: game_pda,
          win: win_pda,
          userDetails: user_pda,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [organizerWallet]
      }
    );

  });
});
