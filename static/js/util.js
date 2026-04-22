export async function setUserSessionInfo(user_id) {
    const user_info = await fetch(`/api/users/${user_id}`).then((response) =>
        response.json()
    );
    user_info.toString = function () {
        return JSON.stringify(this);
    };
    // sessionStorage = user_info;
    Object.assign(sessionStorage, user_info);
    console.log(`User info: ${JSON.stringify(sessionStorage)}`);
}

export function dateTimeFromJSDate(date_obj) {
    const year = `${date_obj.getFullYear()}`.padStart(4, '0');
    const month = `${date_obj.getMonth() + 1}`.padStart(2, '0');
    const date = `${date_obj.getDate()}`.padStart(2, '0');
    const hour = `${date_obj.getHours()}`.padStart(2, '0');
    const minute = `${date_obj.getMinutes()}`.padStart(2, '0');
    return {
        date: `${year}-${month}-${date}`,
        time: `${hour}:${minute}`,
    };
}

export function dateStringFomDate(date_obj) {}

export function clampNumber(n, min, max) {
    if (n < min) {
        return min;
    } else if (n > max) {
        return max;
    }
    return n;
}

export function getJSDate() {
    const demo_time_input = document.querySelector('#demo-time');
    if (demo_time_input && demo_time_input.value) {
        return new Date(demo_time_input.value);
    }
    return new Date();
}

export const DELETE_ICON_SVG =
    '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="red" viewBox="0 0 16 16"> <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z"/> <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z"/> </svg>';

export const BURST_ICON_SVG =
    '<svg width="16" height="16" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg"> <g stroke="#0F0000" stroke-width="4" stroke-linecap="round"> <line x1="50" y1="20" x2="50" y2="5" /> <line x1="50" y1="80" x2="50" y2="95" /> <line x1="20" y1="50" x2="5"  y2="50" /> <line x1="80" y1="50" x2="95" y2="50" /> <line x1="28.8" y1="28.8" x2="18.2" y2="18.2" /> <line x1="71.2" y1="71.2" x2="81.8" y2="81.8" /> <line x1="28.8" y1="71.2" x2="18.2" y2="81.8" /> <line x1="71.2" y1="28.8" x2="81.8" y2="18.2" /> </g> </svg>';
