const {
    Connection,
    sendAndConfirmTransaction,
    Keypair,
    Transaction,
    SystemProgram,
    PublicKey,
    TransactionInstruction,
} = require("@solana/web3.js");

const BN = require("bn.js");



const main = async () => {
    var args = process.argv.slice(2);
    // args[0]: Program ID
    // args[1] (Optional): Manager buffer account
    const programId = new PublicKey(args[0]);

    console.log(programId.toBase58());
    const connection = new Connection("https://api.devnet.solana.com/");
    const feePayer = new Keypair();

    console.log("Requesting Airdrop of 1 SOL...");
    await connection.requestAirdrop(feePayer.publicKey, 2e9);
    console.log("Airdrop received");

    const manager = new Keypair();
    const lottery = new Keypair();
    let managerKey = manager.publicKey;
    let lotteryKey = lottery.publicKey;
    let tx = new Transaction();
    let signers = [feePayer];
    if (args.length > 1) {
        console.log("Found manger address");
        managerKey = new PublicKey(args[1]);
    } else {
        console.log("Generating new manager address");
        let createIx1 = SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey,
            newAccountPubkey: managerKey,
            /** Amount of lamports to transfer to the created account */
            lamports: await connection.getMinimumBalanceForRentExemption(32),
            /** Amount of space in bytes to allocate to the created account */
            space: 32,
            /** Public key of the program to assign as the owner of the created account */
            programId: programId,
        });
        signers.push(manager);
        tx.add(createIx1);

        console.log("Generating new Lottery address");
        let createIx2 = SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey,
            newAccountPubkey: lotteryKey,
            /** Amount of lamports to transfer to the created account */
            lamports: await connection.getMinimumBalanceForRentExemption(16),
            /** Amount of space in bytes to allocate to the created account */
            space: 16,
            /** Public key of the program to assign as the owner of the created account */
            programId: programId,
        });
        signers.push(lottery);
        tx.add(createIx2);
    }

    const idx = Buffer.from(new Uint8Array([0]));
    let incrIx = new TransactionInstruction({
        keys: [
            {
                pubkey: managerKey,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: lotteryKey,
                isSigner: false,
                isWritable: true,
            },
        ],
        programId: programId,
        data: idx,
    });
    tx.add(incrIx);


    let txid = await sendAndConfirmTransaction(connection, tx, signers, {
        skipPreflight: true,
        preflightCommitment: "confirmed",
        confirmation: "confirmed",
    });
    console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

    data = (await connection.getAccountInfo(managerKey)).data;
    managerData = new BN(data, "le");
    console.log("Manager Key:", managerKey.toBase58());
    console.log("Lottery Key: ", lotteryKey.toBase58());
};

main()
    .then(() => {
        console.log("Success");
    })
    .catch((e) => {
        console.error(e);
    });
