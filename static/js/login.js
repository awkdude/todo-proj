console.log(`Cookies on load: ${document.cookie}`);
const login_form = document.querySelector('form#login');

login_form?.addEventListener('submit', function (event) {
    event.preventDefault();
    const form_data = new FormData(event.target); // event.target);
    const data = Object.fromEntries(form_data.entries());
    fetch('/api/sessions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(data),
    }).then((response) => {
        const error_element = document.querySelector('#login-error');
        error_element.hidden = response.ok;
        if (response.ok) {
            console.log(`Cookies: ${response.headers.getSetCookie()}`);
            // document.cookie = `user`
        } else {
        }
        console.log(response.ok ? 'OK' : 'NO');
    });
});
