import { getConsumerClient, getConsumerWallet } from './clients';
import {  gatewayHookMemo, sendIBCToken } from './ibc';
import { loadContractConfig, loadIbcConfig } from './config';
import dotenv from 'dotenv';
dotenv.config();

let CONSUMER_TOKEN = process.env.CONSUMER_TOKEN;

const instantiateContract = async () => {

    const signingWallet = await getConsumerWallet();
    const signingClient = await getConsumerClient(signingWallet);
    const signerAddress = (await signingWallet.getAccounts())[0].address;

    const ibcConfig = loadIbcConfig();
    const secretGateway = loadContractConfig().gateway!;


    const responseSimple = await sendIBCToken(
        signingClient,
        signerAddress,
        secretGateway.address,
        CONSUMER_TOKEN!,
        "1",
        ibcConfig.consumer_channel_id,
        gatewayHookMemo(
            { extension: { msg: { instantiate: { 
                code_id: 2123, 
                code_hash: "138c4984186458a1ea7887dd271295f8f2547fc381d8991c780fdd2eb2f6ee73"
             } } }},
            secretGateway
        )
    )

    console.log("Instantiate IBC Hook Response:", responseSimple);


}

instantiateContract();


