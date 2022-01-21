const {
  Connection,
  clusterApiUrl,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
} = require("@solana/web3.js");
const {
  Token,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} = require("@solana/spl-token");
const { BN } = require("bn.js");

const { mintAuthority, user } = require("./keys.js");
const { pubkeyCmp } = require("./utils.js");

const mint1Decimals = 9;
const mint2Decimals = 6;
const fee = 5;
const feeDecimals = 3;

const programId = new PublicKey("G4QQ465gehN97upZxMh1Z4GWi347nhi9cuoxVRDdUTZf");

// Connect to cluster
const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed',
);

const initTokens = async () => {

  // create/get mints
  console.log("Creating mint1...");
  const mint1 = await Token.createMint(
    connection,
    mintAuthority,
    mintAuthority.publicKey,
    mintAuthority.publicKey,
    mint1Decimals,
    TOKEN_PROGRAM_ID,
  );

  console.log("Creating mint2...");
  const mint2 = await Token.createMint(
    connection,
    mintAuthority,
    mintAuthority.publicKey,
    mintAuthority.publicKey,
    mint2Decimals,
    TOKEN_PROGRAM_ID,
  );

  // create/get user token accounts
  console.log("Getting/creating user token accounts...");
  const userToken1Account = await mint1.getOrCreateAssociatedAccountInfo(
    user.publicKey,
  );
  const userToken2Account = await mint2.getOrCreateAssociatedAccountInfo(
    user.publicKey,
  );

  // mint tokens to user
  console.log("Minting tokens to user...");
  await mint1.mintTo(
    userToken1Account.address,
    mintAuthority.publicKey,
    [],
    5 * (10 ** mint1Decimals),
  )
  await mint2.mintTo(
    userToken2Account.address,
    mintAuthority.publicKey,
    [],
    5 * (10 ** mint2Decimals),
  )

  // PDAs
  // exchange booth pda
  console.log("Getting exchange booth PDA...");
  const [ firstMintSeed, secondMintSeed ] = ((mint1.publicKey == pubkeyCmp(mint1.publicKey, mint2.publicKey)) ? 
    [ mint1.publicKey, mint2.publicKey ] :
    [ mint2.publicKey, mint1.publicKey ]);
  
  console.log("firstMintSeed:", firstMintSeed.toBase58());
  console.log("secondMintSeed:", secondMintSeed.toBase58());
    
  const [poolPubkey, poolBump] = (await PublicKey.findProgramAddress(
    [Buffer.from("chudex_pool"), firstMintSeed.toBuffer(), secondMintSeed.toBuffer()],
    programId
  ));

  // exchange booth token accounts
  console.log("Getting exchange booth token accounts...");
  
  const boothVault1Pubkey = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint1.publicKey,
    poolPubkey,
    true,
  );
  const boothVault2Pubkey = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint2.publicKey,
    poolPubkey,
    true,
  );

  // pool mint
  poolMint = Keypair.generate();

  return {
    mint1,
    mint2,
    userToken1Account,
    userToken2Account,
    poolPubkey,
    boothVault1Pubkey,
    boothVault2Pubkey,
    poolMint
  }

}

// init booth

// send tokens to booth

// exchange

