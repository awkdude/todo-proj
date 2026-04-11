const LIGHT_THEME_INDEX = 0;
const DARK_THEME_INDEX = 1;

let current_theme_index = 0;

const dark_mode_mql =
    window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)');

if (dark_mode_mql && dark_mode_mql.matches) {
    setTheme(1);
} else {
    setTheme(0);
}

dark_mode_mql?.addEventListener('change', (event) => {
    console.log('theme change!');
    setTheme(event.matches ? 1 : 0);
});

function setTheme(index) {
    const theme_paths = ['css/light.css', 'css/dark.css'];
    let new_theme = document.createElement('link');
    new_theme.id = 'theme';
    new_theme.rel = 'stylesheet';
    new_theme.href = theme_paths[index % theme_paths.length];
    let current_theme = document.querySelector('#theme');
    if (current_theme) {
        current_theme.replaceWith(new_theme);
    } else {
        document.head.append(new_theme);
    }
    current_theme_index = index;
}

document.body.addEventListener('keyup', function (event) {
    if (event.key === 'PageUp') {
        setTheme(current_theme_index + 1);
    }
});

let toast_element = document.createElement('div');
toast_element.id = 'toast';
// toast_element.classList.add('transparent');
toast_element.classList.add('fadeable');
toast_element.innerHTML = '<p>This is a toast!</p>';
document.body.prepend(toast_element);
setTimeout(() => {
    document.querySelector('#toast').classList.remove('transparent');
}, 2000);

// setTheme(0);
