import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  Connection,
  Commitment,
} from "@solana/web3.js";
import { assert } from "chai";
import * as snarkjs from "snarkjs";
import path from "path";
import { buildBn128, utils } from "ffjavascript";
import { parseProofToBytesArray, parseToBytesArray } from "../src/utils";
const { unstringifyBigInts } = utils;

import { IDL } from "./zk_user_verification";

const wasmPath = "tests/bday.wasm";
const zkeyPath = "tests/bday_0001.zkey";

// Define the file name and access type
const fileName = "CSFSDF545";
const accessType = "read";

function g1Uncompressed(curve, p1Raw) {
  let p1 = curve.G1.fromObject(p1Raw);

  let buff = new Uint8Array(64); // 64 bytes for G1 uncompressed
  curve.G1.toRprUncompressed(buff, 0, p1);

  return Buffer.from(buff);
}

function g2Uncompressed(curve, p2Raw) {
  let p2 = curve.G2.fromObject(p2Raw);

  let buff = new Uint8Array(128); // 128 bytes for G2 uncompressed
  curve.G2.toRprUncompressed(buff, 0, p2);

  return Buffer.from(buff);
}

function to32ByteBuffer(bigInt) {
  const hexString = bigInt.toString(16).padStart(64, "0"); // Pad to 64 hex characters (32 bytes)
  const buffer = Buffer.from(hexString, "hex");
  return buffer;
}

describe("zk_user_verification", () => {
  // Configure the client to use the local cluster.
  const commitment: Commitment = "confirmed";
  const connection = new Connection("https://api.devnet.solana.com", {
    commitment,
    // wsEndpoint: "wss://api.devnet.solana.com/",
  });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  // CAUTTION: if you are intended to use the program that is deployed by yourself,
  // please make sure that the programIDs are consistent
  const programId = new PublicKey(
    "27hymAk2PCtdYHhMiwwEYiBvs2gbYZeyjNA7x1nT54Hw"
  );
  const program = new anchor.Program(IDL, programId, provider);

  it("Initialize Main State", async () => {
    const admin = provider.wallet;

    // Derive the PDA for the company state
    const [mainInfoPda, mainInfoBump] = await PublicKey.findProgramAddress(
      [Buffer.from("maininfo"), admin.publicKey.toBuffer()],
      program.programId
    );

    console.log(mainInfoPda);

    // Initialize the company
    // await program.methods
    //   .initializeMainInfo(accessType, fileName)
    //   .accounts({
    //     mainInfo: mainInfoPda,
    //     admin: admin.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .signers([])
    //   .rpc();

    // Fetch the company account
    const mainInfoAccount = await program.account.mainInfo.fetch(mainInfoPda);
    // assert.equal(mainInfoAccount.fileName, fileName);
    // assert.equal(mainInfoAccount.accessType, accessType);
    console.log(mainInfoAccount);
  });


  it("Set Allowed Time Range", async () => {
    const admin = provider.wallet;

    // Derive the PDA for the company state
    const [mainInfoPda, mainInfoBump] = await PublicKey.findProgramAddress(
      [Buffer.from("maininfo"), admin.publicKey.toBuffer()],
      program.programId
    );

    console.log("MainInfo PDA:", mainInfoPda.toBase58());

    // Define the new allowed time range
    const newStartHour = 10; // 10 AM UTC
    const newEndHour = 12; // 8 PM UTC

    // Call the setAllowedTimeRange function
    await program.methods
      .setAllowedTimeRange(newStartHour, newEndHour)
      .accounts({
        mainInfo: mainInfoPda,
        admin: admin.publicKey,
      })
      .rpc();

    // Fetch the updated company account
    const updatedMainInfoAccount = await program.account.mainInfo.fetch(
      mainInfoPda
    );

    // Assert the updated time range
    assert.equal(updatedMainInfoAccount.startHour, newStartHour);
    assert.equal(updatedMainInfoAccount.endHour, newEndHour);

    console.log("Updated Time Range:", {
      startHour: updatedMainInfoAccount.startHour,
      endHour: updatedMainInfoAccount.endHour,
    });
  });

  it("Verify CEO", async () => {
    // let input = {
    //   name: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   role: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   birthday: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //   companyName: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    // };
    // let { proof, publicSignals } = await snarkjs.groth16.fullProve(
    //   input,
    //   wasmPath,
    //   zkeyPath
    // );

    const proof = {
      pi_a: [
        "20920484545824152768219349738343510607158410503126077592472764520138252279016",
        "16824256373575500513475155716659457654976325184303372848952184577867821922452",
        "1",
      ],
      pi_b: [
        [
          "19813440401321065736652884152611074958308449904973199973522391512162144633309",
          "402600029144489764568829976673844079282480516937508862729049824806179051256",
        ],
        [
          "14190804147780252444981521397771935103012423917555308796784146707179447623265",
          "1818833764210069006520801744369119451117122789509610147626581124328904286187",
        ],
        ["1", "0"],
      ],
      pi_c: [
        "16355866557349470780983632567598425040149198945977416032229926831089284931440",
        "13622008620868569352558427195194991479683304263576106757415539531550203357967",
        "1",
      ],
      protocol: "groth16",
      curve: "bn128",
    };

    const publicSignals = [
      "12190118236461756001683974151677789172651763991449464096001596654482483488634",
      "3112626621302619800867442836869779507811667491815112894613862090609351071117",
    ];

    const parsed_proof = parseProofToBytesArray(JSON.stringify(proof));
    const parsed_public_signal = parseToBytesArray(publicSignals);

    let final_public_signal = [];
    for (let i = 0; i < parsed_public_signal.length; i++) {
      final_public_signal = final_public_signal.concat(parsed_public_signal[i]);
    }

    const admin = provider.wallet;

    // Derive the PDA for the company state
    const [mainInfoPda, mainInfoBump] = await PublicKey.findProgramAddress(
      [Buffer.from("maininfo"), admin.publicKey.toBuffer()],
      program.programId
    );

    console.log(mainInfoPda);

    // Generate a new keypair for the user
    const user = provider.wallet;

    const proofInfo = {
      proofA: parsed_proof.proofA,
      proofB: parsed_proof.proofB,
      proofC: parsed_proof.proofC,
      publicSignals: final_public_signal,
    };

    // Call the verify_ceo function
    await program.methods
      .verifyUser(proofInfo, accessType, fileName)
      .accounts({
        mainInfo: mainInfoPda,
        user: user.publicKey,
      })
      .rpc();

    // Fetch the company account
    const companyAccount = await program.account.mainInfo.fetch(mainInfoPda);
    console.log(companyAccount);
  });
});
