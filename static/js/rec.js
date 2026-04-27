const container_element = document.querySelector('main');

fetch(`/api/proto_tasks/${sessionStorage.id}`)
    .then((response) => response.json())
    .then((data) => {
        const DAY_NAMES = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
        for (const proto_task of data) {
            let proto_task_element = document.createElement('div');
            proto_task_element.classList.add('proto-task-item');
            proto_task_element.id = `p${proto_task.proto_id}`;
            let title_label = document.createElement('label');
            title_label.classList.add('rec-label');
            title_label.textContent = `${proto_task.title}`;
            let freq_label = document.createElement('label');
            freq_label.classList.add('rec-label');
            switch (proto_task.frequency_type) {
                case 0: {
                    freq_label.textContent = proto_task.due_date;
                    if (proto_task.end_date) {
                        freq_label.textContent += ` to ${proto_task.end_date}`;
                    }
                    break;
                }
                case 1: {
                    let inner_dates = proto_task.due_date;
                    if (proto_task.end_date) {
                        inner_dates += ` to ${proto_task.end_date}`;
                    }
                    freq_label.textContent = `Everyday (${inner_dates})`;

                    break;
                }
                case 2: {
                    for (const [i, day_name] of DAY_NAMES.entries()) {
                        if ((proto_task.day_bits & (1 << i)) != 0) {
                            freq_label.textContent += `${day_name},`;
                        }
                    }
                    break;
                }
            }
            let delete_button = document.createElement('button');
            delete_button.textContent = 'DELETE';
            delete_button.id = `d${proto_task.proto_id}`;
            delete_button.classList.add('delete-button2');
            delete_button.addEventListener('click', (event) => {
                event.preventDefault();
                fetch(`/api/proto_tasks/${proto_task.proto_id}`, {
                    method: 'DELETE',
                })
                    .then((response) => {
                        if (response.ok) {
                            console.log(
                                `Proto task ${proto_task.proto_id} deleted!`
                            );
                            const proto_task_element = document.querySelector(
                                `#p${proto_task.proto_id}`
                            );
                            proto_task_element.remove();
                        } else {
                            console.error(
                                `Error deleting Proto task ${proto_task.proto_id}`
                            );
                        }
                    })
                    .catch((err) => {
                        console.error(err.toString());
                    });
            });
            console.log(`${JSON.stringify(proto_task)}`);
            proto_task_element.append(title_label, freq_label, delete_button);
            if (proto_task.due_time) {
                const time_label = document.createElement('label');
                time_label.classList.add('rec-label');
                time_label.textContent = proto_task.due_time;
                proto_task_element.append(time_label);
            }
            container_element.append(proto_task_element);
        }
    });
