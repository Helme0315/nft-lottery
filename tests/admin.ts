import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Win } from '../target/types/win';
import { TOKEN_PROGRAM_ID, Token, ASSOCIATED_TOKEN_PROGRAM_ID, } from '@solana/spl-token';
import * as constant from "./libs/constant";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import invariant from "tiny-invariant";
import { BalanceTree } from "./libs/balance-tree";

describe('win', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Win as Program<Win>;

  const adminWalletAddress = anchor.web3.Keypair.generate();
  const botWalletAddress = anchor.web3.Keypair.generate();
  const devWalletAddress = anchor.web3.Keypair.generate();
  const fundWalletAddress = anchor.web3.Keypair.generate();
  const nftAuthority = anchor.web3.Keypair.generate();
  const sale_fee = 50; // 5%

  let winMintAddress = null;
  let adminWinAta = null;
  let win_pda = null;
  let airdrop_pda = null;
  let dao_treasury_pda = null;
  let contributor_pda = null;
  let p2e_pda = null;
  let game_win_pda = null;
  let vault_auth_pda = null;
  let merkle_pda = null;
  let leaves: { account: PublicKey }[] = [];
  let tree = null;
  let merkle_hash = null;
  let nftArray = [];

  it('Init all variables', async () => {
    // Airdrop 2 SOL to payer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(adminWalletAddress.publicKey, 2000000000),
      "confirmed"
    );

    winMintAddress = await Token.createMint(
      provider.connection,
      adminWalletAddress,
      nftAuthority.publicKey,
      null,
      6, // Decimal is 6
      TOKEN_PROGRAM_ID,
    );

    adminWinAta = (await PublicKey.findProgramAddress(
      [
        adminWalletAddress.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        winMintAddress.publicKey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    ))[0];

    let instr = Token.createAssociatedTokenAccountInstruction(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      winMintAddress.publicKey,
      adminWinAta,
      adminWalletAddress.publicKey,
      provider.wallet.publicKey
    );

    let createTrns = new anchor.web3.Transaction().add(instr);
    await provider.send(createTrns);

    await winMintAddress.mintTo(
      adminWinAta,
      nftAuthority.publicKey,
      [nftAuthority],
      1000000000
    );
  });

  it('Initialize Win details', async () => {
    win_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.WIN_SEED),
    ], program.programId))[0];

    await program.rpc.initialize(
      {
        botWallet: botWalletAddress.publicKey,
        devWallet: devWalletAddress.publicKey,
        fundWallet: fundWalletAddress.publicKey,
        saleFee: sale_fee
      },
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          tokenMintAddress: winMintAddress.publicKey,
          win: win_pda,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [adminWalletAddress]
      }
    );

  });

  it('Update emergency flag', async () => {
    let emergency = false;
    await program.rpc.updateEmegencyFlag(
      emergency,
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          win: win_pda
        },
        signers: [adminWalletAddress]
      }
    );
  });

  it('Initial PDA for fund $WIN', async () => {
    airdrop_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.AIRDROP_VAULT_SEED),
    ], program.programId))[0];

    dao_treasury_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.DAO_TREASURY_SEED),
    ], program.programId))[0];

    contributor_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.CONTRIBUTORS_SEED),
    ], program.programId))[0];

    p2e_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.P2E_SEED),
    ], program.programId))[0];

    game_win_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.WIN_POOL_SEED),
    ], program.programId))[0];

    vault_auth_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.AIRDROP_AUTH_SEED),
    ], program.programId))[0];

    await program.rpc.initializePda(
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          tokenMint: winMintAddress.publicKey,
          airdropTokenAccount: airdrop_pda,
          daoTreasuryAccount: dao_treasury_pda,
          contributorAccount: contributor_pda,
          p2eAccount: p2e_pda,
          GameWinPool: game_win_pda,
          vaultAuthority: vault_auth_pda,
          win: win_pda,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [adminWalletAddress]
      }
    );
  });

  it('Fund $WIN to airdrop', async () => {
    // set Airdrop PDA(5%) address
    let tokenAirdropAmount = 50000000;
    let vaultAccountAddress = airdrop_pda

    await program.rpc.assetsDistribution(
      new anchor.BN(tokenAirdropAmount),
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          adminWinAta: adminWinAta,
          vaultTokenAccount: vaultAccountAddress,
          win: win_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [adminWalletAddress]
      }
    );

    // set DAO Treasury PDA(10%) address
    tokenAirdropAmount = 100000000;
    vaultAccountAddress = dao_treasury_pda

    await program.rpc.assetsDistribution(
      new anchor.BN(tokenAirdropAmount),
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          adminWinAta: adminWinAta,
          vaultTokenAccount: vaultAccountAddress,
          win: win_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [adminWalletAddress]
      }
    );

    // set Contributors PDA(10%) address
    tokenAirdropAmount = 100000000;
    vaultAccountAddress = contributor_pda

    await program.rpc.assetsDistribution(
      new anchor.BN(tokenAirdropAmount),
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          adminWinAta: adminWinAta,
          vaultTokenAccount: vaultAccountAddress,
          win: win_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [adminWalletAddress]
      }
    );

    // set P2E PDA(40%) address
    tokenAirdropAmount = 400000000;
    vaultAccountAddress = p2e_pda

    await program.rpc.assetsDistribution(
      new anchor.BN(tokenAirdropAmount),
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          adminWinAta: adminWinAta,
          vaultTokenAccount: vaultAccountAddress,
          win: win_pda,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [adminWalletAddress]
      }
    );
  });

  it('Withdraw $WIN from PDA to target wallet', async () => {
    const tokenWithdrawAmount = 10;
    let vaultAccountAddress = airdrop_pda;
    let _win_info = await program.account.win.fetch(win_pda);

    const tokenMintAddress = new PublicKey(_win_info.winMintAddress);
    const owner = new PublicKey(_win_info.fundWallet);

    const withdraw_ata = (await PublicKey.findProgramAddress(
      [
        owner.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        tokenMintAddress.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    ))[0];

    let _ata_info = await program.provider.connection.getAccountInfo(withdraw_ata);
    let instr = null;
    if (!_ata_info) {
      instr = Token.createAssociatedTokenAccountInstruction(
        ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID,
        tokenMintAddress,
        withdraw_ata,
        owner,
        provider.wallet.publicKey
      );
    }

    const [vault_auth_pda, vault_auth_bump] = await PublicKey.findProgramAddress([
      Buffer.from(constant.AIRDROP_AUTH_SEED),
    ], program.programId);

    if (instr == null) {
      await program.rpc.withdrawPdaToken(
        new anchor.BN(tokenWithdrawAmount),
        {
          accounts: {
            devWallet: devWalletAddress.publicKey,
            receiveWinAta: withdraw_ata,
            vaultTokenAccount: vaultAccountAddress,
            win: win_pda,
            vaultAuthority: vault_auth_pda,
            tokenProgram: TOKEN_PROGRAM_ID,
          },
          signers: [devWalletAddress]
        }
      );
    } else {

      await program.rpc.withdrawPdaToken(
        new anchor.BN(tokenWithdrawAmount),
        {
          accounts: {
            devWallet: devWalletAddress.publicKey,
            receiveWinAta: withdraw_ata,
            vaultTokenAccount: vaultAccountAddress,
            win: win_pda,
            vaultAuthority: vault_auth_pda,
            tokenProgram: TOKEN_PROGRAM_ID,
          },
          instructions: [
            instr
          ],
          signers: [devWalletAddress]
        }
      );
    }
  });

  it('Withdraw $SOL from PDA to target wallet', async () => {
    let _win_info = await program.account.win.fetch(win_pda);
    const fundWallet = new PublicKey(_win_info.fundWallet);
    let communityPda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.COMMUNITY_SEED),
    ], program.programId))[0];

    const solWithdrawAmount = 10000000;

    await program.rpc.withdrawPdaSol(
      new anchor.BN(solWithdrawAmount),
      {
        accounts: {
          devWallet: devWalletAddress.publicKey,
          fundWallet: fundWallet,
          vaultTokenAccount: communityPda,
          win: win_pda,
          systemProgram: SystemProgram.programId,
        },
        signers: [devWalletAddress]
      }
    );
  });

  it('intial merkle tree', async () => {
    merkle_pda = (await PublicKey.findProgramAddress([
      Buffer.from(constant.MERKLE_SEED),
    ], program.programId))[0];

    let nftList = [
      "BfnPHQtCzy3ZVnPbWAPwyqEjZofZGirCACZcHrenHSJi",
      "XJruJeFYHbvRPFVgEQkxbowZyF6nxznDLEscx7Wxs4D",
      "365sECzzHZy3hcojKdZXsrffURZgcqBdsLMCakj2zym2",
      "2ERSbbmiCuh9rwcjS44dCQ5jvCQh43n5QRxSX3JoMJ91",
      "8spWVK1GJuDBQjHmwZjjCP9T7GUpqwNX9CEtrUXCN343",
      "CJBvJuLdk5357UjcVkN8pP2ECXHtEc7wGkikoh8h7VHf",
      "8aVnHKnedtLAxABDvBggZvUwwTQrEK75gb2h3rutKXoi",
      "Do9D3NaXe833EA6VHNnw2KtBtiZJJeA8fpHo8WKdcCdJ",
      "JDAZoJFrgjKAN4vJtPdG8dzp1YAZw8PXqMDQUpKRFhcL",
      "HfRtbmpw1SH8nheQmMxj1bWRnqbQ4b78GmaEaY4ozhed"
    ];
    for (var i = 0; i < nftList.length; i++) {
      nftArray.push({ account: new PublicKey(nftList[i]) })
    }

    nftArray.map(x => leaves.push(x));
    tree = new BalanceTree(leaves);
    merkle_hash = tree.getRoot();

    await program.rpc.initializeMerkle(
      toBytes32Array(merkle_hash),
      {
        accounts: {
          adminWallet: adminWalletAddress.publicKey,
          merkle: merkle_pda,
          win: win_pda,
          systemProgram: SystemProgram.programId,
        },
        signers: [adminWalletAddress]
      }
    );
  });
});

const toBytes32Array = (b: Buffer): number[] => {
  invariant(b.length <= 32, `invalid length ${b.length}`);
  const buf = Buffer.alloc(32);
  b.copy(buf, 32 - b.length);
  return Array.from(buf);
};
