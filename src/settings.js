get_settings(); // initial load

async function get_settings() {
    let settings = await invoke("get_settings");

    document.querySelector("#startup").checked = settings[0];
    document.querySelector("#notifications").checked = settings[1];
}

async function set_startup() {
    await invoke('set_startup_setting', { startup: document.querySelector("#startup").checked });
}

async function set_notifications() {
    await invoke('set_notifications_setting', { notifications: document.querySelector("#notifications").checked });
}