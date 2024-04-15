get_settings(); // initial load

async function get_settings() {
    let settings = await invoke("get_settings");

    document.querySelector("#startup").checked = settings[0];
    document.querySelector("#notifications").checked = settings[1];
    document.querySelector("#all_authors").checked = settings[2];
}

async function set_startup() {
    await invoke('set_setting', { setting: "startup", value: document.querySelector("#startup").checked });
}

async function set_notifications() {
    await invoke('set_setting', { setting: "notifications", value: document.querySelector("#notifications").checked });
}

async function set_all_authors() {
    await invoke('set_setting', { setting: "all_authors", value: document.querySelector("#all_authors").checked });
    reload();
}
