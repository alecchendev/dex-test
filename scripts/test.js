const {
  PublicKey, Keypair,
} = require("@solana/web3.js");

const pubkey1 = new PublicKey("6fyqNvTiYBZfiqEYjfu6buQX44ymC5oFnzNtHcmdhpsu");
const pubkey2 = new PublicKey("qzdcEeD5YBo5ERfaQFk6FeLgjktCrDFpJPFMRi5d8hD");

const pubkey3 = new Keypair();
console.log(pubkey3.publicKey);

// testing public key string comparison

if (pubkey1.toBase58() < pubkey2.toBase58()) {
  console.log(pubkey1.toBase58());
} else {
  console.log(pubkey2.toBase58());
}