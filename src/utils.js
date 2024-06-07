//import TonConnectSDK from "https://cdn.jsdelivr.net/npm/@tonconnect/sdk@3.0.3/+esm";
const USE_TEST_NETWORK = true;
const KONNEKTOREN_COIN = "EQCP0BHV18JPMrt0JbSUulzcL4geZ_JrGgMZmbVv8gBk2iTe";
const TON_API_URL = "https://testnet.tonapi.io/v2";

let tonConnectUI;

export async function connectWallet(onConnectCallback) {
  try {
    tonConnectUI = new TON_CONNECT_UI.TonConnectUI({
      manifestUrl:
        "https://telegram-mini-app.konnektoren.help/tonconnect-manifest.json",
      buttonRootId: "wallet-button",
    });

    tonConnectUI.onStatusChange(async (wallet) => {
      try {
        console.log("Wallet status changed:", wallet);
        const rawAddress = wallet.account.address;
        const address = TonConnectSDK.toUserFriendlyAddress(
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
  const jettonAddress = KONNEKTOREN_COIN;
  const response = await fetch(
    `${TON_API_URL}/accounts/${address}/jettons/${jettonAddress}`,
  );
  const data = await response.json();
  return data.balance;
}
