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

const mint1Decimals = 9;
const mint2Decimals = 6;
const fee = 5;
const feeDecimals = 3;
const poolMintDecimals = 9;

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
  const [ mint1Info, mint2Info ] = [ await mint1.getMintInfo(), await mint2.getMintInfo() ];
  const [ firstMintSeed, secondMintSeed ] = (mint1Info.decimals === mint2Info.decimals)
    ? ((mint1.publicKey < mint2.publicKey)
      ? [ mint1.publicKey, mint2.publicKey ]
      : [ mint2.publicKey, mint1.publicKey ])
    : ((mint1Info.decimals < mint2Info.decimals)
      ? [ mint1.publicKey, mint2.publicKey ]
      : [ mint2.publicKey, mint1.publicKey ]);
  
  console.log("mint1 pubkey:", mint1.publicKey.toBase58());
  console.log("mint2 pubkey:", mint2.publicKey.toBase58());
  
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
  // poolMint = Keypair.generate();
  const [poolMint, poolMintBump] = (await PublicKey.findProgramAddress(
    [Buffer.from("chudex_pool_mint"), poolPubkey.toBuffer()],
    programId
  ));
  
  const userPoolTokenAccount = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    poolMint,
    user.publicKey,
    true,
  );

  return {
    mint1,
    mint2,
    userToken1Account,
    userToken2Account,
    poolPubkey,
    boothVault1Pubkey,
    boothVault2Pubkey,
    poolMint,
    userPoolTokenAccount
  }

}

const printTokens = ({
  mint1,
  mint2,
  userToken1Account,
  userToken2Account,
  poolPubkey,
  boothVault1Pubkey,
  boothVault2Pubkey,
  poolMint,
  userPoolTokenAccount
}) => {

  console.log("const mint1Pubkey = new PublicKey(\"" + mint1.publicKey.toBase58() + "\");");
  console.log("const mint2Pubkey = new PublicKey(\"" + mint2.publicKey.toBase58() + "\");");
  console.log("const userToken1AccountPubkey = new PublicKey(\"" + userToken1Account.address.toBase58() + "\");");
  console.log("const userToken2AccountPubkey = new PublicKey(\"" + userToken2Account.address.toBase58() + "\");");
  console.log("const poolPubkey = new PublicKey(\"" + poolPubkey.toBase58() + "\");");
  console.log("const boothVault1Pubkey = new PublicKey(\"" + boothVault1Pubkey.toBase58() + "\");");
  console.log("const boothVault2Pubkey = new PublicKey(\"" + boothVault2Pubkey.toBase58() + "\");");
  console.log("const poolMintPubkey = new PublicKey(\"" + poolMint.toBase58() + "\");");
  console.log("const userPoolTokenAccountPubkey = new PublicKey(\"" + userPoolTokenAccount.toBase58() + "\");");

}

const loadTokens = () => {

  const mint1Pubkey = new PublicKey("BiBni27MdhNyY95P99M1sjVkyVw22ZiXgUvZYkNG6GmT");
  const mint2Pubkey = new PublicKey("3XvwB3YfGFWEW8L2Ux3kwi4pG7nRpdjALAEqctwDHPvM");
  const userToken1AccountPubkey = new PublicKey("Gb48WX13wcmd5F5YtNnDUaYNQTZR8fKnaeDjdrnZnmKY");
  const userToken2AccountPubkey = new PublicKey("UoJxY6ysrJxtDQ9DpZ8WLHy8zbGnpkgi35EQzmSXbem");
  const poolPubkey = new PublicKey("9z4aCtUFSohwTm8vnbytPC7v1k5UKuSjCk7hF1RtT11e");
  const boothVault1Pubkey = new PublicKey("HSMqk8JvCR99XMZu3Ayw24xbCWyXeLjKiwyUJVNDQNJd");
  const boothVault2Pubkey = new PublicKey("99XW8hy3g3Z2G6iiZ4ZVdETUBaysVgo8CtJfswY2wMQm");
  const poolMintPubkey = new PublicKey("2ptfWi36hwpQLHxhSJy1mhKjARVWpW4NJjLwj9YE3GVT");
  const userPoolTokenAccountPubkey = new PublicKey("DLuHvLQ5hCw5qY9ftpstywNwGyGrzvcb7YG5DDERH2rv");

  return {
    mint1Pubkey,
    mint2Pubkey,
    userToken1AccountPubkey,
    userToken2AccountPubkey,
    poolPubkey,
    boothVault1Pubkey,
    boothVault2Pubkey,
    poolMintPubkey,
    userPoolTokenAccountPubkey
  }
}

