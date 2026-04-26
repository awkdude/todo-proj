import {
    clampNumber,
    BURST_ICON_SVG,
    POWER_ICON_SVG,
    APP_NAME,
    dateTimeFromJSDate,
    getJSDate,
} from '/js/util.js';

import { createTaskElement } from '/js/task.js';

const logout = document.querySelector('#logout');
logout.innerHTML += POWER_ICON_SVG;
console.log(`User Info: ${JSON.stringify(sessionStorage)}`);
if (!sessionStorage.id) {
    window.location.href = '/';
}
const page_title = document.querySelector('title');
page_title.text = `${APP_NAME} | ${sessionStorage.username}`;
fetch(`/api/users/${sessionStorage.id}`).then((response) => {
    if (!response.ok) {
        // Goto home screen if user id doesn't exist
        window.location.href = '/';
    }
});

setTimeout(() => {
    // TODO:
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker
            .register('/js/service_worker.js')
            .then((registration) => {
                let serviceWorker;
                if (registration.installing) {
                    serviceWorker = registration.installing;
                    console.log('installing');
                } else if (registration.waiting) {
                    serviceWorker = registration.waiting;
                    console.log('waiting');
                } else if (registration.active) {
                    serviceWorker = registration.active;
                    console.log('active');
                }
                if (serviceWorker) {
                    // logState(serviceWorker.state);
                    serviceWorker.addEventListener('statechange', (e) => {
                        // logState(e.target.state);
                    });
                }
            })
            .catch((error) => {
                consle.log(`Error registering service worker: ${e.toString()}`);
            });
    }
}, 1000);

const demo_time_input = document.querySelector('#demo-time');
demo_time_input.value = sessionStorage.demo_time ?? '';
demo_time_input.addEventListener('change', (event) => {
    console.log(`Date changed to ${demo_time_input.value}`);
    sessionStorage.demo_time = demo_time_input.value;
    // updateTaskList();
    window.location.reload();
});

const welcome = document.querySelector('#welcome');
welcome.textContent = `Welcome, ${sessionStorage.fullname} (@${sessionStorage.username})`;
const clear_date = document.querySelector('#clear-date');
const clear_time = document.querySelector('#clear-time');
const clear_end_date = document.querySelector('#clear-end-date');
clear_date.innerHTML =
    clear_time.innerHTML =
    clear_end_date.innerHTML =
        BURST_ICON_SVG;
const date_element = document.querySelector('#create-task-date');
const time_element = document.querySelector('#create-task-time');
const end_date_element = document.querySelector('#create-task-end-date');
let date_time = dateTimeFromJSDate(getJSDate());
if (!sessionStorage.setDate) {
    sessionStorage.setDate = date_time.date;
}
const is_today = sessionStorage.setDate == date_time.date;

if (!is_today) {
    const today_button = document.querySelector('#today');
    today_button.disabled = false;
    today_button.addEventListener('click', (event) => {
        sessionStorage.setDate = '';
        window.location.reload();
    });
    const add_button = document.querySelector('#addtask');
    add_button.disabled = true;
}
console.log(`${JSON.stringify(date_time)}`);
date_element.min = end_date_element.min = date_time.date;
time_element.min = date_time.time;

clear_date.addEventListener('click', (event) => {
    const date_input = document.querySelector('#create-task-date');
    date_input.value = '';
});

clear_time.addEventListener('click', (event) => {
    const time_input = document.querySelector('#create-task-time');
    time_input.value = '';
});
clear_end_date.addEventListener('click', (event) => {
    const end_date_input = document.querySelector('#create-task-end-date');
    end_date_input.value = '';
});

// update time & date {{{
function updateDateTime() {
    let datetime = document.querySelector('#datetime');
    const DAY_NAMES = [
        'Sunday',
        'Monday',
        'Tuesday',
        'Wednesday',
        'Thursday',
        'Friday',
        'Saturday',
    ];
    // TODO: Use custom datetime format
    const now = is_today
        ? getJSDate()
        : new Date(`${sessionStorage.setDate} GMT-5:00`);

    // Simple, readable format
    const formatted_date = now.toLocaleString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
    });
    const formatted_time = now.toLocaleString('en-US', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
    });
    if (datetime) {
        const datetime_intro = document.querySelector('#datetime-intro');
        const date = document.querySelector('#date');
        date.textContent = formatted_date;
        if (is_today) {
            const time = document.querySelector('#time');
            datetime_intro.textContent = `Today is ${DAY_NAMES[now.getUTCDay()]}, `;
            time.textContent = formatted_time;
        } else {
            datetime_intro.textContent = `Day of ${DAY_NAMES[now.getUTCDay()]}, `;
        }
    }
}
updateDateTime();

