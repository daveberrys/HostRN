const configPromise = fetch('/config').then(res => res.json());
async function getUrl() {
    const { API_URL } = await configPromise;
    return API_URL;
}

async function fetch_services() {
    try {
        const response = await fetch(`http://${await getUrl()}:3001/get-services`);
        const data = await response.json();
        return data
    } catch (e) {
        return "Could not fetch services: " + e;
    }
}

async function save_service(name, path, command) {
    try {
        if (name === '' || path === '' || command === '') return "Please fill in all fields";
        
        const response = await fetch(`http://${await getUrl()}:3001/save-service`, {
            method: "POST",
            body: JSON.stringify({
                name: name,
                path: path,
                command: command,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });
        location.reload();
        return "Successfully saved " + name
    } catch (e) {
        return "Could not save service: " + e;
    }
}

async function delete_service(uuid, name, path, command) {
    try {
        await fetch(`http://${await getUrl()}:3001/delete-service`, {
            method: "POST",
            body: JSON.stringify({
                uuid: uuid,
                name: name,
                path: path,
                command: command,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });
        location.reload();
        return "Successfully deleted " + name
    } catch (e) {
        return "Could not delete service: " + e;
    }
}

async function start_service(uuid, name, path, command) {
    try {
        await fetch(`http://${await getUrl()}:3001/start-service`, {
            method: "POST",
            body: JSON.stringify({
                uuid: uuid,
                name: name,
                path: path,
                command: command,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });
        return "Started " + name
    } catch (e) {
        return "Could not start service: " + e;
    }
}

async function stop_service(uuid, name, path, command) {
    try {
        await fetch(`http://${await getUrl()}:3001/stop-service`, {
            method: "POST",
            body: JSON.stringify({
                uuid: uuid,
                name: name,
                path: path,
                command: command,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });
        return "Stopped " + name
    } catch (e) {
        return "Could not stop service: " + e;
    }
}

async function check_service(uuid) {
    try {
        const response = await fetch(`http://${await getUrl()}:3001/check-service?uuid=${uuid}`);
        return await response.json();
    } catch (e) {
        return false;
    }
}

export { fetch_services, save_service, delete_service, start_service, stop_service, check_service };