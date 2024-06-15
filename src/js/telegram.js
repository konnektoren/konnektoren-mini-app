export function tma_ready() {
  return Telegram.WebApp.ready();
}

export function tma_version() {
  return Telegram.WebApp.version;
}

export function tma_set_header_color(color) {
  return Telegram.WebApp.setHeaderColor(color);
}

export function tma_setItem(key, value, callback) {
  if (typeof Telegram.WebApp.cloudStorage.setItem !== "function") {
    callback({ message: "setItem is undefined" }, null);
    return;
  }
  Telegram.WebApp.cloudStorage.setItem(key, value, callback);
}

export function tma_getItem(key) {
  return new Promise((resolve, reject) => {
    if (
      typeof Telegram.WebApp.cloudStorage === "undefined" ||
      typeof Telegram.WebApp.cloudStorage.getItem !== "function"
    ) {
      reject(new Error("getItem is undefined"));
      return;
    }
    Telegram.WebApp.cloudStorage.getItem(key, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}
