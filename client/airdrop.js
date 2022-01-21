const {
  Connection,
  clusterApiUrl,
} = require("@solana/web3.js");

// keys
const { mintAuthority, user } = require("./keys.js");

// connection
const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed',
);

const airDrop = async () => {
  
  console.log("Airdropping mintAuthority 2 SOL...");
  const airdropMintAuthorityTx = await connection.requestAirdrop(mintAuthority.publicKey, 2e9);
  await connection.confirmTransaction(airdropMintAuthorityTx, "finalized");
  console.log("Done");

  console.log("Airdropping user 2 SOL...");
  const airdropUserTx = await connection.requestAirdrop(user.publicKey, 2e9);
  await connection.confirmTransaction(airdropUserTx, "finalized");
  console.log("Done");
}

airDrop()
.then(() => {
  console.log("Success");
})
.catch((e) => {
  console.error(e);
});