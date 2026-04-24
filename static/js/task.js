import { DELETE_ICON_SVG, clampNumber } from '/js/util.js';

export function createTaskElement(task) {
    // {{{
    const newid = task.id;
    let new_task_item = document.createElement('div');
    new_task_item.classList.add('taskitem');
    // new_task_item.classList.add('disabled');
    new_task_item.id = `t${newid}`;
    let input_element = document.createElement('input');
    if (!task.is_range) {
        input_element.type = 'checkbox';
        input_element.id = `i${newid}`;
        input_element.checked = task.completion_value != 0;
        input_element.addEventListener('change', function (event) {
            event.preventDefault();
            let checkbox = event.target;
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
        });
    } else {
        input_element.type = 'range';
        input_element.id = `i${newid}`;
        input_element.max = task.range_max;
        input_element.value = clampNumber(
            task.completion_value,
            1,
            task.range_max
        );
        input_element.addEventListener('change', function (event) {
            let slider = event.target;
            event.preventDefault();
            const response = fetch(`/api/tasks/${task.id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    completion_value: Number.parseInt(slider.value),
                }),
            }).then((response) => {
                if (response.ok) {
                    console.log(`${task.title} updated!`);
                } else {
                    console.error(`${task.title} NOT updated!`);
                }
            });
        });
    }
    let label_element = document.createElement('label');
    label_element.className = 'tasklabel';
    label_element.id = `l${newid}`;
    label_element.textContent = task.title;
    label_element.htmlFor = `i${newid}`;
    let delete_element = document.createElement('button');
    delete_element.classList.add('delete-button');
    delete_element.id = `b${newid}`;
    console.log(Number.parseInt(delete_element.id.substring(1)));
    delete_element.innerHTML = DELETE_ICON_SVG;
    delete_element.addEventListener('click', function (event) {
        const response = fetch(`/api/tasks/${task.id}`, {
            method: 'DELETE',
        }).then((response) => {
            if (response.ok) {
                console.log(`${task.title} deleted!`);
                const task_element = document.querySelector(`#t${newid}`);
                task_element.remove();
            } else {
                console.error(`${task.title} NOT deleted!`);
            }
        });
        event.preventDefault();
    });
    // FIXME: Only works when clicking outside checkbox
    if (task.due_date !== null) {
        // TODO: add time
    }
    const tooltip = document.createElement('div');
    tooltip.innerHTML = `<span class="tooltiptext">${task.description}</span>`;
    tooltip.classList.add('tooltip');
    new_task_item.append(input_element, label_element, delete_element, tooltip);
    const task_view = document.querySelector('#task-elements');
    task_view.append(new_task_item);
    console.log(`${task.title} added`);
    // }}}
}
