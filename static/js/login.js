import { setUserSessionInfo } from '/js/util.js';

const login_form = document.querySelector('form#login');

login_form?.addEventListener('submit', async function (event) {
    event.preventDefault();
    const form_data = new FormData(event.target); // event.target);
    const login_data = Object.fromEntries(form_data.entries());
    let response = await fetch('/api/sessions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(login_data),
    });
    const error_element = document.querySelector('#login-error');
    error_element.hidden = response.ok;
    if (response.ok) {
        const data = await response.json();
        setUserSessionInfo(data.user_id);
        window.location.href = data.redirect;
    } else {
    }
    console.log(response.ok ? 'OK' : 'NO');
});

setUserSessionInfo(null);
