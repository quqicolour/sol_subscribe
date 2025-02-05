import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { web3 } from "@project-serum/anchor";
import type { Library } from "../target/types/library";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Library as anchor.Program<Library>;
  
  let owner = pg.wallet;
  let manager = new web3.Keypair();
  let feeReceiver = new web3.Keypair();
  let newFeeReceiver1 = new web3.Keypair();
  console.log("owner:", owner.publicKey.toBase58());
  console.log("manager:", manager.publicKey.toBase58());
  console.log("feeReceiver:", feeReceiver.publicKey.toBase58());
  console.log("newFeeReceiver1:", newFeeReceiver1.publicKey.toBase58());

  let ProgramId = web3.SystemProgram.programId;
  console.log("ProgramId:", ProgramId.toBase58());

  it("All", async () => {
    let [managementAuthority, managementBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("subscribe_management")],
        program.programId
      );
    console.log("managementAuthority:", managementAuthority.toString());
    console.log("managementBump:", managementBump);

    let [productTypeAuthority, productTypeBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("subscribe_type")],
        program.programId
      );
    console.log("productTypeAuthority:", productTypeAuthority.toString());
    console.log("productTypeBump:", productTypeBump);

    let [userInfoAuthority, userInfoBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("subscribe_user_info"), owner.publicKey.toBuffer()],
        program.programId
      );
    console.log("userInfoAuthority:", userInfoAuthority.toString());
    console.log("userInfoBump:", userInfoBump);

    async function Initialize() {
      try {
        const initializeTx = await program.methods
          .initialize(
            owner.publicKey,
            manager.publicKey,
            feeReceiver.publicKey
          )
          .accounts({
            management: managementAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`initializeTx: ${initializeTx}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initializeTx);

        // Fetch the created account
        const managementAccount = await program.account.management.fetch(
          managementAuthority
        );
        console.log("managementAccount:", managementAccount);
      } catch (e) {
        console.log("initialize fail:", e);
      }
    }
    // await Initialize();

    //transfer owner
    async function TransferOwner(newOwner: PublicKey) {
      try {
        const transferOwnerTx = await program.methods.transferOwner(newOwner)
          .accounts({
            management: managementAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`transferOwnerTx: ${transferOwnerTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(transferOwnerTx);
      } catch (e) {
        console.log("TransferOwner fail:", e);
      }
    }
    await TransferOwner(owner.publicKey);

    //transfer manager
    async function TransferManager(newManager: PublicKey) {
      try {
        const transferManagerTx = await program.methods.transferManager(newManager)
          .accounts({
            management: managementAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`transferManagerTx: ${transferManagerTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(transferManagerTx);
      } catch (e) {
        console.log("TransferManager fail:", e);
      }
    }
    await TransferManager(owner.publicKey);

    //set fee receiver
    async function SetFeeReceiver(newFeeReceiver: PublicKey) {
      try {
        const setFeeReceiverTx = await program.methods.setFeeReceiver(newFeeReceiver)
          .accounts({
            management: managementAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`setFeeReceiverTx: ${setFeeReceiverTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setFeeReceiverTx);
      } catch (e) {
        console.log("SetFeeReceiver fail:", e);
      }
    }

    //Init product type
    async function InitProductType(subscribe_type: number, fee: BN, validTime: BN) {
      try {
        const initProductTypeTx = await program.methods.initProductType(subscribe_type, fee, validTime)
          .accounts({
            management: managementAuthority,
            mappingProductType: productTypeAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`initProductTypeTx: ${initProductTypeTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initProductTypeTx);
      } catch (e) {
        console.log("InitProductType fail:", e);
      }
    }
    let fee = new anchor.BN(1_000_000);
    let days_30 = new anchor.BN(259_2000);
    // await InitProductType(0, fee, days_30);

    //set product type
    async function SetProductType(subscribe_type: number, fee: BN, validTime: BN) {
      try {
        const setProductTypeTx = await program.methods.setProductType(subscribe_type, fee, validTime)
          .accounts({
            management: managementAuthority,
            mappingProductType: productTypeAuthority,
            signer: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`setProductTypeTx: ${setProductTypeTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setProductTypeTx);
      } catch (e) {
        console.log("InitProductType fail:", e);
      }
    }
    let new_fee = new anchor.BN(1_000_000);
    let new_days_30 = new anchor.BN(259_2000);
    await SetProductType(0, new_fee, new_days_30);

    await SetFeeReceiver(feeReceiver.publicKey);

    //register buy
    async function RegisterBuy(subscribe_type: number, copies: number) {
      try {
        const registerBuyTx = await program.methods.registerBuy(subscribe_type, copies)
          .accounts({
            management: managementAuthority,
            mappingProductType: productTypeAuthority,
            mappingUserInfo: userInfoAuthority,
            feeReceiver: feeReceiver.publicKey,
            user: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`registerBuyTx: ${registerBuyTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(registerBuyTx);
      } catch (e) {
        console.log("RegisterBuy fail:", e);
      }
    }
    // await RegisterBuy(0, 1);

    
    await SetFeeReceiver(newFeeReceiver1.publicKey);

    //buy
    async function Buy(subscribe_type: number, copies: number) {
      try {
        const buyTx = await program.methods.buy(subscribe_type, copies)
          .accounts({
            management: managementAuthority,
            mappingProductType: productTypeAuthority,
            mappingUserInfo: userInfoAuthority,
            feeReceiver: newFeeReceiver1.publicKey,
            user: owner.publicKey,
            systemProgram: ProgramId,
          })
          .signers([owner.keypair])
          .rpc();
        console.log(`buyTx: ${buyTx}`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(buyTx);
      } catch (e) {
        console.log("Buy fail:", e);
      }
    }
    await Buy(0, 2);

  });
});
