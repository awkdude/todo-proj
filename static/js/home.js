console.log(`User Info: ${JSON.stringify(sessionStorage)}`);
if (!sessionStorage.id) {
    alert('No session for a user!');
    window.location.href = '/';
}

updateTaskList();

const welcome = document.querySelector('#welcome');
welcome.textContent = `Welcome, ${sessionStorage.fullname}`;

const grid = document.querySelector('canvas#grid');
const context = grid.getContext('2d');

window.addEventListener('resize', function (event) {
    const size_element = document.querySelector('#size');
    size_element.textContent = `(${window.innerWidth}), (${window.innerHeight})`;
    const rect = document.documentElement.getBoundingClientRect();
    size_element.textContent = `(${rect.width}), (${rect.height})`;
});

// FIXME:
let offset = 0;
function render() {
    const total_minutes = 60 * 24;
    console.log(`alignment: ${context.textBaseline}`);
    const width = window.innerWidth;
    let minutes = 0;
    offset += 0.001;
    // context.reset();
    context.translate(0, offset);
    context.strokeStyle = 'white';
    for (let i = 0; minutes < total_minutes; ++i, minutes += 30) {
        const hour = String(Math.trunc(minutes / 60)).padStart(2, '0');
        const minute = String(minutes % 60).padStart(2, '0');
        context.beginPath();
        context.moveTo(0, 50 * i + 5);
        context.lineTo(width, 50 * i);
        context.stroke();
        context.fillText(`${hour}:${minute}`, 120, 50 * i);
    }
}

setInterval(render, (1 / 75.0));

function updateDateTime() {
    // {{{
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
    const now = new Date();

    // Simple, readable format
    const formatted = now.toLocaleString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
    });
    if (datetime) {
        datetime.textContent = `Today is ${DAY_NAMES[now.getDay()]}, ${formatted}`;
    }
    // }}}
}
setInterval(() => {
    updateDateTime();
}, 1000);
updateDateTime();

let task_item_count = 0;

async function updateTaskList() {
    const task_list = await fetch(`/api/tasks/${sessionStorage.id}`).then(
        (response) => response.json()
    );
    console.log(task_list);
    const task_view = document.querySelector('#task-elements');
    task_view.replaceChildren();

    for (const task of task_list) {
        const newid = task.id;
        let new_task_item = document.createElement('div');
        new_task_item.className = 'taskitem';
        new_task_item.id = `t${newid}`;
        let input_element = document.createElement('input');
        input_element.type = 'checkbox';
        input_element.id = `c${newid}`;
        input_element.checked = task.completion_value != 0;
        let span_element = document.createElement('span');
        span_element.className = 'tasklabel';
        span_element.id = `l${newid}`;
        span_element.textContent = task.title;
        let delete_element = document.createElement('button');
        delete_element.id = `b${newid}`;
        delete_element.textContent = 'DELETE';

        delete_element.addEventListener('click', function (event) {
            const response = fetch(`/api/tasks/${task.id}`, {
                method: 'DELETE',
            }).then((response) => {
                if (response.ok) {
                    console.log(`${task.title} deleted!`);
                } else {
                    console.error(`${task.title} NOT deleted!`);
                }
            });
            updateTaskList();
        });
        // FIXME: Only works when clicking outside checkbox
        new_task_item.addEventListener('click', function (event) {
            if (event.target === event.currentTarget) {
                console.log(`Target: ${event.target}`);
                let checkbox = event.target.querySelector(
                    'input[type="checkbox"]'
                );
                checkbox.checked = !checkbox.checked;
                console.log('checked!');
                const response = fetch(`/api/tasks/${task.id}`, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        completion_value: checkbox.checked ? 1 : 0,
                    }),
                }).then((response) => {
                    if (response.ok) {
                        console.log(`${task.title} updated!`);
                    } else {
                        console.error(`${task.title} NOT updated!`);
                    }
                });
                updateTaskList();
            }
        });
        new_task_item.append(input_element, span_element, delete_element);
        task_view.append(new_task_item);
    }
}

document
    .querySelector('#submit-creation')
    ?.addEventListener('click', async function (event) {
        const dialog = document.querySelector('#create-task-dialog');
        const title = dialog.querySelector('#create-task-title');
        console.log(`Created with title: ${title.value}`);
        const task_data = { title: title.value };
        const response = await fetch(`/api/tasks/${sessionStorage.id}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(task_data),
        });
        if (response.ok) {
            console.log('Posted task!');
            updateTaskList();
            // TODO: reload task and re-render task-view
        } else {
            console.error('Could not create task!');
        }
    });
