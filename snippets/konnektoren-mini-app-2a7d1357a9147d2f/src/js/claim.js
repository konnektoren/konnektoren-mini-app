export async function claim(address, amount) {
  if (!Telegram.WebApp.initDataUnsafe.user) {
    console.error("User is not available");
    return;
  }
  const data = {
    id: Telegram.WebApp.initDataUnsafe.user.id,
    user: Telegram.WebApp.initDataUnsafe.user.username,
    type: "claim",
    address,
    amount,
  };

  Telegram.WebApp.sendData(JSON.stringify(data));
}