const main = async () => {

  // cli arguments
  var args = process.argv.slice(2);
  const action = parseInt(args[0]);
  // const echo = args[1];
  // const price = parseInt(args[2]);

  const {
    mint1,
    mint2,
    userToken1Account,
    userToken2Account,
    poolPubkey,
    boothVault1Pubkey,
    boothVault2Pubkey,
    poolMint,
  } = await initTokens();

  if (action === 0) {
    return;
  }

  // TODO print balances of different tokens for each account

  // Initialize exchange booth
  console.log("Initializing pool...");

  const initIdx = Buffer.from(new Uint8Array([0]));
  const feeBuffer = Buffer.from(new Uint8Array((new BN(fee)).toArray("le", 8)));
  const feeDecimalsBuffer = Buffer.from(new Uint8Array((new BN(feeDecimals)).toArray("le", 1)));

  let initIx = new TransactionInstruction({
    keys: [
      {
        pubkey: user.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: poolPubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: boothVault1Pubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: boothVault2Pubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint1.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: mint2.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: poolMint.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SYSVAR_RENT_PUBKEY,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      initIdx,
      feeBuffer,
      feeDecimalsBuffer,
    ]),
  });

  let initTx = new Transaction();
  initTx.add(initIx);

  let initTxid = await sendAndConfirmTransaction(
    connection,
    initTx,
    [user],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${initTxid}?cluster=devnet`);

  data = (await connection.getAccountInfo(poolPubkey)).data;
  console.log("Init Buffer Text:", data);

  if (action === 1) {
    return;
  }

  // Send tokens from admin to vaults
  console.log("Sending tokens from admin to vaults...");

  const transferIdx = Buffer.from(new Uint8Array([3]));
  const transferAmount1 = Buffer.from(new Uint8Array( (new BN(4 * (10 ** mint1Decimals))).toArray("le", 8)) );
  const sendToken1Ix = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    adminToken1Account.address,
    boothVault1Pubkey,
    admin.publicKey,
    [],
    4 * (10 ** mint1Decimals)
  );

  const transferAmount2 = Buffer.from( new Uint8Array((new BN(4 * (10 ** mint2Decimals))).toArray("le", 8)) );
  const sendToken2Ix = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    adminToken2Account.address,
    boothVault2Pubkey,
    admin.publicKey,
    [],
    4 * (10 ** mint2Decimals)
  );

  // make the tx
  let sendTx = new Transaction();
  sendTx.add(sendToken1Ix).add(sendToken2Ix);

  // send tx
  let sendTxid = await sendAndConfirmTransaction(
    connection,
    sendTx,
    [admin],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );

  // tx url on devnet
  console.log(`https://explorer.solana.com/tx/${sendTxid}?cluster=devnet`);

  // check token accounts
  const adminToken1AccountInfo = (await connection.getParsedTokenAccountsByOwner(admin.publicKey, { mint: mint1.publicKey })).value[0].account.data.parsed.info;
  const adminToken2AccountInfo = (await connection.getParsedTokenAccountsByOwner(admin.publicKey, { mint: mint2.publicKey })).value[0].account.data.parsed.info;
  let vault1AccountInfo = (await connection.getParsedTokenAccountsByOwner(exchangeBoothPubkey, { mint: mint1.publicKey })).value[0].account.data.parsed.info;
  let vault2AccountInfo = (await connection.getParsedTokenAccountsByOwner(exchangeBoothPubkey, { mint: mint2.publicKey })).value[0].account.data.parsed.info;
  // console.log(adminToken1AccountInfo, adminToken2AccountInfo, vault1AccountInfo, vault2AccountInfo);
  console.log("adminToken1Account balance:", adminToken1AccountInfo.tokenAmount.amount);
  console.log("adminToken2ccount balance:", adminToken2AccountInfo.tokenAmount.amount);
  console.log("vault1Account balance:", vault1AccountInfo.tokenAmount.amount);
  console.log("vault2Account balance:", vault2AccountInfo.tokenAmount.amount);


  // Exchange
  console.log("Exchanging token1 for token2...");

  // ix
  const exchangeIdx = Buffer.from(new Uint8Array([3]));
  const swapAmount = Buffer.from( new Uint8Array( (new BN(3 * (10 ** mint2Decimals))).toArray("le", 8) ) );
  let exchangeIx = new TransactionInstruction({
    keys: [
      { pubkey: exchangeBoothPubkey, isSigner: false, isWritable: false },
      { pubkey: boothVault1Pubkey, isSigner: false, isWritable: true },
      { pubkey: boothVault2Pubkey, isSigner: false, isWritable: true },
      { pubkey: user.publicKey, isSigner: true, isWritable: false },
      { pubkey: userToken2Account.address, isSigner: false, isWritable: true },
      { pubkey: userToken1Account.address, isSigner: false, isWritable: true },
      { pubkey: mint1.publicKey, isSigner: false, isWritable: false },
      { pubkey: mint2.publicKey, isSigner: false, isWritable: false },
      { pubkey: oraclePubkey, isSigner: false, isWritable: false },
      { pubkey: adminToken2Account.address, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: exchangeBoothProgramId,
    data: Buffer.concat([exchangeIdx, swapAmount]),
  });
  
  // make the tx
  let exchangeTx = new Transaction();
  exchangeTx.add(exchangeIx);

  // send tx
  let exchangeTxid = await sendAndConfirmTransaction(
    connection,
    exchangeTx,
    [user],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );

  // tx url on devnet
  console.log(`https://explorer.solana.com/tx/${exchangeTxid}?cluster=devnet`);

  // check balances
  const userToken1AccountInfo = (await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mint1.publicKey })).value[0].account.data.parsed.info;
  const userToken2AccountInfo = (await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mint2.publicKey })).value[0].account.data.parsed.info;
  vault1AccountInfo = (await connection.getParsedTokenAccountsByOwner(exchangeBoothPubkey, { mint: mint1.publicKey })).value[0].account.data.parsed.info;
  vault2AccountInfo = (await connection.getParsedTokenAccountsByOwner(exchangeBoothPubkey, { mint: mint2.publicKey })).value[0].account.data.parsed.info;
  console.log("userToken1Account balance:", userToken1AccountInfo.tokenAmount.amount);
  console.log("userToken2ccount balance:", userToken2AccountInfo.tokenAmount.amount);
  console.log("vault1Account balance:", vault1AccountInfo.tokenAmount.amount);
  console.log("vault2Account balance:", vault2AccountInfo.tokenAmount.amount);
}

main()
.then(() => {
  console.log("Success");
})
.catch((e) => {
  console.error(e);
});