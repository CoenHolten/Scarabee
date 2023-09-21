function createFormHeaders() {
    const headers = new Headers();
    headers.append('Content-Type', 'application/x-www-form-urlencoded');

    return headers;
}

async function createUser(name, password, email, phone) {
    const form = new URLSearchParams();
    form.append('name', name);
    form.append('password', password);
    form.append('email', email);
    form.append('phone', phone);

    return fetch('/api/user_new', {
        method: 'PUT',
        headers: createFormHeaders(),
        body: form,
    }).then(() => {
        return null;
    });
}

async function getUser(name) {
    return fetch(`/api/user/${name}`, {
        method: 'GET',
    }).then((response) => {
        return response.json();
    });
}

async function login(name, password) {
    const form = new URLSearchParams();
    form.append('name', name);
    form.append('password', password);

    return fetch('/api/user_login', {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then(() => {
        return null;
    });
}

async function logout() {
    return fetch('/api/user_logout', {
        method: 'GET',
    }).then(() => {
        return null;
    });
}

// Returns all commitments.
async function getCommitments() {
    const form = new URLSearchParams();

    return fetch('/api/commitment_search', {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then((response) => {
        if (!response.ok) {
            return [];
        }

        return response.json();
    });
}

async function getCommitment(id) {
    return fetch(`/api/commitment/${id}`, {
        method: 'GET',
    }).then((response) => {
        return response.json();
    });
}

async function addCommitment(name, description) {
    const form = new URLSearchParams();
    form.append('name', name);
    form.append('description', description);

    return fetch('/api/commitment_new', {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then((response) => {
        return response;
    });
}

async function getInitiatives(commitmentName) {
    const form = new URLSearchParams();
    form.append('commitment', commitmentName);

    return fetch('/api/initiative_search', {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then((response) => {
        return response.json();
    });
}

async function getInitiative(id) {
    return fetch(`/api/initiative/${id}`, {
        method: 'GET',
    }).then((response) => {
        return response.json();
    });
}

async function addInitiative(commitmentName, name, description) {
    const form = new URLSearchParams();
    form.append('commitment_name', commitmentName);
    form.append('name', name);
    form.append('description', description);

    return fetch(`/api/initiative_new`, {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then((response) => {
        return response;
    });
}

async function getSupports(commitmentName, initiativeName) {
    const form = new URLSearchParams();
    form.append('commitment', commitmentName);
    form.append('initiative', initiativeName);

    return fetch('/api/support_search', {
        method: 'POST',
        headers: createFormHeaders(),
        body: form,
    }).then((response) => {
        return response.json();
    });
}

async function addSupport(initiativeName) {
    return fetch(`/api/support_add/${initiativeName}`, {
        method: 'PUT',
    });
}

async function removeSupport(initiativeName) {
    return fetch(`/api/support_remove/${initiativeName}`, {
        // TODO: should this be DELETE?
        method: 'PUT',
    });
}

export default {
    getUser,
    createUser,
    login,
    logout,

    addCommitment,
    getCommitments,
    getCommitment,

    getInitiatives,
    getInitiative,
    addInitiative,

    getSupports,
    addSupport,
    removeSupport,
};
