import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CnftVault } from "../target/types/cnft_vault";
import { loadWalletKey, decode, mapProof } from "./utils";
import { IDL } from "../target/types/cnft_vault"
import {PROGRAM_ID as BUBBLEGUM_PROGRAM_ID} from "@metaplex-foundation/mpl-bubblegum";
import { SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, SPL_NOOP_PROGRAM_ID} from "@solana/spl-account-compression";
import { getAsset, getAssetProof } from "./readAPI";


const c = new anchor.web3.Connection("https://api.devnet.solana.com");
const kp = loadWalletKey("../../AndYPfCmbSSHpe2yukLXDT9N29twa7kJDk3yrRMQW7SN.json");
const w = new anchor.Wallet(kp);
const provider = new anchor.AnchorProvider(c, w, {});
const programID = new anchor.web3.PublicKey("CNftyK7T8udPwYRzZUMWzbh79rKrz9a5GwV2wv7iEHpk")
const program = new Program<CnftVault>(IDL, programID, provider);
async function main(){
    const [vaultPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("cNFT-vault", "utf8")],
        programID,
      );
    
    const tree = new anchor.web3.PublicKey("trezdkTFPKyj4gE9LAJYPpxn8AYVCvM7Mc4JkTb9X5B")

    const receiver = new anchor.web3.PublicKey("Andys9wuoMdUeRiZLgRS5aJwYNFv4Ut6qQi8PNDTAPEM")
    
    const [treeAuthority, _bump2] = anchor.web3.PublicKey.findProgramAddressSync(
        [tree.toBuffer()],
        BUBBLEGUM_PROGRAM_ID,
      );

    const assetId = "DGWU3mHenDerCvjkeDsKYEbsvXbWvqdo1bVoXy3dkeTd";
    const asset = await getAsset(assetId);
    // console.log(res)

    const proof = await getAssetProof(assetId);
    const proofPathAsAccounts = mapProof(proof);

    const root = decode(proof.root);
    const dataHash = decode(asset.compression.data_hash);
    const creatorHash = decode(asset.compression.creator_hash);
    const nonce = asset.compression.leaf_id;
    const index = asset.compression.leaf_id;

    const tx = await program.methods.withdrawCnft(root, dataHash, creatorHash, nonce, index)
    .accounts({
        leafOwner: vaultPDA,
        leafDelegate: vaultPDA,
        merkleTree: tree,
        newLeafOwner: receiver,
        treeAuthority: treeAuthority,
        bubblegumProgram: BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log(tx);
};

main();