export function tma_ready() {
  return Telegram.WebApp.ready();
}

export function tma_version() {
  return Telegram.WebApp.version;
}

export function tma_get_user_id() {
  if (typeof Telegram.WebApp.initDataUnsafe.user === "undefined") {
    throw new Error("User ID is undefined");
  }
  return Telegram.WebApp.initDataUnsafe.user.id.toString();
}

export function tma_get_user_name() {
  if (typeof Telegram.WebApp.initDataUnsafe.user === "undefined") {
    throw new Error("User name is undefined");
  }
  return Telegram.WebApp.initDataUnsafe.user.username;
}

export function tma_set_header_color(color) {
  return Telegram.WebApp.setHeaderColor(color);
}

export function tma_set_item(key, value) {
  return new Promise((resolve, reject) => {
    // Check if running in Telegram Mini App environment
    if (
      typeof Telegram === "undefined" ||
      typeof Telegram.WebApp === "undefined" ||
      typeof Telegram.WebApp.CloudStorage === "undefined" ||
      typeof Telegram.WebApp.CloudStorage.setItem !== "function"
    ) {
      // Fallback to localStorage when not in Telegram
      console.warn("Using localStorage fallback for Telegram CloudStorage");
      try {
        localStorage.setItem(`tg_${key}`, value);
        resolve(true);
      } catch (err) {
        reject(new Error("Storage not available"));
      }
      return;
    }

    // Use Telegram CloudStorage when available
    Telegram.WebApp.CloudStorage.setItem(key, value, (error, value) => {
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
    // Check if running in Telegram Mini App environment
    if (
      typeof Telegram === "undefined" ||
      typeof Telegram.WebApp === "undefined" ||
      typeof Telegram.WebApp.CloudStorage === "undefined" ||
      typeof Telegram.WebApp.CloudStorage.getItem !== "function"
    ) {
      // Fallback to localStorage when not in Telegram
      console.warn("Using localStorage fallback for Telegram CloudStorage");
      try {
        const value = localStorage.getItem(`tg_${key}`);
        resolve(value !== null ? value : "");
      } catch (err) {
        reject(new Error("Storage not available"));
      }
      return;
    }

    // Use Telegram CloudStorage when available
    Telegram.WebApp.CloudStorage.getItem(key, (error, value) => {
      if (error) {
        reject(error);
      } else {
        resolve(value);
      }
    });
  });
}