let temp_value = 0;
const date_span = document.querySelector('#date');
date_span.addEventListener('wheel', (event) => {
    event.preventDefault();
    if (event.deltaY < 0) {
        --temp_value;
    } else {
        ++temp_value;
    }
    date_span.textContent = temp_value;
});
setInterval(() => {
    updateDateTime();
}, 500);
// }}}

let task_item_count = 0;

async function updateTaskList() {
    // {{{
    let date_time = dateTimeFromJSDate(getJSDate());
    let url = `/api/tasks/${sessionStorage.id}/${sessionStorage.setDate}`;
    console.log(url);
    const task_list = await fetch(url).then((response) => response.json());
    console.log(`Tasks: ${JSON.stringify(task_list)}`);
    console.log(task_list);
    const task_view = document.querySelector('#task-elements');
    task_view.replaceChildren();

    const progress_label = document.querySelector('#progress-label');
    progress_label.textContent = is_today
        ? "Today's Progress"
        : "Day's Progress";

    for (const task of task_list) {
        createTaskElement(task);
    }
    // }}}
}

function updateProgressBar() {
    const progress_bar = document.querySelector('#progress');
    let url = `/api/progress/${sessionStorage.id}/${sessionStorage.setDate}`;
    fetch(url)
        .then((response) => response.json())
        .then((data) => {
            const v =
                data.max_value > 0 ? (data.value / data.max_value) * 100 : 0;
            // console.log(v);
            progress_bar.value = v;
        });
}

setInterval(() => {
    updateProgressBar();
}, 250);

document
    .querySelector('#submit-creation')
    ?.addEventListener('click', async function (event) {
        // {{{
        let date_time = dateTimeFromJSDate(getJSDate());
        const dialog = document.querySelector('#create-task-dialog');
        const title = dialog.querySelector('#create-task-title');
        const date_input = document.querySelector('#create-task-date');
        const time_input = document.querySelector('#create-task-time');
        const end_date_input = document.querySelector('#create-task-end-date');
        const description_input = document.querySelector('#description');
        console.log(
            `Date input: ${date_input.value}\nTime input: ${time_input.value}\nEnd date input: ${end_date_input.value}`
        );
        let task_data = {
            title: title.value,
            user_id: sessionStorage.id,
            date: date_input.value || date_time.date,
            time: time_input.value,
            end_date: !end_date_input.disabled ? end_date_input.value : '',
            description: description.value.trim(),
        };
        task_data.frequency_type = 0;
        const frequency_select = document.querySelector('#frequency-select');
        if (frequency_select.value === 'daily-opt') {
            task_data.frequency_type = 1;
        } else if (frequency_select.value === 'weekly-opt') {
            task_data.frequency_type = 2;
        }
        const weekly_group = document.querySelector('#show-if-weekly');
        task_data.day_bits = 0;
        let day_bit_mask = 1;
        for (let day_box of weekly_group.children) {
            if (day_box.firstElementChild.checked) {
                task_data.day_bits |= day_bit_mask;
            }
            day_bit_mask <<= 1;
        }
        const is_range_input = document.querySelector('#create-task-is-range');
        task_data.is_range = is_range_input.checked;
        const range_input = document.querySelector('#create-task-range-max');
        task_data.range_min = 0;
        task_data.range_max = parseInt(range_input.value) ?? 2;
        if (isNaN(task_data.range_max)) {
            task_data.range_max = 2;
        }
        if (task_data.range_max < 2) {
            task_data.range_max = 2;
        }
        console.log(`Task data: ${JSON.stringify(task_data)}`);

        const response = await fetch(`/api/tasks/${sessionStorage.id}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(task_data),
        });
        if (response.ok) {
            console.log('Posted task!');
            updateTaskList();
        } else {
            console.error('Could not create task!');
        }
        // }}}
    });

const is_range_checkbox = document.querySelector('#create-task-is-range');
is_range_checkbox?.addEventListener('change', (event) => {
    const range_max_input = document.querySelector('#show-if-range');
    range_max_input.hidden = is_range_checkbox.checked ? '' : 'hidden';
});

const frequency_select = document.querySelector('#frequency-select');
frequency_select?.addEventListener('change', (event) => {
    const weekly_input = document.querySelector('#show-if-weekly');
    weekly_input.hidden =
        frequency_select.value === 'weekly-opt' ? '' : 'hidden';
    const end_date_input = document.querySelector('#create-task-end-date');
    end_date_input.disabled = frequency_select.value === 'none-opt';
});
updateTaskList();
