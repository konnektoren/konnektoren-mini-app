import {toUserFriendlyAddress} from "https://esm.run/@tonconnect/sdk";
import {TonConnectUI} from "https://esm.run/@tonconnect/ui";

const USE_TEST_NETWORK = true;
const KONNEKTOREN_COIN = "EQCP0BHV18JPMrt0JbSUulzcL4geZ_JrGgMZmbVv8gBk2iTe";
const TON_API_URL = "https://testnet.tonapi.io/v2";

let tonConnectUI;

export async function connectWallet(onConnectCallback) {
    try {
        tonConnectUI = new TonConnectUI({
            manifestUrl:
                "https://telegram-mini-app.konnektoren.help/tonconnect-manifest.json",
            buttonRootId: "wallet-button",
        });

        tonConnectUI.onStatusChange(async (wallet) => {
            try {
                if (!wallet || !wallet.account || !wallet.account.address) {
                    onConnectCallback("", "0");
                    return;
                }
                const rawAddress = wallet.account.address;
                const address = toUserFriendlyAddress(
                    rawAddress,
                    USE_TEST_NETWORK,
                );

                const balance = await getJettonBalance(rawAddress);

                onConnectCallback(address, balance);
            } catch (innerError) {
                console.error("Error in onStatusChange callback:", innerError);
            }
        });
    } catch (outerError) {
        console.error("Error in connectWallet:", outerError);
    }
}

export async function getJettonBalance(address) {
    const response = await fetch(
        `${TON_API_URL}/accounts/${address}/jettons/${KONNEKTOREN_COIN}`,
    );
    const data = await response.json();
    return data.balance;
}

export async function sendRawTransaction(rawTxBase64, destinationAddress) {
    try {
        const response = await tonConnectUI.sendTransaction({
            validUntil: Math.floor(new Date() / 1000) + 360,
            messages: [
                {
                    address: destinationAddress,
                    payload: rawTxBase64,
                    amount: 1_100_000_000
                },
            ],
        });
    } catch (error) {
        console.error("Error sending transaction:", error);
        throw error;
    }
}
