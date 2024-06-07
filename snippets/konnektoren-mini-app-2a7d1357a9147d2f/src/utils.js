export const tonConnect = new TonConnectSDK.TonConnect();

export async function connectTonWallet() {
  const connector = new TonConnectSDK.TonConnect({
    manifestUrl:
      "https://telegram-mini-app.konnektoren.help/tonconnect-manifest.json",
  });
  console.log("Connecting to wallet");
  await connector.connect();
  return connector.wallet.address;
}
