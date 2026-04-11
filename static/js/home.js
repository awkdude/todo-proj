let completion_sound = new Audio('/assets/Found Another Toad.mp3');
const welcome = document.querySelector('#welcome');
const username = 'Malcolm'; // TODO: get user's name
welcome.textContent = `Welcome, ${username}`;
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
    // createTaskItem();
}, 1000);
updateDateTime();
let task_item_count = 0;

function updateTaskList() {
    // TODO:
}

function createTaskItem() {
    let task_list = document.querySelector('#tasklist');
    if (task_list) {
        const newid = task_item_count++;
        let new_task_item = document.createElement('div');
        new_task_item.className = 'taskitem';
        new_task_item.id = `t${newid}`;
        let input_element = document.createElement('input');
        input_element.type = 'checkbox';
        input_element.id = `c${newid}`;
        let span_element = document.createElement('span');
        span_element.className = 'tasklabel';
        span_element.id = `l${newid}`;
        span_element.textContent = `Task ${newid}`;

        new_task_item.append(input_element, span_element);
        new_task_item.addEventListener('click', (event) => {
            if (event.target === event.currentTarget) {
                console.log(`Target: ${event.target}`);
                let checkbox = event.target.querySelector(
                    'input[type="checkbox"]'
                );
                checkbox.checked = !checkbox.checked;
                completion_sound.play();
            }
        });
        task_list.append(new_task_item);
    }
}
document.querySelector('#submit-creation')?.addEventListener('click', () => {
    // FIXME: Create real task!

    createTaskItem();
});
