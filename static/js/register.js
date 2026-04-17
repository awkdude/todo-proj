import { setUserSessionInfo } from '/js/util.js';

const test_button = document.querySelector('button#test');
test_button?.addEventListener('click', (event) => {
    fetch('', { method: 'POST' })
        .then((response) => {
            console.log('No error');
            throw new Error();
        })
        .catch((error) => {
            console.error(`ERROR: ${error}`);
        });
});
const MINIMUM_PASSWORD_LENGTH = 4;
const form = document.querySelector('form');
form?.addEventListener('submit', async function (event) {
    event.preventDefault();
    console.log(`Action: ${event.target.action}`);
    const form_data = new FormData(form); // event.target);
    const register_data = Object.fromEntries(form_data.entries());
    for (let [k, v] of form_data.entries()) {
        console.log(`Form data: ${k}: ${v}`);
    }
    let response = fetch(event.target.action, {
        method: 'POST', // event.target.method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(register_data),
    })
        .then(async (response) => {
            if (response.ok) {
                console.log('User created!');
                const data = await response.json();
                setUserSessionInfo(data.user_id);
                window.location.href = data.redirect;
            } else {
                console.error('ERROR');
            }
        })
        .catch((reason) => {
            console.error(`FAIL: ${reason}`);
        });
    let stuff_element = document.querySelector('p#stuff');
});
let password_element = document.querySelector('#password');
let confirm_password_element = document.querySelector('#confirm-password');
// FIXME:
password_element.minLength = MINIMUM_PASSWORD_LENGTH;
password_element.addEventListener('input', (event) => {
    document.querySelector('#password-comment').innerHTML =
        event.target.value.length < MINIMUM_PASSWORD_LENGTH
            ? '❌This is too short!'
            : '✅Good password';
});
confirm_password_element.addEventListener('input', (event) => {
    document.querySelector('#confirm-password-comment').innerHTML =
        event.target.value === password_element.value
            ? '✅Password matches'
            : "❌Password does't match";
});
