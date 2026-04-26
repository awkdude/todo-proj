import { CATEGORY_COLOR_MAP } from '/js/util.js';

let canvas = document.querySelector('#c');
let context = canvas.getContext('2d');

export function renderPie(obj, e_scale, _ff) {
    let cx = canvas.width / 2;
    let cy = canvas.height / 2;
    let radius_scale = 0.7;
    let r = Math.min(cx, cy) * radius_scale;

    const rads = (x) => (Math.PI * x) / 180;
    const degs = (x) => (180 * x) / Math.PI;
    let total = 0;
    for (const [name, value] of obj) {
        total += value;
    }
    let ff = _ff ?? 0;
    context.clearRect(0, 0, canvas.width, canvas.height);
    let start_angle = 0;
    context.beginPath();
    for (let ent of obj) {
        context.beginPath();
        context.fillStyle = CATEGORY_COLOR_MAP[ent[0]]; 
        let theta = 2 * Math.PI * (ent[1] / total) ;
        let end_angle = (start_angle + theta) ;
        context.moveTo(cx , cy );
        context.arc(cx , cy , r, start_angle , end_angle );
        context.closePath();
        context.fill();
        context.stroke();
        // context.fillStyle = 'black';
        // let labelX = cx + Math.cos(mid_angle) * (r * 0.7);
        // let labelY = cy + Math.sin(mid_angle) * (r * 0.7);
        // context.fillText(ent[0], labelX, labelY);
        start_angle = end_angle;
    }
}
