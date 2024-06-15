export function tma_ready() {
  console.log(Telegram.WebApp.version);
  return Telegram.WebApp.ready();
}

export function tma_version() {
  return Telegram.WebApp.version;
}

export function tma_set_header_color(color) {
  return Telegram.WebApp.setHeaderColor(color);
}

export function tma_set_item(key, value) {
  return new Promise((resolve, reject) => {
    if (
        typeof Telegram.WebApp.cloudStorage === "undefined" ||
        typeof Telegram.WebApp.cloudStorage.setItem !== "function"
    ) {
      reject(new Error("setItem is undefined"));
      return;
    }
    Telegram.WebApp.cloudStorage.setItem(key, value, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}

export function tma_get_item(key) {
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
