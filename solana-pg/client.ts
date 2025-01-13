// Client
console.log("My address:", pg.wallet.publicKey.toString());

// Fetch balance
try {
  const balance = await pg.connection.getBalance(pg.wallet.publicKey);
  console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
} catch (error) {
  console.error("Error fetching balance:", error);
}

// Create a transaction
const transaction = new web3.Transaction();

// Add instructions to the transaction
transaction.add(
  new web3.TransactionInstruction({
    keys: [], // Ensure the correct keys are added here
    programId: new web3.PublicKey(pg.PROGRAM_ID), // Validate pg.PROGRAM_ID is a valid public key
  })
);

console.log("Sending Transaction");
try {
  const txHash = await web3.sendAndConfirmTransaction(
    pg.connection,
    transaction,
    [pg.wallet.keypair] // Make sure this is the correct keypair
  );
  console.log("Transaction sent with hash:", txHash);
} catch (error) {
  console.error("Error sending transaction:", error);
}
