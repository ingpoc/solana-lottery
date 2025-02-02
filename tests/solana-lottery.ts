import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaLottery } from "../target/types/solana_lottery";
import { expect } from "chai";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("solana-lottery", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaLottery as Program<SolanaLottery>;
  const authority = Keypair.generate();
  const treasury = Keypair.generate();
  
  before(async () => {
    // Fund authority
    const signature = await provider.connection.requestAirdrop(
      authority.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);
  });

  it("Creates a new lottery", async () => {
    const lottery = Keypair.generate();
    const ticketPrice = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    
    await program.methods
      .createLottery(ticketPrice)
      .accounts({
        lottery: lottery.publicKey,
        authority: authority.publicKey,
        treasury: treasury.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([lottery, authority])
      .rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery.publicKey);
    expect(lotteryAccount.authority.toString()).to.equal(authority.publicKey.toString());
    expect(lotteryAccount.ticketPrice.toString()).to.equal(ticketPrice.toString());
  });

  it("Buys a ticket", async () => {
    const buyer = Keypair.generate();
    const ticket = Keypair.generate();
    
    // Fund buyer
    const signature = await provider.connection.requestAirdrop(
      buyer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);
    
    await program.methods
      .buyTicket()
      .accounts({
        lottery: lottery.publicKey,
        ticket: ticket.publicKey,
        buyer: buyer.publicKey,
        treasury: treasury.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([buyer, ticket])
      .rpc();

    const ticketAccount = await program.account.ticket.fetch(ticket.publicKey);
    expect(ticketAccount.owner.toString()).to.equal(buyer.publicKey.toString());
  });

  it("Schedules a draw", async () => {
    await program.methods
      .scheduleDraw()
      .accounts({
        lottery: lottery.publicKey,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery.publicKey);
    expect(lotteryAccount.status).to.equal("DRAW_SCHEDULED");
  });

  it("Processes VRF callback", async () => {
    const vrf = Keypair.generate();
    
    await program.methods
      .processVrfCallback()
      .accounts({
        lottery: lottery.publicKey,
        vrf: vrf.publicKey,
        authority: authority.publicKey,
      })
      .signers([authority, vrf])
      .rpc();

    const lotteryAccount = await program.account.lottery.fetch(lottery.publicKey);
    expect(lotteryAccount.vrfVerified).to.be.true;
  });

  it("Withdraws from treasury with timelock", async () => {
    // Wait for timelock
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    const destination = Keypair.generate();
    const additionalSigner = Keypair.generate();
    const amount = new anchor.BN(0.05 * anchor.web3.LAMPORTS_PER_SOL);

    await program.methods
      .withdrawTreasury(amount)
      .accounts({
        treasury: treasury.publicKey,
        authority: authority.publicKey,
        destination: destination.publicKey,
        additionalSigner: additionalSigner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority, additionalSigner])
      .rpc();

    const treasuryAccount = await program.account.treasury.fetch(treasury.publicKey);
    expect(treasuryAccount.balance.toString()).to.equal("0");
  });
}); 