import { u64 } from "@saberhq/token-utils";
import type { PublicKey } from "@solana/web3.js";
import type BN from "bn.js";
import { keccak_256 } from "js-sha3";

import { MerkleTree } from './libs/merkle-tree';

export class BalanceTree {
  private readonly _tree: MerkleTree;
  constructor(balances: { account: PublicKey; }[]) {
    this._tree = new MerkleTree(
      balances.map(({ account }, index) => {
        return BalanceTree.toNode(account);
      })
    );
  }

  static verifyProof(
    account: PublicKey,
    proof: Buffer[],
    root: Buffer
  ): boolean {
    let pair = BalanceTree.toNode(account);
    for (const item of proof) {
      pair = MerkleTree.combinedHash(pair, item);
    }

    return pair.equals(root);
  }

  // keccak256(abi.encode(index, account, amount))
  static toNode(account: PublicKey): Buffer {
    const buf = Buffer.concat([
      Buffer.from("winto-wings-nft-merkle"),
      account.toBuffer(),
    ]);
    return Buffer.from(keccak_256(buf), "hex");
  }

  getHexRoot(): string {
    return this._tree.getHexRoot();
  }

  // returns the hex bytes32 values of the proof
  getHexProof(account: PublicKey): string[] {
    return this._tree.getHexProof(BalanceTree.toNode(account));
  }

  getRoot(): Buffer {
    return this._tree.getRoot();
  }

  getProof(account: PublicKey): Buffer[] {
    return this._tree.getProof(BalanceTree.toNode(account));
  }
}
