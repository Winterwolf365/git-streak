get_settings(); // initial load

async function get_settings() {
    let settings = await invoke("get_settings");

    document.querySelector("#startup").checked = settings[0];
    document.querySelector("#notifications").checked = settings[1];
    document.querySelector("#all_authors").checked = settings[2];
}

async function set_startup() {
    await invoke('set_startup_setting', { startup: document.querySelector("#startup").checked });
}

async function set_notifications() {
    await invoke('set_notifications_setting', { notifications: document.querySelector("#notifications").checked });
}

async function set_all_authors() {
    await invoke('set_all_authors_setting', { allAuthors: document.querySelector("#all_authors").checked });
    reload();
}
