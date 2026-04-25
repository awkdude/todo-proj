import { dateTimeFromJSDate, getJSDate } from '/js/util.js';

const month_select = document.querySelector('#month-select');
if (sessionStorage.setDate) {
    month_select.value = sessionStorage.setDate.substring(0, 7);
    console.log(`Loaded with: ${sessionStorage.setDate}`);
}
month_select.addEventListener('change', (event) => {
    if (month_select.value) {
        sessionStorage.setDate = `${month_select.value}-01`;
    } else {
        const date_time = dateTimeFromJSDate(getJSDate());
        sessionStorage.setDate = date_time.date;
    }
    window.location.reload();
    console.log(sessionStorage.setDate);
});

function renderCalendar() {
    let calendar_table = document.querySelector('#calender');
    const month_name_element = document.querySelector('#month-name');
    const MONTH_DAY = [
        ['January', 31],
        ['February', 28],
        ['March', 31],
        ['April', 30],
        ['May', 31],
        ['June', 30],
        ['July', 31],
        ['August', 31],
        ['September', 30],
        ['October', 31],
        ['November', 30],
        ['December', 31],
    ];

    const set_date_jsdate = new Date(`${sessionStorage.setDate}`);
    const month = set_date_jsdate.getUTCMonth();
    const year = set_date_jsdate.getFullYear();
    month_name_element.textContent = `${MONTH_DAY[month][0]} ${year}`;
    const first_date_of_month = new Date(
        `${year}-${(month + 1).toString().padStart(2, '0')}-01`
    );
    console.log(`First of month: ${first_date_of_month}`);
    let i = -first_date_of_month.getUTCDay() + 1;
    let calendar_body = document.querySelector('#calendar-body');
    const first_weekday_of_month = new Date('');
    for (let y = 0; y < 6; ++y) {
        let row = document.createElement('tr');
        for (let x = 0; x < 7; ++x) {
            let cell = document.createElement('td');
            cell.classList.add('day-cell');
            if (i > 0 && i <= MONTH_DAY[month][1]) {
                cell.textContent = i;
                cell.id = `d${i}`;
                cell.classList.add('day-cell-exist');
                const date_str = `${year}-${(month + 1).toString().padStart(2, '0')}-${cell.textContent.padStart(2, '0')}`;
                cell.addEventListener('click', (event) => {
                    console.log(date_str);
                    // TODO: set setDate and goto /home
                });
                fetch(`/api/progress/${sessionStorage.id}/${date_str}`)
                    .then((response) => response.json())
                    .then((data) => {
                        if (data.max_value <= 0) {
                            cell.style.backgroundColor = 'grey';
                        } else {
                            const progress = data.value / data.max_value;
                            if (progress < 0.33) {
                                cell.style.backgroundColor = '#ff0000';
                            } else if (progress < 0.75) {
                                cell.style.backgroundColor = '#aaaa00';
                            }
                        }
                    });
            } else {
                cell.style.visibility = 'hidden';
            }
            row.append(cell);
            ++i;
        }
        calendar_body.append(row);
    }
    // Set month progress bar
    const progress_label = document.querySelector('#progress-label');
    progress_label.textContent = "Month's progress";
    const date_str0 = `${year}-${(month + 1).toString().padStart(2, '0')}-00`;
    console.log(`0str: ${date_str0}`);
    fetch(`/api/progress/${sessionStorage.id}/${date_str0}`)
        .then((response) => response.json())
        .then((data) => {
            const value =
                data.max_value > 0 ? (data.value / data.max_value) * 100 : 0;
            const progress_bar = document.querySelector('#progress');
            progress_bar.value = value;
        });
}

renderCalendar();