// init pool
const initPool = async ({
    mint1,
    mint2,
    poolPubkey,
    boothVault1Pubkey,
    boothVault2Pubkey,
    poolMint,
  }) => {

  // Initialize exchange booth
  console.log("Initializing pool...");

  const initIdx = Buffer.from(new Uint8Array([0]));
  const feeBuffer = Buffer.from(new Uint8Array((new BN(fee)).toArray("le", 8)));
  const feeDecimalsBuffer = Buffer.from(new Uint8Array((new BN(feeDecimals)).toArray("le", 8)));

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
        pubkey: poolMint,
        // isSigner: true,
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

}

// send tokens to booth
const deposit = async ({
  userToken1AccountPubkey,
  userToken2AccountPubkey,
  poolPubkey,
  boothVault1Pubkey,
  boothVault2Pubkey,
  poolMintPubkey,
  userPoolTokenAccountPubkey,
  mint1Pubkey,
  mint2Pubkey
}, tokenAAmount, maxTokenBAmount) => {

  console.log("Depositing tokens...");

  const depositIdx = Buffer.from(new Uint8Array([1]));
  const tokenAAmountBuffer = Buffer.from(new Uint8Array((new BN(tokenAAmount)).toArray("le", 8)));
  const maxTokenBAmountBuffer = Buffer.from(new Uint8Array((new BN(maxTokenBAmount)).toArray("le", 8)));

  let depositIx = new TransactionInstruction({
    keys: [
      {
        pubkey: user.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: userToken1AccountPubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: userToken2AccountPubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: userPoolTokenAccountPubkey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: poolPubkey,
        isSigner: false,
        isWritable: false,
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
        pubkey: poolMintPubkey,
        // isSigner: true,
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
      depositIdx,
      tokenAAmountBuffer,
      maxTokenBAmountBuffer,
    ]),
  });

  let depositTx = new Transaction();
  depositTx.add(depositIx);

  let depositTxid = await sendAndConfirmTransaction(
    connection,
    depositTx,
    [user],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      confirmation: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${depositTxid}?cluster=devnet`);

  // check balances
  const userToken1AccountInfo = (await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mint1Pubkey })).value[0].account.data.parsed.info;
  const userToken2AccountInfo = (await connection.getParsedTokenAccountsByOwner(user.publicKey, { mint: mint2Pubkey })).value[0].account.data.parsed.info;
  vault1AccountInfo = (await connection.getParsedTokenAccountsByOwner(poolPubkey, { mint: mint1Pubkey })).value[0].account.data.parsed.info;
  vault2AccountInfo = (await connection.getParsedTokenAccountsByOwner(poolPubkey, { mint: mint2Pubkey })).value[0].account.data.parsed.info;
  console.log("userToken1Account balance:", userToken1AccountInfo.tokenAmount.amount);
  console.log("userToken2ccount balance:", userToken2AccountInfo.tokenAmount.amount);
  console.log("vault1Account balance:", vault1AccountInfo.tokenAmount.amount);
  console.log("vault2Account balance:", vault2AccountInfo.tokenAmount.amount);


}

// exchange

const main = async () => {

  // cli arguments
  var args = process.argv.slice(2);
  const action = parseInt(args[0]);
  // const echo = args[1];
  // const price = parseInt(args[2]);

  if (action === 0) {

    const accounts = await initTokens();
    await initPool(accounts);
    printTokens(accounts);
    return;

  }

  if (action === 1) {
    const accounts = await initTokens();
    await initPool(accounts);  
    return;
  }


  if (action === 2) {

    // load in already initialized accounts
    const accounts = loadTokens();

    await deposit(accounts, 0.01 * (10 ** mint1Decimals), 0.021 * (10 ** mint2Decimals));
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