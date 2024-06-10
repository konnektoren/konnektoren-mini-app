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

  console.log("Claiming data", data);

  try {
    const response = await fetch(
      "https://tg-api.konnektoren.help/api/v1/claim",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      },
    );

    if (!response.ok) {
      throw new Error("Network response was not ok");
    }

    const result = await response.json();

    if (result.success) {
      Telegram.WebApp.showPopup(
        {
          title: "Claimed successfully",
          message: "Success! View transaction",
          buttons: [
            { id: "link", type: "default", text: "Open Explorer" },
            { type: "cancel" },
          ],
        },
        function (btn) {
          if (btn === "link") {
            Telegram.WebApp.openLink(result.explorer_url);
          }
        },
      );
      return Promise.resolve(result.explorer_url);
    } else {
      alert("Claim failed.");
      return Promise.reject(result);
    }
  } catch (error) {
    console.error("Error:", error);
    alert("Error occurred while claiming.");
    return Promise.reject(error);
  }
}
