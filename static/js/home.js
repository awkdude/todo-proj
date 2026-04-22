import {
    clampNumber,
    BURST_ICON_SVG,
    dateTimeFromJSDate,
    getJSDate,
} from '/js/util.js';

import { createTaskElement } from '/js/task.js';

console.log(`User Info: ${JSON.stringify(sessionStorage)}`);
if (!sessionStorage.id) {
    alert('No session for a user!');
    window.location.href = '/';
}

updateTaskList();

const welcome = document.querySelector('#welcome');
welcome.textContent = `Welcome, ${sessionStorage.fullname}`;
const clear_date = document.querySelector('#clear-date');
const clear_time = document.querySelector('#clear-time');
clear_date.innerHTML = clear_time.innerHTML = BURST_ICON_SVG;
const date_element = document.querySelector('#create-task-date');
const time_element = document.querySelector('#create-task-time');
let date_time = dateTimeFromJSDate(getJSDate());
console.log(`${JSON.stringify(date_time)}`);
date_element.min = date_time.date;
time_element.min = date_time.time;

clear_date.addEventListener('click', (event) => {
    const date_input = document.querySelector('#create-task-date');
    date_input.value = '';
});

clear_time.addEventListener('click', (event) => {
    const time_input = document.querySelector('#create-task-time');
    time_input.value = '';
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
    const now = getJSDate(); // new Date();

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
        const time = document.querySelector('#time');
        datetime_intro.textContent = `Today is ${DAY_NAMES[now.getDay()]}, `;
        date.textContent = formatted_date;
        time.textContent = formatted_time;
        // datetime.innerHTML = `<span id="datetime-intro">Today is</span> ${DAY_NAMES[now.getDay()]}, <span id="date">${formatted_date}</span> <span id="time">${formatted_time}</span>`;
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
    const task_list = await fetch(`/api/tasks/${sessionStorage.id}?date=134&time=men`).then(
        (response) => response.json()
    );
    console.log(`Tasks: ${JSON.stringify(task_list)}`);
    console.log(task_list);
    const task_view = document.querySelector('#task-elements');
    task_view.replaceChildren();

    const progress_label = document.querySelector('#progress-label');
    progress_label.textContent = "Today's Progress";

    for (const task of task_list) {
        createTaskElement(task);
    }
    // }}}
}
setInterval(() => {
    const progress_bar = document.querySelector('#progress');
    fetch(`/api/progress/${sessionStorage.id}`);
    const value = Math.sin((2.0 * Math.PI * window.performance.now()) / 1000);
    progress_bar.value = value * 50 + 50;
}, 500);

document
    .querySelector('#submit-creation')
    ?.addEventListener('click', async function (event) {
        // {{{
        const dialog = document.querySelector('#create-task-dialog');
        const title = dialog.querySelector('#create-task-title');
        const date_input = document.querySelector('#create-task-date');
        const time_input = document.querySelector('#create-task-time');
        console.log(
            `Date input: ${date_input.value}\nTime input: ${time_input.value}`
        );
        let task_data = {
            title: title.value,
            user_id: sessionStorage.id,
            date: date_input.value,
            time: time_input.value,
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

        if (true) {
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
});

const demo_time_input = document.querySelector('#demo-time');
demo_time_input.value = sessionStorage.demo_time ?? '';
demo_time_input.addEventListener('change', (event) => {
    console.log(`Date changed to ${demo_time_input.value}`);
    sessionStorage.demo_time = demo_time_input.value;
    updateTaskList();
});

// FIXME: Check if in demo mode
fetch('/api/demo')
    .then((response) => {
        sessionStorage.demo_mode = response.ok ? 'true' : 'false';
        console.log('Demo mode: ' + sessionStorage.demo_mode);
        if (sessionStorage.demo_mode === 'true') {
            console.log(sessionstorage.demo_mode);
            const demo_buttons = document.querySelector('#demo-input');
            demo_buttons.hidden = '';
        }
    })
    .catch((e) => {
        sessionStorage.demo_mode = false;
    });
