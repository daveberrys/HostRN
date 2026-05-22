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
        console.error(e);
        return null;
    }
}

async function save_service(name, path, command) {
    try {
        if (name === '' || path === '' || command === '') return;
        
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
        const data = await response.json();
        location.reload();
        return data
    } catch (e) {
        console.error(e);
        return null;
    }
}

async function delete_service(uuid, name, path, command) {
    try {
        const response = await fetch(`http://${await getUrl()}:3001/delete-service`, {
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
        const data = await response.json();
        location.reload();
        return data
    } catch (e) {
        console.error(e);
        return null;
    }
}

async function start_service(uuid, name, path, command) {
    try {
        const response = await fetch(`http://${await getUrl()}:3001/start-service`, {
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
        const data = await response.json();
        return data
    } catch (e) {
        console.error(e);
        return null;
    }
}

async function stop_service(uuid, name, path, command) {
    try {
        const response = await fetch(`http://${await getUrl()}:3001/stop-service`, {
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
        const data = await response.json();
        return data
    } catch (e) {
        console.error(e);
        return null;
    }
}

export { fetch_services, save_service, delete_service, start_service, stop_service };