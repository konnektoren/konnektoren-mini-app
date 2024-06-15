export function tma_ready() {
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
            typeof Telegram.WebApp.CloudStorage === "undefined" ||
            typeof Telegram.WebApp.CloudStorage.setItem !== "function"
        ) {
            reject(new Error("setItem is undefined"));
            return;
        }
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
        if (
            typeof Telegram.WebApp.CloudStorage === "undefined" ||
            typeof Telegram.WebApp.CloudStorage.getItem !== "function"
        ) {
            reject(new Error("getItem is undefined"));
            return;
        }
        Telegram.WebApp.CloudStorage.getItem(key, (error, value) => {
            if (error) {
                reject(error);
            } else {
                resolve(value);
            }
        });
    });
}
