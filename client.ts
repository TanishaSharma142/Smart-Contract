// Client.ts
// Solpg automatically gives us 'pg' (playground) and 'web3' objects.

console.log("🚀 Starting Test Script...");

// 1. Get the current Program and Wallet from the playground environment
const program = pg.program;
const wallet = pg.wallet;

// 2. Derive the Addresses (PDAs)
// We use the exact same math as the Rust contract to find the addresses
const [vaultStatePda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("state"), wallet.publicKey.toBuffer()],
  program.programId
);

const [vaultAuthPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("auth"), vaultStatePda.toBuffer()],
  program.programId
);

console.log("📍 Vault State Address:", vaultStatePda.toString());
console.log("📍 Vault Auth Address:", vaultAuthPda.toString());

// 3. Main execution function
(async () => {
  try {
    // --- TEST 1: INITIALIZE ---
    console.log("\nTrying to Initialize...");
    const txInit = await program.methods
      .initialize()
      .accounts({
        vaultState: vaultStatePda,
        vaultAuth: vaultAuthPda,
        owner: wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log("✅ Initialize Success! Tx:", txInit);
  } catch (err) {
    // If it fails because it's already there, that's actually a success for us!
    if (err.message.includes("already in use")) {
      console.log("✅ Vault is ALREADY initialized. Moving on...");
    } else {
      console.log("❌ Initialize Failed:", err.message);
    }
  }

  try {
    // --- TEST 2: DEPOSIT ---
    console.log("\nTrying to Deposit 1 SOL...");
    const amount = new anchor.BN(1000000000); // 1 SOL
    
    const txDep = await program.methods
      .deposit(amount)
      .accounts({
        vaultState: vaultStatePda,
        vaultAuth: vaultAuthPda,
        user: wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log("✅ Deposit Success! Tx:", txDep);
    
    // Check balance
    const balance = await pg.connection.getBalance(vaultAuthPda);
    console.log(`💰 Current Vault Balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

  } catch (err) {
    console.log("❌ Deposit Failed:", err.message);
  }
  
})();

