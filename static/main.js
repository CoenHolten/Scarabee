import api from './api.js';

function listenToFormSubmit(element, onSubmit) {
    element.addEventListener('submit', async (event) => {
        event.preventDefault();

        const form = event.target;
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        const data = new FormData(form);
        const props = Object.fromEntries(data);
        form.reset();

        await onSubmit(props);
    });
}

function createElement(spec) {
    const [elementName, attributes, ...childSpecs] = spec;
    const element = document.createElement(elementName);

    for (const [key, value] of Object.entries(attributes || {})) {
        element.setAttribute(key, value);
    }

    for (const childSpec of childSpecs) {
        if (Array.isArray(childSpec)) {
            element.appendChild(createElement(childSpec));
        } else {
            element.appendChild(document.createTextNode(childSpec));
        }
    }

    return element;
}

function createInitiativeCard(initiativeName, initiative, users) {
    const initiativeElement = createElement([
        'div',
        {
            class: 'initiative-card',
        },
        ['h4', {}, initiative.commitment_name],
        ['p', {}, initiative.description],
        ['h5', {}, 'Supported by:'],
    ]);

    for (const user of users) {
        initiativeElement.appendChild(
            createElement([
                'div',
                {
                    class: 'supporter',
                },
                ['span', {}, user.name],
                [
                    'a',
                    {
                        href: `mailto:${user.email}`,
                    },
                    user.email,
                ],
                [
                    'a',
                    {
                        href: `tel:${user.phone}`,
                    },
                    user.phone,
                ],
            ]),
        );
    }

    const addSupportButton = initiativeElement.appendChild(
        createElement(['button', {}, 'Add my support']),
    );
    addSupportButton.addEventListener('click', async () => {
        await api.addSupport(initiativeName);

        await loadCommitments();
    });

    const removeSupportButton = initiativeElement.appendChild(
        createElement(['button', {}, 'Remove my support']),
    );
    removeSupportButton.addEventListener('click', async () => {
        await api.removeSupport(initiativeName);

        await loadCommitments();
    });

    return initiativeElement;
}

function createAddInitiativeForm(commitmentName) {
    const addInitiativeElement = createElement([
        'details',
        {
            class: 'add-initiative',
        },
        ['summary', {}, 'Add initiative'],
        [
            'form',
            {
                class: 'form',
            },
            [
                'label',
                {
                    class: 'form__row',
                },
                'Name',
                [
                    'input',
                    {
                        type: 'text',
                        name: 'name',
                        required: '',
                    },
                ],
            ],
            [
                'label',
                {
                    class: 'form__row',
                },
                'Description',
                [
                    'input',
                    {
                        type: 'text',
                        name: 'description',
                        required: '',
                    },
                ],
            ],
            [
                'div',
                {
                    class: 'form__row',
                },
                [
                    'input',
                    {
                        type: 'submit',
                        value: 'Create initiative',
                    },
                ],
            ],
        ],
    ]);

    listenToFormSubmit(
        addInitiativeElement.querySelector('form'),
        async (props) => {
            api.addInitiative(commitmentName, props.name, props.description);

            await loadCommitments();
        },
    );

    return addInitiativeElement;
}

// TODO: text is not sanitized. This needs to happen to prevent XSS attacks.
async function loadCommitments() {
    let commitmentNames = [];
    try {
        commitmentNames = await api.getCommitments();
    } catch (error) {
        console.error(error);
    }

    const commitmentsElement = document.querySelector('#commitments');
    commitmentsElement.textContent = '';

    for (const commitmentName of commitmentNames) {
        const commitment = await api.getCommitment(commitmentName);

        const element = commitmentsElement.appendChild(
            createElement([
                'div',
                {
                    class: 'commitment-card',
                },
                ['h3', {}, commitment.name],
                ['p', {}, commitment.description],
            ]),
        );

        const initiativeNames = await api.getInitiatives(commitmentName);

        // Add each initiative.
        for (const initiativeName of initiativeNames) {
            const initiative = await api.getInitiative(initiativeName);
            const supports = await api.getSupports(
                commitmentName,
                initiativeName,
            );
            const users = await Promise.all(
                supports.map(async (support) => await api.getUser(support)),
            );

            element.appendChild(
                createInitiativeCard(initiativeName, initiative, users),
            );
        }

        element.appendChild(createAddInitiativeForm(commitmentName));
    }
}

listenToFormSubmit(document.querySelector('#user-sign-up'), async (props) => {
    await api.createUser(props.name, props.password, props.email, props.phone);

    await loadCommitments();
});

listenToFormSubmit(document.querySelector('#user-login'), async (props) => {
    await api.login(props.name, props.password);

    await loadCommitments();
});

listenToFormSubmit(document.querySelector('#new-commitment'), async (props) => {
    await api.addCommitment(props.name, props.description);

    await loadCommitments();
});

document.querySelector('#user-logout').addEventListener('click', async () => {
    await api.logout();

    await loadCommitments();
});

await loadCommitments();
